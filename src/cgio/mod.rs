use super::*;

pub mod types;
pub use types::*;

use std::marker::PhantomData;

pub struct Cgio<'g, M> {
    cgio_number: i32,
    root_node: CgioNode,
    _phantom: PhantomData<&'g M>,
}

impl<'g, M: OpenMode> Cgio<'g, M> {
    pub(crate) fn from_file<'f>(file: &File<'f, M>) -> CgnsResult<Cgio<'f, M>> {
        let cgio_number = file.get_cgio_number()?;
        let root_id = file.root_id()?;

        Ok(Cgio {
            cgio_number,
            root_node: CgioNode { id: root_id },
            _phantom: Default::default(),
        })
    }

    pub fn cgio_number(&self) -> i32 {
        self.cgio_number
    }

    pub fn root_id(&self) -> f64 {
        self.root_node.id
    }
}
