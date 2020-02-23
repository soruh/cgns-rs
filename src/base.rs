use super::*;

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::c_char;

pub struct Bases<'b> {
    pub(crate) current: i32,
    pub(crate) max: i32,
    pub(crate) file: &'b File<'b>,
}

impl<'b> Iterator for Bases<'b> {
    type Item = Base<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.max {
            let base = Base::new(self.file, self.current);

            self.current += 1;

            Some(base)
        } else {
            None
        }
    }
}

pub struct Base<'b> {
    pub(crate) base_index: i32,
    pub(crate) file: &'b File<'b>,
}

impl<'b> Base<'b> {
    /// Get the cell dimension for the CGNS base
    pub fn dim(&self) -> CgnsResult<i32> {
        let mut cell_dim = 0;

        to_cgns_result!(unsafe {
            bindings::cg_cell_dim(self.file.file_number, self.base_index, &mut cell_dim)
        })?;

        Ok(cell_dim)
    }

    pub fn n_zones(&self) -> CgnsResult<i32> {
        let mut nzones = 0;

        to_cgns_result!(unsafe {
            bindings::cg_nzones(self.file.file_number, self.base_index, &mut nzones)
        })?;

        Ok(nzones)
    }

    #[inline]
    pub fn get_zone<'z>(&'z self, zone_index: i32) -> Zone<'z> {
        Zone::new(self, zone_index)
    }

    pub fn zones() {
        todo!()
    }
}

impl<'b> Node<'b> for Base<'b> {
    type Item = BaseData;
    type Parent = File<'b>;
    fn path(&self) -> CgnsPath {
        CgnsPath {
            file_number: self.file.file_number,
            base_index: self.base_index,
            nodes: vec![],
        }
    }

    /// Read CGNS base information
    fn read(&self) -> CgnsResult<BaseData> {
        let mut cell_dim = 0;
        let mut phys_dim = 0;
        let mut basename: [MaybeUninit<c_char>; 33] = [MaybeUninit::uninit(); 33];

        to_cgns_result!(unsafe {
            bindings::cg_base_read(
                self.file.file_number,
                self.base_index,
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

        to_cgns_result!(unsafe {
            bindings::cg_base_write(
                parent.file_number,
                basename.as_ptr(),
                data.cell_dim,
                data.phys_dim,
                &mut base_index,
            )
        })?;

        Ok(base_index)
    }

    fn new(parent: &'b Self::Parent, base_index: i32) -> Base<'b> {
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
