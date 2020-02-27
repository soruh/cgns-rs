use super::*;

pub struct NodeIter<'p, M: OpenMode, S>
where
    S: SiblingNode<'p, M>,
{
    n_items: i32,
    current: i32,
    parent: &'p S::Parent,
}

impl<'p, M: OpenMode + 'p, S> Iterator for NodeIter<'p, M, S>
where
    S: SiblingNode<'p, M>,
{
    type Item = S;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.n_items {
            self.current += 1;
            // CGNS idecies start at 1, so we increment before using the index
            Some(S::new_unchecked(self.parent, self.current))
        } else {
            None
        }
    }
}

pub trait IterableNode<'p, M: OpenMode + 'p>: SiblingNode<'p, M> {
    fn iter(parent: &'p Self::Parent) -> CgnsResult<NodeIter<'p, M, Self>> {
        Ok(NodeIter {
            current: 0,
            n_items: parent.n_children()?,
            parent,
        })
    }
}

impl<'p, M: OpenMode + 'p, N> IterableNode<'p, M> for N where N: SiblingNode<'p, M> {}
