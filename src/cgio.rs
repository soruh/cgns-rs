use super::*;
use std::marker::PhantomData;
pub struct Cgio<'g> {
    cgio_number: i32,
    root_id: Option<f64>,
    _phantom: PhantomData<&'g ()>,
}

impl<'g> Cgio<'g> {
    pub(crate) fn from_file<'f>(file: &File<'f>) -> CgnsResult<Cgio<'f>> {
        let cgio_number = file.get_cgio_number()?;
        let root_id = file.root_id()?;

        Ok(Cgio {
            cgio_number,
            root_id: Some(root_id),
            _phantom: Default::default(),
        })
    }

    pub fn cgio_number(&self) -> i32 {
        self.cgio_number
    }

    pub fn root_id(&self) -> Option<f64> {
        self.root_id
    }
}
