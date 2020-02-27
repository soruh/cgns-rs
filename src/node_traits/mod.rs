use super::*;

pub mod family;
pub mod iter;
pub mod named;
pub mod navigation;

pub use family::*;
pub use iter::*;
pub use named::*;
pub use navigation::*;

pub trait Node {}

pub trait IndexableNode: Node {
    fn index(&self) -> i32;
}

pub trait RwNode<'n, M: OpenMode + 'n>: ChildNode<'n, M> {
    type Item;

    fn read(&self) -> CgnsResult<Self::Item>
    where
        M: OpenModeRead;
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32>
    where
        M: OpenModeWrite;

    /// Note: this invalidates sibling nodes with a higher index
    // TODO: should we check that there are no such nodes?
    // TODO: relax trait bounds?
    fn delete_by_name(self, parent: &mut Self::Parent, name: String) -> CgnsResult<()>
    where
        Self: Sized + GotoTarget<M>,
        Self::Parent: GotoTarget<M> + BaseRefNode<M>,
    {
        let lib = parent.lib();
        parent.goto_lib(&lib)?;
        lib.delete_node(name)
    }

    fn delete(self, parent: &mut Self::Parent) -> CgnsResult<()>
    where
        Self: Sized + GotoTarget<M> + NamedNode<M>,
        Self::Parent: GotoTarget<M> + BaseRefNode<M>,
    {
        let name = self.name()?;
        self.delete_by_name(parent, name)
    }
}
