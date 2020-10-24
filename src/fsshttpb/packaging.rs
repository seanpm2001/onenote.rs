use crate::errors::{ErrorKind, Result};
use crate::fsshttpb::data::exguid::ExGuid;
use crate::fsshttpb::data::object_types::ObjectType;
use crate::fsshttpb::data::stream_object::ObjectHeader;
use crate::fsshttpb::data_element::DataElementPackage;
use crate::shared::guid::Guid;
use crate::Reader;

#[derive(Debug)]
pub(crate) struct Packaging {
    pub(crate) file_type: Guid,
    pub(crate) file: Guid,
    pub(crate) legacy_file_version: Guid,
    pub(crate) file_format: Guid,
    pub(crate) storage_index: ExGuid,
    pub(crate) cell_schema: Guid,
    pub(crate) data_element_package: DataElementPackage,
}

impl Packaging {
    pub(crate) fn parse(reader: Reader) -> Result<Packaging> {
        let file_type = Guid::parse(reader)?;
        let file = Guid::parse(reader)?;
        let legacy_file_version = Guid::parse(reader)?;
        let file_format = Guid::parse(reader)?;

        if file != legacy_file_version {
            return Err(
                ErrorKind::MalformedOneStoreData("not a legacy OneStore file".into()).into(),
            );
        }

        if reader.get_u32()? != 0 {
            return Err(ErrorKind::MalformedFssHttpBData("invalid padding data".into()).into());
        }

        ObjectHeader::try_parse_32(reader, ObjectType::OneNotePackaging)?;

        let storage_index = ExGuid::parse(reader)?;
        let cell_schema = Guid::parse(reader)?;

        let data_element_package = DataElementPackage::parse(reader)?;

        ObjectHeader::try_parse_end_16(reader, ObjectType::OneNotePackaging)?;

        Ok(Packaging {
            file_type,
            file,
            legacy_file_version,
            file_format,
            storage_index,
            cell_schema,
            data_element_package,
        })
    }
}
