use super::*;

pub struct Zone<'z> {
    base: &'z Base<'z>,
    zone_index: i32,
}

pub struct ZoneData {}

pub enum ZoneChildren {}

impl<'z> Zone<'z> {}

impl<'z> Node<'z> for Zone<'z> {
    type Item = ZoneData;
    type Parent = Base<'z>;
    type Children = ZoneChildren;
    const KIND: BaseChildren = BaseChildren::Zone;
    fn path(&self) -> CgnsPath {
        let mut path = self.base.path();

        path.nodes.push((CgnsNodeLabel::Zone, self.zone_index));

        path
    }
    fn new_unchecked(parent: &'z Self::Parent, zone_index: i32) -> Self {
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
    fn n_children(&self, child_kind: Self::Children) -> CgnsResult<i32> {
        match child_kind {
            _ => todo!(),
        }
    }
    #[inline]
    fn parent(&self) -> &Self::Parent {
        self.base
    }
}

pub struct Zones {}
// TODO
