//! The properties of OneNote objects that we can parse.
//!
//! See [\[MS-ONE\] 2.1.12] for the list of properties the OneNote file format specifies.
//!
//! [\[MS-ONE\] 2.1.12]: https://docs.microsoft.com/en-us/openspecs/office_file_formats/ms-one/e9bf7da8-7aab-4668-be5e-e0c421175e3c

pub(crate) mod author;
pub(crate) mod charset;
pub(crate) mod color;
pub(crate) mod color_ref;
pub(crate) mod file_type;
pub(crate) mod ink_dimensions;
pub(crate) mod layout_alignment;
pub(crate) mod note_tag;
pub(crate) mod note_tag_property_status;
pub(crate) mod note_tag_shape;
pub(crate) mod object_reference;
pub(crate) mod object_space_reference;
pub(crate) mod outline_indent_distance;
pub(crate) mod page_size;
pub(crate) mod paragraph_alignment;
mod references;
pub(crate) mod simple;
pub(crate) mod time;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
#[allow(clippy::enum_clike_unportable_variant)]
pub(crate) enum PropertyType {
    ActionItemSchemaVersion = 0x0C003473,
    ActionItemStatus = 0x10003470,
    ActionItemType = 0x10003463,
    Author = 0x1C001D75,
    AuthorMostRecent = 0x20001D79,
    AuthorOriginal = 0x20001D78,
    BodyTextAlignment = 0x0C001C13,
    Bold = 0x08001C04,
    CachedTitleString = 0x1C001CF3,
    CachedTitleStringFromPage = 0x1C001D3C,
    CannotBeSelected = 0x08001CB2,
    Charset = 0x0C001D01,
    ChildGraphSpaceElementNodes = 0x2C001D63,
    ColumnCount = 0x14001D58,
    ConflictingUserName = 0x1C001D9E,
    ContentChildNodes = 0x24001C1F,
    CreationTimeStamp = 0x14001D09,
    Deletable = 0x08001D0C,
    DescendantsCannotBeMoved = 0x08001CF9,
    DisplayedPageNumber = 0x14003480,
    EditRootRtl = 0x08001C92,
    ElementChildNodes = 0x24001C20,
    EmbeddedFileContainer = 0x20001D9B,
    EmbeddedFileName = 0x1C001D9C,
    EnableHistory = 0x08001E1E,
    EnforceOutlineStructure = 0x08001C91,
    FileAncestorIdentityGuid = 0x1C001D95,
    FileIdentityGuid = 0x1C001D94,
    FileLastCodeVersionThatWroteToIt = 0x14001D99,
    FileNameCrc = 0x14001D93,
    Font = 0x1C001C0A,
    FontColor = 0x14001C0C,
    FontSize = 0x10001C0B,
    HasVersionPages = 0x08003462,
    Hidden = 0x08001E16,
    Highlight = 0x14001C0D,
    Hyperlink = 0x08001E14,
    HyperlinkProtected = 0x08001E19,
    IRecordMedia = 0x14001D24,
    ImageAltText = 0x1C001E58,
    ImageFilename = 0x1C001DD7,
    ImageUploadState = 0x140034CB,
    IsBackground = 0x08001D13,
    IsBoilerText = 0x08001C88,
    IsConflictObjectForRender = 0x08001D96,
    IsConflictObjectForSelection = 0x08001DDB,
    IsConflictPage = 0x08001D7C,
    IsDeletedGraphSpaceContent = 0x00001DE9,
    IsLayoutSizeSetByUser = 0x08001CBD,
    IsReadOnly = 0x08001CDE,
    IsTitleDate = 0x08001CB5,
    IsTitleText = 0x08001CB4,
    IsTitleTime = 0x08001C87,
    Italic = 0x08001C05,
    LanguageId = 0x14001C3B,
    LastModifiedTime = 0x14001D7A,
    LastModifiedTimeStamp = 0x18001D77,
    LayoutAlignmentInParent = 0x14001C3E,
    LayoutAlignmentSelf = 0x14001C84,
    LayoutCollisionPriority = 0x14001CF1,
    LayoutMaxHeight = 0x14001C1C,
    LayoutMaxWidth = 0x14001C1B,
    LayoutMinimumOutlineWidth = 0x14001CEC,
    LayoutOutlineReservedWidth = 0x14001CDB,
    LayoutResolveChildCollisions = 0x08001CDC,
    LayoutTightAlignment = 0x08001CFF,
    LayoutTightLayout = 0x08001C00,
    ListFont = 0x1C001C52,
    ListMsaaIndex = 0x10001D0E,
    ListNodes = 0x24001C26,
    ListRestart = 0x14001CB7,
    ListSpacingMu = 0x14001CCB,
    MathFormatting = 0x08003401,
    MetaDataObjectsAboveGraphSpace = 0x24003442,
    NextStyle = 0x1C00348A,
    NoteTagCompleted = 0x1400346F,
    NoteTagCreated = 0x1400346E,
    NoteTagDefinitionOid = 0x20003488,
    NoteTagHighlightColor = 0x14003465,
    NoteTagLabel = 0x1C003468,
    NoteTagPropertyStatus = 0x14003467,
    NoteTagShape = 0x10003464,
    NoteTagStates = 0x04003489,
    NoteTagTextColor = 0x14003466,
    NotebookManagementEntityGuid = 0x1C001C30,
    NumberListFormat = 0x1C001C1A,
    OffsetFromParentHoriz = 0x14001C14,
    OffsetFromParentVert = 0x14001C15,
    OutlineElementChildLevel = 0x0C001C03,
    OutlineElementRtl = 0x08001C34,
    PageHeight = 0x14001C02,
    PageLevel = 0x14001DFF,
    PageMarginBottom = 0x14001C4D,
    PageMarginLeft = 0x14001C4E,
    PageMarginOriginX = 0x14001D0F,
    PageMarginOriginY = 0x14001D10,
    PageMarginRight = 0x14001C4F,
    PageMarginTop = 0x14001C4C,
    PageSize = 0x14001C8B,
    PageWidth = 0x14001C01,
    ParagraphAlignment = 0x0C003477,
    ParagraphLineSpacingExact = 0x14003430,
    ParagraphSpaceAfter = 0x1400342F,
    ParagraphSpaceBefore = 0x1400342E,
    ParagraphStyle = 0x2000342C,
    ParagraphStyleId = 0x1C00345A,
    PictureContainer = 0x20001C3F,
    PictureHeight = 0x140034CE,
    PictureWidth = 0x140034CD,
    PortraitPage = 0x08001C8E,
    ReadingOrderRtl = 0x08003476,
    RgOutlineIndentDistance = 0x1C001C12,
    RichEditTextLangId = 0x10001CFE,
    RichEditTextUnicode = 0x1C001C22,
    RowCount = 0x14001D57,
    SchemaRevisionInOrderToRead = 0x14001D82,
    SchemaRevisionInOrderToWrite = 0x1400348B,
    SectionDisplayName = 0x1C00349B,
    SourceFilepath = 0x1C001D9D,
    Strikethrough = 0x08001C07,
    StructureElementChildNodes = 0x24001D5F,
    Subscript = 0x08001C09,
    Superscript = 0x08001C08,
    TableBordersVisible = 0x08001D5E,
    TableColumnWidths = 0x1C001D66,
    TableColumnsLocked = 0x1C001D7D,
    TaskTagDueDat = 0x1400346B,
    TextExtendedAscii = 0x1C003498,
    TextRunData = 0x40003499,
    TextRunDataObject = 0x24003458,
    TextRunFormatting = 0x24001E13,
    TextRunIndex = 0x1C001E12,
    TextRunIsEmbeddedObject = 0x08001E22,
    TopologyCreationTimeStamp = 0x18001C65,
    Underline = 0x08001C06,
    UnderlineType = 0x0C001E15,
    VersionHistoryGraphSpaceContextNodes = 0x3400347B,
    WebPictureContainer14 = 0x200034C8,
    WzHyperlinkUrl = 0x1C001E20,

