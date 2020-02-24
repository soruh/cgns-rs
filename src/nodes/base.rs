use super::*;

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

pub struct Base<'b> {
    base_index: i32,
    file: &'b File<'b>,
}

impl<'b> Base<'b> {
    pub fn file<'f>(&'f self) -> &'f File {
        self.file
    }

    pub fn lib<'l>(&'l self) -> &'l Library {
        self.file().lib
    }

    /// Get the cell dimension for the CGNS base
    pub fn dim(&self) -> CgnsResult<i32> {
        let mut cell_dim = 0;

        to_cgns_result(unsafe {
            bindings::cg_cell_dim(self.file().file_number(), self.index(), &mut cell_dim)
        })?;

        Ok(cell_dim)
    }

    pub fn n_zones(&self) -> CgnsResult<i32> {
        let mut nzones = 0;

        to_cgns_result(unsafe {
            bindings::cg_nzones(self.file().file_number(), self.index(), &mut nzones)
        })?;

        Ok(nzones)
    }

    #[inline]
    pub fn get_zone<'z>(&'z self, zone_index: i32) -> CgnsResult<Zone<'z>> {
        Zone::new(self, zone_index)
    }

    pub fn zones() {
        todo!()
    }
}

impl<'b> Node for Base<'b> {}

impl<'b> GotoTarget for Base<'b> {
    const NODE_LABEL: CgnsNodeLabel = CgnsNodeLabel::Base;
    fn name(&self) -> CgnsResult<String> {
        Ok(String::from(&self.read()?.name))
    }
    fn path(&self) -> CgnsPath {
        CgnsPath {
            file_number: self.file().file_number(),
            base_index: self.index(),
            nodes: vec![],
        }
    }
}

impl<'b> BaseRefNode for Base<'b> {
    #[inline]
    fn base<'b_>(&'b_ self) -> &'b_ Base {
        self
    }
}

impl<'b> RwNode<'b> for Base<'b> {
    type Item = BaseData;

    /// Read CGNS base information
    fn read(&self) -> CgnsResult<BaseData> {
        let mut cell_dim = 0;
        let mut phys_dim = 0;
        let mut basename: [MaybeUninit<c_char>; 33] = [MaybeUninit::uninit(); 33];

        to_cgns_result(unsafe {
            bindings::cg_base_read(
                self.file().file_number(),
                self.index(),
                basename.as_mut_ptr() as *mut c_char,
                &mut cell_dim,
                &mut phys_dim,
            )
        })?;

        let name: String = unsafe { CStr::from_ptr(basename.as_ptr() as *const i8) }
            .to_str()?
            .to_string();

        Ok(BaseData {
            name,
            cell_dim,
            phys_dim,
        })
    }

    /// Create and/or write to a CGNS base node
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        // file_number: c_int,
        let basename = CString::new(data.name.clone())?;
        let mut base_index = 0;

        to_cgns_result(unsafe {
            bindings::cg_base_write(
                parent.file_number(),
                basename.as_ptr(),
                data.cell_dim,
                data.phys_dim,
                &mut base_index,
            )
        })?;

        Ok(base_index)
    }
}

impl<'b> ParentNode<'b, Zone<'b>> for Base<'b> {
    fn n_children(&self) -> CgnsResult<i32> {
        self.n_zones()
    }
}

impl<'b> ParentNode<'b, SimulationType<'b>> for Base<'b> {
    fn n_children(&self) -> CgnsResult<i32> {
        Ok(1) // TODO
    }
}

impl<'b> ChildNode<'b> for Base<'b> {
    type Parent = File<'b>;

    #[inline]
    fn parent(&self) -> &Self::Parent {
        self.file
    }
}

impl<'b> IndexableNode for Base<'b> {
    #[inline]
    fn index(&self) -> i32 {
        self.base_index
    }
}

impl<'b> SiblingNode<'b> for Base<'b> {
    #[inline]
    fn new_unchecked(parent: &'b Self::Parent, base_index: i32) -> Base<'b> {
        Base {
            file: parent,
            base_index,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BaseData {
    pub name: String,
    pub cell_dim: i32,
    pub phys_dim: i32,
}
