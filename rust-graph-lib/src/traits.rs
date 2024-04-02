pub trait StorageSelectorTrait: Default {}
pub trait DirectedSelectorTrait: Default {}
pub trait ContainerGenTrait
{
    type Type: Default;
}

pub trait VertexTypeSelectorTrait
{
    type Type: Default;
}

pub trait ConfigTrait
{
    type VertexEdgeStorage: Default;
    type VertexType: Default;
    type EdgeType: Default;
    type VertexStorage: Default;
    type EdgeStorage: Default;
    type EdgeDirected;
}
