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
    const NODE_LABEL: CgnsNodeLabel = CgnsNodeLabel::SimulationType;
    fn path(&self) -> CgnsPath {
        let mut path = self.parent().path();
        path.nodes.push((Self::NODE_LABEL, 0));
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

        to_cgns_result(unsafe {
            cgns_bindings::cg_simulation_type_read(
                self.file().file_number(),
                self.base().index(),
                &mut simulation_type,
            )
        })?;

        Ok(match simulation_type {
            cgns_bindings::CG_Null => SimulationTypeData::Null,
            cgns_bindings::CG_UserDefined => SimulationTypeData::UserDefined,
            cgns_bindings::SimulationType_t_TimeAccurate => SimulationTypeData::TimeAccurate,
            cgns_bindings::SimulationType_t_NonTimeAccurate => SimulationTypeData::NonTimeAccurate,

            _ => Err(CgnsError::invalid_lib_result())?,
        })
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        let simulation_type = match data {
            SimulationTypeData::Null => cgns_bindings::CG_Null,
            SimulationTypeData::UserDefined => cgns_bindings::CG_UserDefined,
            SimulationTypeData::TimeAccurate => cgns_bindings::SimulationType_t_TimeAccurate,
            SimulationTypeData::NonTimeAccurate => cgns_bindings::SimulationType_t_NonTimeAccurate,
        };

        to_cgns_result(unsafe {
            cgns_bindings::cg_simulation_type_write(
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
