use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CgnsNodeLabel {
    Zone,
    Base,
    SimulationType,
    Descriptor,
    Ordinal,
    Custom(String),
}

impl CgnsNodeLabel {
    pub fn as_str<'a>(&'a self) -> &'a str {
        use CgnsNodeLabel::*;
        match self {
            Ordinal => "Ordinal_t",
            Zone => "Zone_t",
            Base => "CGNSBase_t",
            SimulationType => "SimulationType_t",
            Descriptor => "Descriptor_t",
            Custom(inner) => &inner,
        }
    }
}

pub type CgnsPathNodes = Vec<(CgnsNodeLabel, i32)>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgnsPath {
    pub file_number: i32,
    pub base_index: i32,
    pub nodes: CgnsPathNodes,
}

impl CgnsPath {
    pub fn parent(&self) -> Option<CgnsPath> {
        if self.nodes.is_empty() {
            None
        } else {
            let mut parent = self.clone();
            parent.nodes.pop();

            Some(parent)
        }
    }
}
// TODO: impl Display

#[repr(u32)]
pub enum CgnsOpenMode {
    // Closed = cgns_bindings::CG_MODE_CLOSED,
    Modify = cgns_bindings::CG_MODE_MODIFY,
    Read = cgns_bindings::CG_MODE_READ,
    Write = cgns_bindings::CG_MODE_WRITE,
}

#[repr(u32)]
pub enum CgnsFileType {
    ADF = cgns_bindings::CG_FILE_ADF,
    ADF2 = cgns_bindings::CG_FILE_ADF2,
    HDF5 = cgns_bindings::CG_FILE_HDF5,
    NONE = cgns_bindings::CG_FILE_NONE,
}
