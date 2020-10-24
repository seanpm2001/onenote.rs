use crate::errors::{ErrorKind, Result};
use crate::fsshttpb::data::exguid::ExGuid;
use crate::one::property::note_tag::ActionItemStatus;
use crate::one::property::object_reference::ObjectReference;
use crate::one::property::object_space_reference::ObjectSpaceReference;
use crate::one::property::time::Time;
use crate::one::property::PropertyType;
use crate::onestore::object::Object;
use crate::onestore::types::compact_id::CompactId;
use crate::onestore::types::jcid::JcId;
use crate::onestore::types::object_prop_set::ObjectPropSet;
use crate::onestore::types::prop_set::PropertySet;

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) definition: Option<ExGuid>,
    pub(crate) created_at: Time,
    pub(crate) completed_at: Option<Time>,
    pub(crate) item_status: ActionItemStatus,
}

impl Data {
    pub(crate) fn parse(object: &Object) -> Result<Option<Vec<Data>>> {
        object
            .props()
            .get(PropertyType::NoteTags)
            .map(|value| {
                value.to_property_values().ok_or_else(|| {
                    ErrorKind::MalformedOneNoteFileData(
                        "note tag state is not a property values list".into(),
                    )
                })
            })
            .transpose()?
            .map(|(id, sets)| {
                Ok(sets
                    .iter()
                    .map(|props| {
                        Ok(Object {
                            context_id: object.context_id,
                            jc_id: JcId(id.value()),
                            props: ObjectPropSet {
                                object_ids: Self::get_object_ids(props, object)?,
                                object_space_ids: Self::get_object_space_ids(props, object)?,
                                context_ids: vec![],
                                properties: props.clone(),
                            },
                            file_data: None,
                            mapping: object.mapping.clone(),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?
                    .iter()
                    .map(|object| {
                        let definition =
                            ObjectReference::parse(PropertyType::NoteTagDefinitionOid, &object)?;

                        let created_at = Time::parse(PropertyType::NoteTagCreated, &object)?
                            .ok_or_else(|| {
                                ErrorKind::MalformedOneNoteFileData(
                                    "note tag has no created at time".into(),
                                )
                            })?;

                        let completed_at = Time::parse(PropertyType::NoteTagCompleted, &object)?;

                        let item_status = ActionItemStatus::parse(&object)?.ok_or_else(|| {
                            ErrorKind::MalformedOneNoteFileData(
                                "note tag container has no item status".into(),
                            )
                        })?;

                        Ok(Data {
                            definition,
                            created_at,
                            completed_at,
                            item_status,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?)
            })
            .transpose()
    }

    fn get_object_ids(props: &PropertySet, object: &Object) -> Result<Vec<CompactId>> {
        Ok(object
            .props
            .object_ids
            .iter()
            .skip(ObjectReference::get_offset(PropertyType::NoteTags, object)?)
            .take(ObjectReference::count_references(props.values()))
            .copied()
            .collect())
    }

    fn get_object_space_ids(props: &PropertySet, object: &Object) -> Result<Vec<CompactId>> {
        Ok(object
            .props
            .object_ids
            .iter()
            .skip(ObjectSpaceReference::get_offset(
                PropertyType::NoteTags,
                object,
            )?)
            .take(ObjectSpaceReference::count_references(props.values()))
            .copied()
            .collect())
    }
}
