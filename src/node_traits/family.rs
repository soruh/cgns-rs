use super::*;

pub trait ChildNode<'p, M: OpenMode + 'p>: Node + 'p + Sized
// TODO: Why do we need this + Sized bound?
where
    Self::Parent: ParentNode<'p, M, Self>,
{
    type Parent;
    fn parent(&self) -> &Self::Parent;
}

pub trait OnlyChildNode<'p, M: OpenMode + 'p>: ChildNode<'p, M> {
    fn new(parent: &'p Self::Parent) -> Self;
}

pub trait ParentNode<'p, M: OpenMode + 'p, C>: Node
where
    C: ChildNode<'p, M> + 'p,
{
    fn n_children(&self) -> CgnsResult<i32>
    where
        C: SiblingNode<'p, M>;
}

pub trait SiblingNode<'p, M: OpenMode + 'p>: ChildNode<'p, M> + IndexableNode {
    fn new_unchecked(parent: &'p Self::Parent, index: i32) -> Self;
    fn new(parent: &'p Self::Parent, index: i32) -> CgnsResult<Self>
    where
        Self: Sized,
    {
        if index > 0 && index <= parent.n_children()? {
            Ok(Self::new_unchecked(parent, index))
        } else {
            Err(CgnsError::out_of_bounds())
        }
    }
}
