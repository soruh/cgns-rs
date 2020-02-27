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

impl std::fmt::Display for CgnsNodeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CgnsNodeLabel::*;
        let res = match self {
            Ordinal => "Ordinal_t",
            Zone => "Zone_t",
            Base => "CGNSBase_t",
            SimulationType => "SimulationType_t",
            Descriptor => "Descriptor_t",
            Custom(inner) => &inner,
        };
        write!(f, "{}", res)
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

impl std::fmt::Display for CgnsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node_path = String::new();

        for (label, index) in &self.nodes {
            node_path.push_str(&format!("{}/{}/", label, index));
        }

        write!(
            f,
            "File_t/{}/Base_t/{}/{}",
            self.file_number, self.base_index, node_path
        )
    }
}

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
