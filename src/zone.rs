use super::*;

pub struct Zone<'z> {
    base: &'z Base<'z>,
    zone_index: i32,
}

pub struct ZoneData {}

impl<'z> Node<'z> for Zone<'z> {
    type Item = ZoneData;
    type Parent = Base<'z>;
    fn path(&self) -> CgnsPath {
        let mut path = self.base.path();

        path.nodes.push((CgnsNodeLabel::Zone, self.zone_index));

        path
    }
    fn new(parent: &'z Self::Parent, zone_index: i32) -> Self
    where
        Self: Sized,
    {
        Zone {
            base: parent,
            zone_index,
        }
    }

    fn read(&self) -> CgnsResult<Self::Item> {
        todo!()
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        todo!()
    }
}

pub struct Zones {}
// TODO
