use super::*;

pub trait LabeledNode {
    const NODE_LABEL: CgnsNodeLabel;
}

pub trait NamedNode<M: OpenMode>: LabeledNode {
    fn name(&self) -> CgnsResult<String> {
        Ok(Self::NODE_LABEL.to_string())
    }
}

// NOTE: This doesn't work due to rusts coherence rules,
/*
impl<'p, N, M: OpenMode + 'p> NamedNode<M> for N
where
    N: LabeledNode + OnlyChildNode<'p, M>,
{
    fn name(&self) -> CgnsResult<String> {
        Ok(Self::NODE_LABEL.as_str().into())
    }
}
*/
