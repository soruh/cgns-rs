use super::*;

pub enum SimulationTypeData {
    Null,
    UserDefined,
    TimeAccurate,
    NonTimeAccurate,
}

pub struct SimulationType<'s> {
    base: &'s Base<'s>,
}

impl<'s> Node for SimulationType<'s> {}

impl<'s> GotoTarget for SimulationType<'s> {
    fn path(&self) -> CgnsPath {
        let mut path = self.base.path();
        path.nodes.push((CgnsNodeLabel::SimulationType, 0));
        path
    }
}

impl<'s> BaseRefNode for SimulationType<'s> {
    #[inline]
    fn base<'b>(&'b self) -> &'b Base {
        self.base
    }
}

impl<'s> RwNode<'s> for SimulationType<'s> {
    type Item = SimulationTypeData;

    fn read(&self) -> CgnsResult<Self::Item> {
        let mut simulation_type = 0;

        to_cgns_result!(unsafe {
            bindings::cg_simulation_type_read(
                self.file().file_number(),
                self.base().index(),
                &mut simulation_type,
            )
        })?;

        Ok(match simulation_type {
            bindings::CG_Null => SimulationTypeData::Null,
            bindings::CG_UserDefined => SimulationTypeData::UserDefined,
            bindings::SimulationType_t_TimeAccurate => SimulationTypeData::TimeAccurate,
            bindings::SimulationType_t_NonTimeAccurate => SimulationTypeData::NonTimeAccurate,

            _ => Err(CgnsError::invalid_lib_result())?,
        })
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        let simulation_type = match data {
            SimulationTypeData::Null => bindings::CG_Null,
            SimulationTypeData::UserDefined => bindings::CG_UserDefined,
            SimulationTypeData::TimeAccurate => bindings::SimulationType_t_TimeAccurate,
            SimulationTypeData::NonTimeAccurate => bindings::SimulationType_t_NonTimeAccurate,
        };

        to_cgns_result!(unsafe {
            bindings::cg_simulation_type_write(
                parent.file().file_number(),
                parent.index(),
                simulation_type,
            )
        })?;

        Ok(0)
    }
}

impl<'s> ChildNode<'s> for SimulationType<'s> {
    type Parent = Base<'s>;

    fn parent(&self) -> &Self::Parent {
        self.base
    }
}

impl<'s> OnlyChildNode<'s> for SimulationType<'s> {
    #[inline]
    fn new(parent: &'s Self::Parent) -> Self {
        SimulationType { base: parent }
    }
}
