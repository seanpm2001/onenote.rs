use crate::errors::{ErrorKind, Result};
use crate::fsshttpb::data::exguid::ExGuid;
use crate::one::property::object_reference::ObjectReference;
use crate::one::property::{simple, PropertyType};
use crate::one::property_set::PropertySetId;
use crate::onestore::object::Object;

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) children: Vec<ExGuid>,
    pub(crate) filename: Option<String>,
    pub(crate) ordering_id: Option<u32>,
    // FIXME: Color!?
}

pub(crate) fn parse(object: &Object) -> Result<Data> {
    if object.id() != PropertySetId::TocContainer.as_jcid() {
        return Err(ErrorKind::MalformedOneNoteFileData(
            format!("unexpected object type: 0x{:X}", object.id().0).into(),
        )
        .into());
    }

    let children =
        ObjectReference::parse_vec(PropertyType::TocChildren, object)?.unwrap_or_default();
    let filename = simple::parse_string(PropertyType::FolderChildFilename, object)?
        .map(|s| s.replace("^M", "+"))
        .map(|s| s.replace("^J", ","));
    let ordering_id = simple::parse_u32(PropertyType::NotebookElementOrderingId, object)?;

    Ok(Data {
        children,
        filename,
        ordering_id,
    })
}
