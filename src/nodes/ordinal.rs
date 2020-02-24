use super::*;
pub struct Ordinal<'p, P>
where
    P: ParentNode<'p, Self>,
{
    parent: &'p P,
}

pub struct OrdinalData(pub i32);

impl<'p, P> Node for Ordinal<'p, P> where P: ParentNode<'p, Self> {}

impl<'p, P> ChildNode<'p> for Ordinal<'p, P>
where
    P: ParentNode<'p, Self>,
{
    type Parent = P;
    fn parent(&self) -> &Self::Parent {
        self.parent
    }
}

impl<'p, N> ParentNode<'p, Ordinal<'p, Self>> for N
where
    N: Node + GotoTarget + BaseRefNode,
{
    fn n_children(&self) -> CgnsResult<i32>
    where
        Ordinal<'p, Self>: SiblingNode<'p>,
    {
        unreachable!()
    }
}

impl<'p, P> OnlyChildNode<'p> for Ordinal<'p, P>
where
    P: ParentNode<'p, Self>,
{
    fn new(parent: &'p Self::Parent) -> Self {
        Self { parent }
    }
}

impl<'p, P> RwNode<'p> for Ordinal<'p, P>
where
    P: ParentNode<'p, Self> + GotoTarget + BaseRefNode,
{
    type Item = OrdinalData;
    fn read(&self) -> CgnsResult<Self::Item> {
        let mut ordinal = 0;
        to_cgns_result(unsafe { bindings::cg_ordinal_read(&mut ordinal) })?;
        Ok(OrdinalData(ordinal))
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        parent.goto()?;
        to_cgns_result(unsafe { bindings::cg_ordinal_write(data.0) })?;
        Ok(0)
    }
}

impl<'p, P> GotoTarget for Ordinal<'p, P>
where
    P: ParentNode<'p, Self> + GotoTarget + BaseRefNode,
{
    const NODE_LABEL: CgnsNodeLabel = CgnsNodeLabel::Ordinal;
    fn path(&self) -> CgnsPath {
        let mut path = self.parent.path();
        path.nodes.push((CgnsNodeLabel::Descriptor, 0));
        path
    }
}

impl<'p, P> BaseRefNode for Ordinal<'p, P>
where
    P: BaseRefNode + GotoTarget,
{
    #[inline]
    fn base<'b>(&'b self) -> &'b Base {
        self.parent().base()
    }
}
