use super::*;

pub trait GotoTarget {
    fn path(&self) -> CgnsPath;
    #[inline]
    fn goto_lib(&self, lib: &Library) -> CgnsResult<()> {
        lib.goto(&self.path())
    }
    #[inline]
    fn goto(&self) -> CgnsResult<()>
    where
        Self: LeafNode,
    {
        self.goto_lib(self.lib())
    }
}

pub trait RwNode
where
    Self: ChildNode,
{
    type Item;

    fn read(&self) -> CgnsResult<Self::Item>;
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32>;

    /// Note: this invalidates sibling nodes with a higher index
    // TODO: should we check that there are no such nodes?
    fn delete(self, parent: &mut Self::Parent) -> CgnsResult<()>
    where
        Self: Sized + GotoTarget + LeafNode,
        Self::Parent: GotoTarget,
    {
        if let Some((node_label, node_index)) = self.path().nodes.last() {
            let lib = self.lib();

            self.parent().goto_lib(&lib)?;

            let node_name = todo!();

            lib.delete_node(node_name)?;

            Ok(())
        } else {
            Err(CgnsError::node_not_found())
        }
    }
}

pub trait ChildNode
where
    Self: Node,
    Self::Parent: ParentNode,
{
    type Parent;
    const KIND: <Self::Parent as ParentNode>::Children;
    fn parent(&self) -> &Self::Parent;
}

pub trait LeafNode {
    fn base<'b>(&'b self) -> &'b Base;

    #[inline]
    fn file<'f>(&'f self) -> &'f File {
        self.base().file
    }
    #[inline]
    fn lib<'l>(&'l self) -> &'l Library {
        self.file().lib
    }
}

pub trait OnlyChildNode<'p>
where
    Self: ChildNode,
{
    fn new(parent: &'p Self::Parent) -> Self;
}

pub trait SiblingNode<'p>
where
    Self: ChildNode,
{
    fn new_unchecked(parent: &'p Self::Parent, index: i32) -> Self;
    fn new(parent: &'p Self::Parent, index: i32) -> CgnsResult<Self>
    where
        Self: Sized,
    {
        if index > 0 && index <= parent.n_children(Self::KIND)? {
            Ok(Self::new_unchecked(parent, index))
        } else {
            Err(CgnsError::out_of_bounds())
        }
    }
}

pub trait ParentNode
where
    Self: Node,
{
    type Children;

    fn n_children(&self, child_kind: Self::Children) -> CgnsResult<i32>;
}

pub trait Node {}
