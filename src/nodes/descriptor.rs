use super::*;

pub struct Descriptor<'p, P>
where
    P: Node,
{
    parent: &'p P,
    descriptor_index: i32,
}

pub struct DescriptorData {}

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
{
    type Item = DescriptorData;
    fn read(&self) -> CgnsResult<Self::Item> {
        unimplemented!()
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        unimplemented!()
    }
}
