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
impl From<i32> for OrdinalData {
    fn from(data: i32) -> Self {
        OrdinalData(data)
    }
}
impl From<OrdinalData> for i32 {
    fn from(data: OrdinalData) -> Self {
        data.0
    }
}

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
        self.parent().goto()?;
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

impl<'p, P: BaseRefNode<M> + GotoTarget<M>, M: OpenMode> LabeledNode for Ordinal<'p, M, P> {
    const NODE_LABEL: CgnsNodeLabel = CgnsNodeLabel::Ordinal;
}

impl<'p, M: OpenMode, P> GotoTarget<M> for Ordinal<'p, M, P>
where
    P: ParentNode<'p, M, Self> + GotoTarget<M> + BaseRefNode<M>,
{
    fn path(&self) -> CgnsPath {
        let mut path = self.parent.path();
        path.nodes.push((CgnsNodeLabel::Ordinal, 0));
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

pub trait OrdinalParent<'p, M: OpenMode + 'p>:
    ParentNode<'p, M, Ordinal<'p, M, Self>> + 'p + Sized + GotoTarget<M> + BaseRefNode<M>
{
    fn get_ordinal(&'p self) -> CgnsResult<OrdinalData>
    where
        M: OpenModeRead,
    {
        Ordinal::new(self).read()
    }
    fn set_ordinal(&mut self, ordinal_data: &OrdinalData) -> CgnsResult<()>
    where
        M: OpenModeWrite,
    {
        Ordinal::write(self, ordinal_data)?;
        Ok(())
    }
}

impl<'p, M: OpenMode + 'p, N> OrdinalParent<'p, M> for N where
    N: ParentNode<'p, M, Ordinal<'p, M, N>> + 'p + GotoTarget<M> + BaseRefNode<M>
{
}