    // Undocumented:
    TocChildren = 0x24001CF6,
    FolderChildFilename = 0x1C001D6B,
    NotebookElementOrderingId = 0x14001CB9,
    PictureFileExtension = 0x24003424,
    NoteTags = 0x40003489,
    NoteTag = 0x44000811,
    SectionColor = 0x14001CBE,
    CellBackgroundColor = 0x14001E26,
    InkBias = 0x0C00341C,
    InkData = 0x20003415,
    InkDimensions = 0x1C00340A,
    InkPath = 0x1C00340B,
    InkStrokeProperties = 0x20003409,
    InkStrokes = 0x24003416,
    InkAntialised = 0x8000340E,
    InkFitToCurve = 0x80003410,
    InkIgnorePressure = 0x08003411,
    InkPenTip = 0x0C003412,
    InkRasterOperation = 0x0C003413,
    InkTransparency = 0x0C003414,
    InkHeight = 0x1400340C,
    InkWidth = 0x1400340D,
    InkColor = 0x1400340F,
    InkScalingX = 0x14001C46,
    InkScalingY = 0x14001C47,
    InkBoundingBox = 0x1C003418,
    EmbeddedObjectType = 0x14003457,
    EmbeddedInkStartX = 0x1400349E,
    EmbeddedInkStartY = 0x1400349F,
    EmbeddedInkWidth = 0x140034A0,
    EmbeddedInkHeight = 0x140034A1,
    EmbeddedInkOffsetHoriz = 0x140034A2,
    EmbeddedInkOffsetVert = 0x140034A3,
    EmbeddedInkSpaceWidth = 0x14001C27,
    EmbeddedInkSpaceHeight = 0x14001C28,
    ImageEmbedType = 0x140035F2,
    ImageEmbeddedUrl = 0x1C0035F3,
}
