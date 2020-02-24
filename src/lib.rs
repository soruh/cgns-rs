// TODO: make types thread-unsafe?

pub use libcgns_sys as bindings;

#[macro_use]
pub mod errors;
pub mod cgio;
pub mod file;
pub mod node;
pub mod nodes;
pub mod types;

pub use cgio::Cgio;
pub use errors::*;
pub use file::*;
pub use node::*;
pub use nodes::*;
pub use types::*;

use std::ffi::CString;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

/// Assures at runtime that the Library struct is not instanciated twice
/// You _really_ shouldn't touch this, since the wrapper relies on `Library` being a singleton
pub static LIB_IN_USE: AtomicBool = AtomicBool::new(false);

/// represents access to the CGNS library. Only one instance can exist at a time due to
/// the design of the CGNS library
pub struct Library {
    _phantom: PhantomData<*const ()>,
}

impl Library {
    #[inline]
    pub fn new() -> Self {
        Self::take()
    }
    pub fn take() -> Self {
        if LIB_IN_USE.compare_and_swap(false, true, Ordering::Acquire) {
            panic!("The CGNS library is already in use.");
        }

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

        to_cgns_result(unsafe {
            bindings::cg_golist(
                path.file_number,
                path.base_index,
                depth as i32,
                labels.as_mut_ptr(),
                indicies.as_mut_ptr(),
            )
        })
    }

    pub fn delete_node(&self, node_name: String) -> CgnsResult<()> {
        let node_name = CString::new(node_name)?;
        to_cgns_result(unsafe { bindings::cg_delete_node(node_name.as_ptr()) })
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

        to_cgns_result(unsafe {
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
