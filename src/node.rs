use super::*;

pub trait GotoTarget: Node {
    const NodeLabel: CgnsNodeLabel;
    /// Note: This `must` be overwritten on SiblingNodes
    fn name(&self) -> CgnsResult<&str> {
        Ok(Self::NodeLabel.as_str())
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
        if let Some((node_label, node_index)) = self.path().nodes.last() {
            let lib = parent.lib();

            parent.goto_lib(&lib)?;

            let node_name = todo!();

            lib.delete_node(node_name)?;

            Ok(())
        } else {
            Err(CgnsError::node_not_found())
        }
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

pub trait IterableNode<'p>: SiblingNode<'p> + IndexableNode {
    // TODO: iter_mut?
    type Iterator: Iterator;
    fn iter(parent: Self::Parent) -> Self::Iterator;
}

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
