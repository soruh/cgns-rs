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
        Self: BaseRefNode,
    {
        self.goto_lib(self.lib())
    }
}

pub trait RwNode<'n>
where
    Self: ChildNode<'n>,
{
    type Item;

    fn read(&self) -> CgnsResult<Self::Item>;
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32>;

    /// Note: this invalidates sibling nodes with a higher index
    // TODO: should we check that there are no such nodes?
    fn delete(self, parent: &mut Self::Parent) -> CgnsResult<()>
    where
        Self: Sized + GotoTarget + BaseRefNode,
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

pub trait ChildNode<'p>
where
    Self: Node + 'p + Sized, // TODO: Why do we need this + Sized bound?
    Self::Parent: ParentNode<'p, Self>,
{
    type Parent;
    fn parent(&self) -> &Self::Parent;
}

pub trait BaseRefNode
where
    Self: Node,
{
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

pub trait OnlyChildNode<'p>
where
    Self: ChildNode<'p>,
{
    fn new(parent: &'p Self::Parent) -> Self;
}

pub trait SiblingNode<'p>
where
    Self: ChildNode<'p>,
    Self::Parent: 'p,
{
    fn index(&self) -> i32;
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

pub trait ParentNode<'p, C>
where
    Self: Node,
    C: ChildNode<'p> + 'p,
{
    fn n_children(&self) -> CgnsResult<i32>
    where
        C: SiblingNode<'p>;
}

pub trait Node {}
