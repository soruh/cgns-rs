use super::*;

pub trait GotoTarget: Node {
    const NODE_LABEL: CgnsNodeLabel;
    /// Note: This `must` be overwritten on SiblingNodes
    fn name(&self) -> CgnsResult<String> {
        Ok(Self::NODE_LABEL.as_str().into())
    }
    fn path(&self) -> CgnsPath;
    #[inline]
    fn goto_lib(&self, lib: &Library) -> CgnsResult<()> {
        lib.goto(&self.path())
    }
    #[inline]
    fn goto(&self) -> CgnsResult<()>
    where
        Self: BaseRefNode,
    {
        self.goto_lib(self.lib())
    }
}

pub trait RwNode<'n>: ChildNode<'n> {
    type Item;

    fn read(&self) -> CgnsResult<Self::Item>;
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32>;

    /// Note: this invalidates sibling nodes with a higher index
    // TODO: should we check that there are no such nodes?
    fn delete(self, parent: &mut Self::Parent) -> CgnsResult<()>
    where
        Self: Sized + GotoTarget,
        Self::Parent: GotoTarget + BaseRefNode,
    {
        let lib = parent.lib();
        parent.goto_lib(&lib)?;
        lib.delete_node(self.name()?)
    }
}

pub trait ChildNode<'p>: Node + 'p + Sized
// TODO: Why do we need this + Sized bound?
where
    Self::Parent: ParentNode<'p, Self>,
{
    type Parent;
    fn parent(&self) -> &Self::Parent;
}

pub trait BaseRefNode: Node {
    fn base<'b>(&'b self) -> &'b Base;

    #[inline]
    fn file<'f>(&'f self) -> &'f File {
        self.base().file()
    }
    #[inline]
    fn lib<'l>(&'l self) -> &'l Library {
        self.file().lib
    }
}

pub trait OnlyChildNode<'p>: ChildNode<'p> {
    fn new(parent: &'p Self::Parent) -> Self;
}

pub trait IndexableNode: Node {
    fn index(&self) -> i32;
}

pub struct NodeIter<'p, S>
where
    S: SiblingNode<'p>,
{
    n_items: i32,
    current: i32,
    parent: &'p S::Parent,
}

impl<'p, S> Iterator for NodeIter<'p, S>
where
    S: SiblingNode<'p>,
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

pub trait IterableNode<'p>: SiblingNode<'p> {
    fn iter(parent: &'p Self::Parent) -> CgnsResult<NodeIter<'p, Self>> {
        Ok(NodeIter {
            current: 0,
            n_items: parent.n_children()?,
            parent,
        })
    }
}

impl<'p, N> IterableNode<'p> for N where N: SiblingNode<'p> {}

pub trait SiblingNode<'p>: ChildNode<'p> + IndexableNode {
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

pub trait ParentNode<'p, C>: Node
where
    C: ChildNode<'p> + 'p,
{
    fn n_children(&self) -> CgnsResult<i32>
    where
        C: SiblingNode<'p>;
}

pub trait Node {}
