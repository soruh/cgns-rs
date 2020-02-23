// TODO: make types thread-unsafe?

pub use libcgns_sys as bindings;

#[macro_use]
pub mod errors;
pub mod base;
pub mod cgio;
pub mod file;
pub mod types;
pub mod zone;

pub(crate) use base::{Base, BaseChildren, Bases};
pub(crate) use cgio::Cgio;
pub use errors::*;
pub(crate) use file::File;
pub(crate) use types::*;
pub(crate) use zone::{Zone, ZoneChildren, Zones};

use std::ffi::CString;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

/// Assures at runtime that the Library struct is not instanciated twice
/// You _really_ shouldn't touch this, since the wrapper relies on `Library` being a singleton
pub static LIB_IN_USE: AtomicBool = AtomicBool::new(false);

/// represents access to the CGNS library. Only one instance can exist at a time due to
/// the design of the CGNS library
pub struct Library {
    _phantom: PhantomData<()>,
}

impl Library {
    #[inline]
    pub fn new() -> Self {
        Self::take()
    }
    pub fn take() -> Self {
        assert!(
            !LIB_IN_USE.compare_and_swap(false, true, Ordering::Acquire),
            "The CGNS library is already in use."
        );
        Self {
            _phantom: Default::default(),
        }
    }

    #[inline]
    pub fn open<'l>(&'l self, filename: &str, mode: CgnsOpenMode) -> CgnsResult<File<'l>> {
        File::open(self, filename, mode)
    }

    pub fn goto(&self, path: &CgnsPath) -> CgnsResult<()> {
        let depth = path.nodes.len().min(bindings::CG_MAX_GOTO_DEPTH as usize);

        let mut label_buffs = Vec::with_capacity(depth);
        let mut labels = Vec::with_capacity(depth);
        let mut indicies = Vec::with_capacity(depth);

        for (label, index) in &path.nodes {
            let label = CString::new(label.as_str())?;
            // Safety: golist needs a *mut, but we can only get imutable pointers from a CString
            // since golist doesn't mutate the pointer we can safely transmute it.
            labels.push(unsafe { std::mem::transmute(label.as_ptr()) });
            label_buffs.push(label);
            indicies.push(*index);
        }

        to_cgns_result!(unsafe {
            bindings::cg_golist(
                path.file_number,
                path.base_index,
                depth as i32,
                labels.as_mut_ptr(),
                indicies.as_mut_ptr(),
            )
        })
    }

    pub fn current_path(&self) -> CgnsResult<CgnsPath> {
        use std::ffi::CStr;
        use std::mem::MaybeUninit;
        use std::os::raw::{c_char, c_int};

        let mut file_number = 0;
        let mut base_index = 0;
        let mut depth = 0;

        let mut labels: [MaybeUninit<*mut c_char>; bindings::CG_MAX_GOTO_DEPTH as usize] =
            [MaybeUninit::uninit(); bindings::CG_MAX_GOTO_DEPTH as usize];

        let mut indecies: [MaybeUninit<c_int>; bindings::CG_MAX_GOTO_DEPTH as usize] =
            [MaybeUninit::uninit(); bindings::CG_MAX_GOTO_DEPTH as usize];

        to_cgns_result!(unsafe {
            bindings::cg_where(
                &mut file_number,
                &mut base_index,
                &mut depth,
                &mut (labels.as_mut_ptr() as *mut c_char),
                &mut (indecies.as_mut_ptr() as c_int),
            )
        })?;

        let depth = depth as usize;
        let mut path = Vec::with_capacity(depth);
        for i in 0..depth {
            unsafe {
                let label = CStr::from_ptr(labels[i].assume_init())
                    .to_str()?
                    .to_string();
                let index = indecies[i].assume_init();
                path.push((CgnsNodeLabel::Custom(label), index));
            };
        }

        let path = CgnsPath {
            file_number,
            base_index,
            nodes: path,
        };

        Ok(path)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        assert!(
            LIB_IN_USE.compare_and_swap(true, false, Ordering::Release),
            "Singleton was instanciated twice"
        );
    }
}

impl std::fmt::Debug for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Library")
    }
}

pub trait Node<'p>
where
    Self: Sized,
    Self::Parent: Node<'p> + 'p,
{
    type Item;
    type Parent;
    type Children;
    const KIND: <Self::Parent as Node<'p>>::Children;

    fn path(&self) -> CgnsPath;
    fn read(&self) -> CgnsResult<Self::Item>;
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32>;
    fn new_unchecked(parent: &'p Self::Parent, index: i32) -> Self;
    fn n_children(&self, child_kind: Self::Children) -> CgnsResult<i32>;
    fn parent(&self) -> &Self::Parent;
    #[inline]
    fn lib(&self) -> &'p Library {
        self.parent().lib()
    }
    fn goto(&self) -> CgnsResult<()> {
        self.lib().goto(&self.path())
    }
    fn new(parent: &'p Self::Parent, index: i32) -> CgnsResult<Self> {
        if index > 0 && index <= parent.n_children(Self::KIND)? {
            Ok(Self::new_unchecked(parent, index))
        } else {
            Err(CgnsError::out_of_bounds())
        }
    }

    // TODO: delete (goto -> parent + delete)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_single_instance() {
        let lib = Library::take();
        drop(lib);
    }

    #[test]
    fn lib_two_instances_sequential() {
        let lib = Library::take();
        drop(lib);

        let lib2 = Library::take();
        drop(lib2);
    }

    #[test]
    #[should_panic]
    fn lib_two_instances_parralel() {
        let lib = Library::take();
        let lib2 = Library::take();

        drop(lib);
        drop(lib2);
    }

    #[test]
    fn open_file() {
        let lib = Library::new();

        let file = lib
            .open("test.cgns", CgnsOpenMode::Write)
            .expect("Failed to open file");

        file.close().expect("Failed to close file");
    }

    #[test]
    fn get_cgio() {
        let lib = Library::new();

        let file = lib
            .open("test.cgns", CgnsOpenMode::Write)
            .expect("Failed to open file");

        let cgio = file.cgio().expect("Failed to get cgio");

        cgio.cgio_number();
        cgio.root_id();
    }

    #[test]
    fn test_goto() {
        let lib = Library::new();
        let file = lib
            .open("../../TRACE.cgns", CgnsOpenMode::Read)
            .expect("failed to open file");
        let base = file.get_base(1).expect("Failed to get base");

        let path = base.path();
        lib.goto(&path).expect("failed to goto path");
        assert_eq!(
            path,
            lib.current_path().expect("failed to get current path")
        );
    }

    #[test]
    fn iter_bases() {
        let lib = Library::new();
        let file = lib
            .open("../../TRACE.cgns", CgnsOpenMode::Read)
            .expect("failed to open file");

        for base in file.bases().expect("failed to read number of bases") {
            println!("base: {:#?}", base.read().expect("failed to read base"));
        }
    }

    #[test]
    fn write_base() {
        let lib = Library::new();

        lib.open("base_test.cgns", CgnsOpenMode::Write)
            .expect("Failed to create file")
            .close()
            .expect("Failed to close file");

        let mut file = lib
            .open("base_test.cgns", CgnsOpenMode::Modify)
            .expect("Failed to open file");

        let base_data = base::BaseData {
            name: "New Base".into(),
            cell_dim: 3,
            phys_dim: 3,
        };

        let base_index = Base::write(&mut file, &base_data).expect("failed to write base");

        let base = file.get_base(base_index).expect("failed to get base");

        let data = base.read().expect("failed to read base");

        assert_eq!(data, base_data);
    }
}
