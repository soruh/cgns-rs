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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DescriptorData {
    pub name: String,
    pub value: String,
}

impl<'p, P> Node for Descriptor<'p, P> where P: Node {}

// TODO: why do we need this + Sized bound?
pub trait DescriptorParent<'p>
where
    Self: ParentNode<'p, Descriptor<'p, Self>> + 'p + Sized,
{
    fn get_descriptor(&'p self, descriptor_index: i32) -> CgnsResult<Descriptor<'p, Self>> {
        Descriptor::new(self, descriptor_index)
    }
    fn set_descriptor(&mut self, descriptor_data: &DescriptorData) -> CgnsResult<()>
    where
        Self: GotoTarget + BaseRefNode,
    {
        Descriptor::write(self, descriptor_data)?;

        Ok(())
    }
    fn n_descriptors(&self) -> CgnsResult<i32> {
        self.n_children()
    }
}

impl<'p, N> DescriptorParent<'p> for N where N: ParentNode<'p, Descriptor<'p, N>> + 'p {}

impl<'p, P> GotoTarget for Descriptor<'p, P>
where
    P: 'p + ParentNode<'p, Descriptor<'p, P>> + GotoTarget,
{
    fn path(&self) -> CgnsPath {
        let mut path = self.parent.path();
        path.nodes.push((CgnsNodeLabel::Descriptor, self.index()));
        path
    }
}

impl<'p, N> ParentNode<'p, Descriptor<'p, N>> for N
where
    N: GotoTarget + ChildNode<'p> + BaseRefNode,
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
    #[inline]
    fn index(&self) -> i32 {
        self.descriptor_index
    }
    #[inline]
    fn new_unchecked(parent: &'p Self::Parent, descriptor_index: i32) -> Self {
        Descriptor {
            parent,
            descriptor_index,
        }
    }
}

impl<'p, P> BaseRefNode for Descriptor<'p, P>
where
    P: BaseRefNode + ParentNode<'p, Self>,
{
    #[inline]
    fn base<'b>(&'b self) -> &'b Base {
        self.parent().base()
    }
}

impl<'p, P> RwNode<'p> for Descriptor<'p, P>
where
    P: ParentNode<'p, Descriptor<'p, P>>, // TODO: + 'p ?
    Self: ChildNode<'p>,
    Self::Parent: GotoTarget + BaseRefNode,
{
    type Item = DescriptorData;
    fn read(&self) -> CgnsResult<Self::Item> {
        let mut name = [MaybeUninit::<c_char>::uninit(); 33];
        let mut value = MaybeUninit::<*mut c_char>::uninit();

        to_cgns_result!(unsafe {
            bindings::cg_descriptor_read(
                self.index(),
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
