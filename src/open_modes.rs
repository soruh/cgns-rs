pub trait OpenMode {}
pub trait OpenModeRead: OpenMode {}
pub trait OpenModeWrite: OpenMode {}

pub struct ReadableFile();
impl OpenMode for ReadableFile {}
impl OpenModeRead for ReadableFile {}

pub struct WriteableFile();
impl OpenMode for WriteableFile {}
impl OpenModeWrite for WriteableFile {}

pub struct ModifiableFile();
impl OpenMode for ModifiableFile {}
impl OpenModeWrite for ModifiableFile {}
impl OpenModeRead for ModifiableFile {}

// NOTE: has the same permissions as `ModifiableFile` but signifies that
// The file mode is not modify but has been dynamically selected
pub struct UnknownFile {}
impl OpenMode for UnknownFile {}
impl OpenModeWrite for UnknownFile {}
impl OpenModeRead for UnknownFile {}
