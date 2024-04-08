pub(crate) trait StorageSelectorTrait: Default {}
pub(crate) trait DirectedSelectorTrait: Default {}
pub(crate) trait ContainerGenTrait<ValueType>
{
    type Type: Default;
    fn iter<'a>(container: &'a Self::Type) -> impl Iterator<Item = &'a ValueType>
    where
        ValueType: 'a;
    fn remove(container: &mut Self::Type, index: usize);
}

pub(crate) trait VertexTypeSelectorTrait
{
    type Type: Default;
}

pub(crate) trait ConfigTrait
{
    type VertexEdgeStorage: Default;
    type VertexType: Default;
    type EdgeType: Default;
    type VertexStorage: Default;
    type EdgeStorage: Default;
    type EdgeDirected;
    type GraphProperty;
    type VertexProperty;
    type EdgeProperty;
    type EdgeStorageSelector;
}

pub(crate) trait EdgeHelper<Config, EdgeDirectedSelector>
where
    Config: ConfigTrait<EdgeDirected = EdgeDirectedSelector>,
    EdgeDirectedSelector: DirectedSelectorTrait,
{
    fn remove_edge(&mut self, index: usize);
    // fn edges<'a>(&'a self) -> impl Iterator<Item = &'a Config::EdgeType>
    // where
    //     <Config as ConfigTrait>::EdgeType: 'a;
    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a Config::EdgeType>
    where
        <Config as ConfigTrait>::EdgeType: 'a;
}

pub(crate) trait GraphHelper<Config, EdgeDirectedSelector>: EdgeHelper<Config, EdgeDirectedSelector>
where
    Config: ConfigTrait<EdgeDirected = EdgeDirectedSelector>,
    EdgeDirectedSelector: DirectedSelectorTrait,
{
    fn remove_edge_from_vertex(&mut self, source: usize, target: usize);
    fn remove_out_edge_if(&mut self, source: usize, predicate: impl Fn(&Config::EdgeType) -> bool);
    fn remove_edge_if(&mut self, predicate: impl Fn(&Config::EdgeType) -> bool);
    fn clear_vertex(&mut self, index: usize);
    fn clear_out_edges(&mut self, index: usize);
    fn num_edges(&mut self) -> usize;
    fn add_edge(&mut self, source: usize, target: usize, property: &Config::EdgeProperty) -> Option<usize>;
    fn in_edge_list(&self, index: usize) -> &Config::VertexEdgeStorage;
    fn in_edge_list_mug(&mut self, index: usize) -> &mut Config::VertexEdgeStorage;
    fn remove_in_edge_if(&mut self, source: usize, predicate: impl Fn(&Config::EdgeType) -> bool);
}
