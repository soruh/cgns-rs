use super::*;
use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    os::raw::c_char,
};
pub struct Zone<'z, M: OpenMode> {
    base: &'z Base<'z, M>,
    zone_index: i32,
}

// TODO: implement constructor methods that ensure required invariants
// i.e. n_cell = n_vertex - 1 etc.

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct StructuredZoneSize {
    pub n_vertex: (i32, i32, i32),
    pub n_cell: (i32, i32, i32),
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct UnstructuredZoneSize {
    pub n_vertex: (i32, i32, i32),
    pub n_cell: (i32, i32, i32),
    pub b_bound_vertex: (i32, i32, i32),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ZoneSize {
    Structured(StructuredZoneSize),
    Unstructured(UnstructuredZoneSize),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ZoneData {
    pub name: String,
    pub size: ZoneSize,
}

impl<'z, M: OpenMode> Zone<'z, M> {
    pub fn index_dim(&self) -> CgnsResult<i32> {
        let mut index_dim = 0;

        to_cgns_result(unsafe {
            cgns_bindings::cg_index_dim(
                self.file().file_number(),
                self.base().index(),
                self.index(),
                &mut index_dim,
            )
        })?;

        Ok(index_dim)
    }
}

impl<'z, M: OpenMode> Node for Zone<'z, M> {}

impl<'z, M: OpenMode> LabeledNode for Zone<'z, M> {
    const NODE_LABEL: CgnsNodeLabel = CgnsNodeLabel::Zone;
}

impl<'z, M: OpenMode> NamedNode<M> for Zone<'z, M>
where
    M: OpenModeRead,
{
    fn name(&self) -> CgnsResult<String> {
        Ok(String::from(&self.read()?.name))
    }
}

impl<'z, M: OpenMode> GotoTarget<M> for Zone<'z, M> {
    fn path(&self) -> CgnsPath {
        let mut path = self.base.path();
        path.nodes.push((CgnsNodeLabel::Zone, self.zone_index));
        path
    }
}

impl<'z, M: OpenMode> RwNode<'z, M> for Zone<'z, M> {
    type Item = ZoneData;
    fn read(&self) -> CgnsResult<Self::Item> {
        let mut zone_type: cgns_bindings::ZoneType_t = 0;
        to_cgns_result(unsafe {
            cgns_bindings::cg_zone_type(
                self.file().file_number(),
                self.base().index(),
                self.zone_index,
                &mut zone_type,
            )
        })?;

        let mut zonename = [MaybeUninit::<c_char>::uninit(); 33];
        let mut size_buffer = [MaybeUninit::<i32>::uninit(); 9];

        to_cgns_result(unsafe {
            cgns_bindings::cg_zone_read(
                self.file().file_number(),
                self.base().index(),
                self.index(),
                zonename.as_mut_ptr() as *mut c_char,
                size_buffer.as_mut_ptr() as *mut i32,
            )
        })?;

        let name: String = unsafe { CStr::from_ptr(zonename.as_ptr() as *const i8) }
            .to_str()?
            .to_string();

        // [NVertexI, NVertexJ, NVertexK, NCellI, NCellJ, NCellK, NBoundVertexI, NBoundVertexJ, NBoundVertexK]
        Ok(match zone_type {
            cgns_bindings::ZoneType_t_Structured => ZoneData {
                name,
                size: ZoneSize::Structured(unsafe {
                    StructuredZoneSize {
                        n_vertex: (
                            size_buffer[0].assume_init(),
                            size_buffer[1].assume_init(),
                            size_buffer[2].assume_init(),
                        ),
                        n_cell: (
                            size_buffer[3].assume_init(),
                            size_buffer[4].assume_init(),
                            size_buffer[5].assume_init(),
                        ),
                    }
                }),
            },
            cgns_bindings::ZoneType_t_Unstructured => ZoneData {
                name,
                size: ZoneSize::Unstructured(unsafe {
                    UnstructuredZoneSize {
                        n_vertex: (
                            size_buffer[0].assume_init(),
                            size_buffer[1].assume_init(),
                            size_buffer[2].assume_init(),
                        ),
                        n_cell: (
                            size_buffer[3].assume_init(),
                            size_buffer[4].assume_init(),
                            size_buffer[5].assume_init(),
                        ),
                        b_bound_vertex: (
                            size_buffer[6].assume_init(),
                            size_buffer[7].assume_init(),
                            size_buffer[8].assume_init(),
                        ),
                    }
                }),
            },
            _ => Err(CgnsError::invalid_lib_result())?,
        })
    }
    fn write(parent: &mut Self::Parent, data: &Self::Item) -> CgnsResult<i32> {
        let mut zone_index = 0;

        let name = CString::new(data.name.clone())?;
        let (size, zone_type) = match &data.size {
            ZoneSize::Structured(size) => {
                let mut size_buffer = [0; 9];
                size_buffer[0] = size.n_vertex.0;
                size_buffer[1] = size.n_vertex.1;
                size_buffer[2] = size.n_vertex.2;
                size_buffer[3] = size.n_cell.0;
                size_buffer[4] = size.n_cell.1;
                size_buffer[5] = size.n_cell.2;

                (size_buffer, cgns_bindings::ZoneType_t_Structured)
            }
            ZoneSize::Unstructured(size) => {
                let mut size_buffer = [0; 9];
                size_buffer[0] = size.n_vertex.0;
                size_buffer[1] = size.n_vertex.1;
                size_buffer[2] = size.n_vertex.2;
                size_buffer[3] = size.n_cell.0;
                size_buffer[4] = size.n_cell.1;
                size_buffer[5] = size.n_cell.2;
                size_buffer[6] = size.b_bound_vertex.0;
                size_buffer[7] = size.b_bound_vertex.1;
                size_buffer[8] = size.b_bound_vertex.2;

                (size_buffer, cgns_bindings::ZoneType_t_Unstructured)
            }
        };

        to_cgns_result(unsafe {
            cgns_bindings::cg_zone_write(
                parent.file().file_number(),
                parent.index(),
                name.as_ptr(),
                size.as_ptr(),
                zone_type,
                &mut zone_index,
            )
        })?;

        Ok(zone_index)
    }
}

impl<'z, M: OpenMode> ChildNode<'z, M> for Zone<'z, M> {
    type Parent = Base<'z, M>;

    #[inline]
    fn parent(&self) -> &Self::Parent {
        self.base
    }
}
impl<'z, M: OpenMode> BaseRefNode<M> for Zone<'z, M> {
    #[inline]
    fn base<'b>(&'b self) -> &'b Base<M> {
        self.base
    }
}

impl<'z, M: OpenMode> IndexableNode for Zone<'z, M> {
    #[inline]
    fn index(&self) -> i32 {
        self.zone_index
    }
}

impl<'z, M: OpenMode> SiblingNode<'z, M> for Zone<'z, M> {
    #[inline]
    fn new_unchecked(parent: &'z Self::Parent, zone_index: i32) -> Self {
        Zone {
            base: parent,
            zone_index,
        }
    }
}

// impl<'z, M: OpenMode> IterableNode<'z> for Zone<'z, M> {}

// TODO
// impl<'z, M: OpenMode> ParentNode<'z> for Zone<'z, M> {
//     fn n_children(&self, child_kind: Self::Children) -> CgnsResult<i32> {}
// }
