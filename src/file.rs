use super::*;
use std::ffi::CString;
pub struct File<'f> {
    file_number: i32,
    pub(crate) lib: &'f Library,
}

impl<'f> std::fmt::Debug for File<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "File[{}]", self.file_number)
    }
}

impl<'f> File<'f> {
    fn close_by_ref(&mut self) -> CgnsResult<()> {
        to_cgns_result!(unsafe { bindings::cg_close(self.file_number) })
    }

    pub(crate) fn open<'l>(
        lib: &'l Library,
        filename: &str,
        mode: CgnsOpenMode,
    ) -> CgnsResult<File<'l>> {
        let filename = CString::new(filename)?;
        let mut file_number = 0;

        to_cgns_result!(unsafe {
            bindings::cg_open(filename.as_ptr(), mode as i32, &mut file_number)
        })?;

        Ok(File { file_number, lib })
    }

    // make sure this `File` isn't used after we close it
    pub fn close(mut self) -> CgnsResult<()> {
        self.close_by_ref()?;

        std::mem::forget(self); // we don't want to call `close` twice...

        Ok(())
    }

    /// exposes the cgns internal file_number (`fn`) of this file
    pub fn file_number(&self) -> i32 {
        self.file_number
    }

    /// exposes the cgns internal cgio_number (`cgio_num`) of this file
    pub fn get_cgio_number(&self) -> CgnsResult<i32> {
        let mut cgio_number = 0;

        to_cgns_result!(unsafe { bindings::cg_get_cgio(self.file_number, &mut cgio_number) })?;

        Ok(cgio_number)
    }

    /// exposes the cgns internal root_id (`root_id`) of this file
    pub fn root_id(&self) -> CgnsResult<f64> {
        let mut root_id = 0.0;

        to_cgns_result!(unsafe { bindings::cg_root_id(self.file_number, &mut root_id) })?;

        Ok(root_id)
    }

    #[inline]
    pub fn cgio<'s>(&'s self) -> CgnsResult<Cgio<'s>> {
        Cgio::from_file(self)
    }

    pub fn save_as(
        &mut self,
        filename: &str,
        file_type: CgnsFileType,
        follow_links: bool,
    ) -> CgnsResult<()> {
        let filename = CString::new(filename)?;

        to_cgns_result!(unsafe {
            bindings::cg_save_as(
                self.file_number,
                filename.as_ptr(),
                file_type as i32,
                follow_links as i32,
            )
        })
    }

    /// Access a base in a CGNS file
    /// Do not use this in a loop. Instead call `.bases()` to get an iterator over all bases
    /// NOTE: the `base_index` is not checked for validity
    // TODO: check base_index for validity
    #[inline]
    pub fn get_base<'b>(&'b self, base_index: i32) -> CgnsResult<Base<'b>> {
        Base::new(self, base_index)
    }

    /// Get number of CGNS base nodes in file
    pub fn n_bases(&self) -> CgnsResult<i32> {
        let mut nbases = 0;

        to_cgns_result!(unsafe { bindings::cg_nbases(self.file_number, &mut nbases) })?;

        Ok(nbases)
    }

    // TODO: abstract into trait
    pub fn bases<'b>(&'b self) -> CgnsResult<Bases<'b>> {
        Ok(Bases {
            current: 1,
            max: self.n_bases()?,
            file: self,
        })
    }
}

impl<'f> Drop for File<'f> {
    fn drop(&mut self) {
        self.close_by_ref()
            .expect(&format!("Failed to close {:?}", self))
    }
}

impl<'f> Node for File<'f> {}

pub enum FileChildren {
    Base,
}

impl<'f> ParentNode<'f, Base<'f>> for File<'f> {
    fn n_children(&self) -> CgnsResult<i32> {
        self.n_bases()
    }
}
