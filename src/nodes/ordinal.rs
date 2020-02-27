use super::*;
use std::marker::PhantomData;

pub struct Ordinal<'p, M: OpenMode + 'p, P>
where
    P: ParentNode<'p, M, Self>,
{
    parent: &'p P,
    _phantom: PhantomData<M>,
}

pub struct OrdinalData(pub i32);

impl<'p, M: OpenMode, P> Node for Ordinal<'p, M, P> where P: ParentNode<'p, M, Self> {}

impl<'p, M: OpenMode, P> ChildNode<'p, M> for Ordinal<'p, M, P>
where
    P: ParentNode<'p, M, Self>,
{
    type Parent = P;
    fn parent(&self) -> &Self::Parent {
        self.parent
    }
}

impl<'p, N, M: OpenMode> ParentNode<'p, M, Ordinal<'p, M, Self>> for N
where
    N: Node + GotoTarget<M> + BaseRefNode<M>,
{
    fn n_children(&self) -> CgnsResult<i32>
    where
        Ordinal<'p, M, Self>: SiblingNode<'p, M>,
    {
        unreachable!()
    }
}

impl<'p, M: OpenMode, P> OnlyChildNode<'p, M> for Ordinal<'p, M, P>
where
    P: ParentNode<'p, M, Self>,
{
    fn new(parent: &'p Self::Parent) -> Self {
        Self {
            parent,
            _phantom: Default::default(),
        }
    }
}

impl<'p, M: OpenMode, P> RwNode<'p, M> for Ordinal<'p, M, P>
where
    P: ParentNode<'p, M, Self> + GotoTarget<M> + BaseRefNode<M>,
{
    type Item = OrdinalData;
    fn read(&self) -> CgnsResult<Self::Item> {
        let mut ordinal = 0;
        to_cgns_result(unsafe { cgns_bindings::cg_ordinal_read(&mut ordinal) })?;
        Ok(OrdinalData(ordinal))
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        parent.goto()?;
        to_cgns_result(unsafe { cgns_bindings::cg_ordinal_write(data.0) })?;
        Ok(0)
    }
}

impl<'p, M: OpenMode, P> GotoTarget<M> for Ordinal<'p, M, P>
where
    P: ParentNode<'p, M, Self> + GotoTarget<M> + BaseRefNode<M>,
{
    const NODE_LABEL: CgnsNodeLabel = CgnsNodeLabel::Ordinal;
    fn path(&self) -> CgnsPath {
        let mut path = self.parent.path();
        path.nodes.push((CgnsNodeLabel::Descriptor, 0));
        path
    }
}

impl<'p, M: OpenMode, P> BaseRefNode<M> for Ordinal<'p, M, P>
where
    P: BaseRefNode<M> + GotoTarget<M>,
{
    #[inline]
    fn base<'b>(&'b self) -> &'b Base<M> {
        self.parent().base()
    }
}
