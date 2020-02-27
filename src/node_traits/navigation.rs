use super::*;

pub trait GotoTarget<M: OpenMode>: Node {
    fn path(&self) -> CgnsPath;
    #[inline]
    fn goto_lib(&self, lib: &Library) -> CgnsResult<()> {
        lib.goto(&self.path())
    }
    #[inline]
    fn goto(&self) -> CgnsResult<()>
    where
        Self: BaseRefNode<M>,
    {
        self.goto_lib(self.lib())
    }
}

pub trait BaseRefNode<M: OpenMode>: Node {
    fn base<'b>(&'b self) -> &'b Base<M>;

    #[inline]
    fn file<'f>(&'f self) -> &'f File<M> {
        self.base().file()
    }
    #[inline]
    fn lib<'l>(&'l self) -> &'l Library
    where
        M: 'l,
    {
        self.file().lib
    }
}
