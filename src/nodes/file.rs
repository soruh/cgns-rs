use super::*;
use std::ffi::CString;
use std::marker::PhantomData;

pub struct File<'f, M: OpenMode> {
    file_number: i32,
    pub(crate) lib: &'f Library,
    _phantom: PhantomData<*const M>,
}
impl<'f, M: OpenMode> std::fmt::Debug for File<'f, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "File[{}]", self.file_number)
    }
}
impl<'f, M: OpenMode> File<'f, M> {
    #[inline]
    pub fn get_base<'b>(&'b self, base_index: i32) -> CgnsResult<Base<'b, M>>
    where
        M: OpenModeRead,
    {
        Base::new(self, base_index)
    }

    #[inline]
    pub fn bases<'b>(&'b self) -> CgnsResult<NodeIter<M, Base<'b, M>>>
    where
        M: OpenModeRead,
    {
        Base::iter(self)
    }

    #[inline]
    pub fn n_bases<'b>(&'b self) -> CgnsResult<i32>
    where
        M: OpenModeRead,
        Self: ParentNode<'b, M, Base<'b, M>>,
    {
        self.n_children()
    }

    fn close_by_ref(&mut self) -> CgnsResult<()> {
        to_cgns_result(unsafe { cgns_bindings::cg_close(self.file_number) })
    }

    // make sure this `File` isn't used after we close it
    pub fn close(mut self) -> CgnsResult<()> {
        self.close_by_ref()?;
        std::mem::forget(self); // we don't want to call `close` twice...
        Ok(())
    }

    /// exposes the cgns_bindings internal file_number (`fn`) of this file
    pub fn file_number(&self) -> i32 {
        self.file_number
    }

    pub(crate) fn open_raw(filename: &str, mode: CgnsOpenMode) -> CgnsResult<i32> {
        let filename = CString::new(filename)?;
        let mut file_number = 0;

        to_cgns_result(unsafe {
            cgns_bindings::cg_open(filename.as_ptr(), mode as i32, &mut file_number)
        })?;

        Ok(file_number)
    }

    pub(crate) fn open_dynamic<'l>(
        lib: &'l Library,
        filename: &str,
        mode: CgnsOpenMode,
    ) -> CgnsResult<File<'l, UnknownFile>> {
        Ok(File {
            file_number: Self::open_raw(filename, mode)?,
            lib,
            _phantom: Default::default(),
        })
    }

    pub(crate) fn open_read<'l>(
        lib: &'l Library,
        filename: &str,
    ) -> CgnsResult<File<'l, ReadableFile>> {
        Ok(File {
            file_number: Self::open_raw(filename, CgnsOpenMode::Read)?,
            lib,
            _phantom: Default::default(),
        })
    }

    pub(crate) fn open_write<'l>(
        lib: &'l Library,
        filename: &str,
    ) -> CgnsResult<File<'l, WriteableFile>> {
        Ok(File {
            file_number: Self::open_raw(filename, CgnsOpenMode::Write)?,
            lib,
            _phantom: Default::default(),
        })
    }

    pub(crate) fn open_modify<'l>(
        lib: &'l Library,
        filename: &str,
    ) -> CgnsResult<File<'l, ModifiableFile>> {
        Ok(File {
            file_number: Self::open_raw(filename, CgnsOpenMode::Modify)?,
            lib,
            _phantom: Default::default(),
        })
    }

    /// exposes the cgns_bindings internal cgio_number (`cgio_num`) of this file
    pub fn get_cgio_number(&self) -> CgnsResult<i32> {
        let mut cgio_number = 0;

        to_cgns_result(unsafe { cgns_bindings::cg_get_cgio(self.file_number, &mut cgio_number) })?;

        Ok(cgio_number)
    }

    /// exposes the cgns_bindings internal root_id (`root_id`) of this file
    pub fn root_id(&self) -> CgnsResult<f64> {
        let mut root_id = 0.0;

        to_cgns_result(unsafe { cgns_bindings::cg_root_id(self.file_number, &mut root_id) })?;

        Ok(root_id)
    }

    #[inline]
    pub fn cgio<'s>(&'s self) -> CgnsResult<Cgio<'s, M>> {
        Cgio::from_file(self)
    }

    pub fn save_as(
        &mut self,
        filename: &str,
        file_type: CgnsFileType,
        follow_links: bool,
    ) -> CgnsResult<()> {
        let filename = CString::new(filename)?;

        to_cgns_result(unsafe {
            cgns_bindings::cg_save_as(
                self.file_number,
                filename.as_ptr(),
                file_type as i32,
                follow_links as i32,
            )
        })
    }
}
impl<'f, M: OpenMode> Drop for File<'f, M> {
    fn drop(&mut self) {
        self.close_by_ref()
            .expect(&format!("Failed to close {:?}", self))
    }
}
impl<'f, M: OpenMode> Node for File<'f, M> {}
impl<'f, M: OpenMode> IndexableNode for File<'f, M> {
    #[inline]
    fn index(&self) -> i32 {
        self.file_number
    }
}
impl<'f, M: OpenMode> ParentNode<'f, M, Base<'f, M>> for File<'f, M> {
    fn n_children(&self) -> CgnsResult<i32> {
        let mut nbases = 0;
        to_cgns_result(unsafe { cgns_bindings::cg_nbases(self.file_number, &mut nbases) })?;
        Ok(nbases)
    }
}
