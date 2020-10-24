use crate::errors::{ErrorKind, Result};
use crate::fsshttpb::data::exguid::ExGuid;
use crate::one::property::object_reference::ObjectReference;
use crate::one::property::time::Timestamp;
use crate::one::property::{simple, PropertyType};
use crate::one::property_set::PropertySetId;
use crate::onestore::object::Object;
use crate::shared::guid::Guid;

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) context_id: ExGuid,
    pub(crate) entity_guid: Guid,
    pub(crate) page_series: Vec<ExGuid>,
    pub(crate) created_at: Timestamp,
}

pub(crate) fn parse(object: &Object) -> Result<Data> {
    if object.id() != PropertySetId::SectionNode.as_jcid() {
        return Err(ErrorKind::MalformedOneNoteFileData(
            format!("unexpected object type: 0x{:X}", object.id().0).into(),
        )
        .into());
    }

    let context_id = object.context_id();

    let entity_guid = simple::parse_guid(PropertyType::NotebookManagementEntityGuid, object)?
        .ok_or_else(|| ErrorKind::MalformedOneNoteFileData("section has no guid".into()))?;
    let page_series =
        ObjectReference::parse_vec(PropertyType::ElementChildNodes, object)?.unwrap_or_default();
    let created_at = Timestamp::parse(PropertyType::TopologyCreationTimeStamp, object)?
        .ok_or_else(|| {
            ErrorKind::MalformedOneNoteFileData("section has no creation timestamp".into())
        })?;

    let data = Data {
        context_id,
        entity_guid,
        page_series,
        created_at,
    };

    Ok(data)
}
