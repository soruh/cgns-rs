use super::*;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_void};

pub struct Descriptor<'p, P>
where
    P: Node,
{
    parent: &'p P,
    descriptor_index: i32,
}

pub struct DescriptorData {
    name: String,
    value: String,
}

impl<'p, P> Node for Descriptor<'p, P> where P: Node {}

impl<'p, N> ParentNode<'p, N> for N
where
    N: GotoTarget + ChildNode<'p> + LeafNode,
{
    fn n_children(&self) -> CgnsResult<i32> {
        self.goto()?;

        let mut n_descriptors = 0;

        to_cgns_result!(unsafe { bindings::cg_ndescriptors(&mut n_descriptors) })?;

        Ok(n_descriptors)
    }
}

impl<'p, P> ChildNode<'p> for Descriptor<'p, P>
where
    P: 'p + ParentNode<'p, Descriptor<'p, P>>,
{
    type Parent = P;
    fn parent(&self) -> &Self::Parent {
        self.parent
    }
}

impl<'p, P> SiblingNode<'p> for Descriptor<'p, P>
where
    P: 'p + ParentNode<'p, Descriptor<'p, P>>,
{
    fn new_unchecked(parent: &'p Self::Parent, descriptor_index: i32) -> Self {
        Descriptor {
            parent,
            descriptor_index,
        }
    }
}

impl<'p, P> LeafNode for Descriptor<'p, P>
where
    P: LeafNode + ParentNode<'p, Self>,
{
    fn base<'b>(&'b self) -> &'b Base {
        self.parent().base()
    }
}

impl<'p, P> RwNode<'p> for Descriptor<'p, P>
where
    P: ParentNode<'p, Descriptor<'p, P>>, // TODO: + 'p ?
    Self: ChildNode<'p>,
    Self::Parent: GotoTarget + LeafNode,
{
    type Item = DescriptorData;
    fn read(&self) -> CgnsResult<Self::Item> {
        let mut name = [MaybeUninit::<c_char>::uninit(); 33];
        let mut value = MaybeUninit::<*mut c_char>::uninit();

        to_cgns_result!(unsafe {
            bindings::cg_descriptor_read(
                self.descriptor_index,
                name.as_mut_ptr() as *mut c_char,
                value.as_mut_ptr(),
            )
        })?;

        let descriptor_data = DescriptorData {
            name: unsafe { CStr::from_ptr(name.as_ptr() as *const c_char) }
                .to_str()?
                .to_string(),
            value: unsafe { CStr::from_ptr(value.assume_init()) }
                .to_str()?
                .to_string(),
        };

        to_cgns_result!(unsafe { bindings::cg_free(value.assume_init() as *mut c_void) })?;

        Ok(descriptor_data)
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        let name = CString::new(data.name.clone())?;
        let value = CString::new(data.value.clone())?;

        parent.goto()?;

        to_cgns_result!(unsafe { bindings::cg_descriptor_write(name.as_ptr(), value.as_ptr()) })?;

        Ok(-1)
    }
}
