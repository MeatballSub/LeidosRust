mod traits;

use std::collections::HashMap;
use std::marker::PhantomData;

use traits::ConfigTrait;
use traits::ContainerGenTrait;
use traits::DirectedSelectorTrait;
use traits::StorageSelectorTrait;
use traits::VertexTypeSelectorTrait;

#[derive(Default)]
pub struct VecS {}
#[derive(Default)]
pub struct MapS {}

impl StorageSelectorTrait for VecS {}
impl StorageSelectorTrait for MapS {}

#[derive(Default)]
pub struct Bidirectional {}
#[derive(Default)]
pub struct Directed {}
#[derive(Default)]
pub struct Undirected {}

impl DirectedSelectorTrait for Bidirectional {}
impl DirectedSelectorTrait for Directed {}
impl DirectedSelectorTrait for Undirected {}

pub struct ContainerGen<Selector, ValueType>
where
    Selector: StorageSelectorTrait,
{
    selector: PhantomData<Selector>,
    value_type: PhantomData<ValueType>,
}

impl<ValueType> ContainerGenTrait for ContainerGen<VecS, ValueType>
{
    type Type = Vec<ValueType>;
}

impl<ValueType> ContainerGenTrait for ContainerGen<MapS, ValueType>
{
    type Type = HashMap<usize, ValueType>;
}

#[derive(Default)]
pub struct Edge<EdgeDirectedSelector, VertexType, EdgeProperty>
where
    EdgeDirectedSelector: DirectedSelectorTrait,
    VertexType: Default,
    EdgeProperty: Default,
{
    source: VertexType,
    target: VertexType,
    property: EdgeProperty,
    edge_type: PhantomData<EdgeDirectedSelector>,
}

impl<EdgeDirectedSelector, VertexType, EdgeProperty> Edge<EdgeDirectedSelector, VertexType, EdgeProperty>
where
    EdgeDirectedSelector: DirectedSelectorTrait,
    VertexType: Default,
    EdgeProperty: Default,
{
    pub fn new(source: VertexType, target: VertexType) -> Self { Self::with_property(source, target, EdgeProperty::default()) }

    pub fn with_property(source: VertexType, target: VertexType, property: EdgeProperty) -> Self
    {
        Self {
            source,
            target,
            property,
            edge_type: PhantomData,
        }
    }

    pub fn get_property(&self) -> &EdgeProperty { &self.property }

    pub fn set_property(&mut self, p: EdgeProperty) { self.property = p; }

    pub fn get_mut_property(&mut self) -> &mut EdgeProperty { &mut self.property }

    pub fn source(&self) -> &VertexType { &self.source }

    pub fn target(&self) -> &VertexType { &self.target }
}

#[derive(Default)]
pub struct RandVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexProperty: Default,
{
    out_edges: VertexEdgeStorageType,
    property: VertexProperty,
}

#[derive(Default)]
pub struct StableVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexProperty: Default,
{
    out_edges: VertexEdgeStorageType,
    property: VertexProperty,
    position: usize,
}

#[derive(Default)]
pub struct BidirectionalRandVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexProperty: Default,
{
    in_edges: VertexEdgeStorageType,
    out_edges: VertexEdgeStorageType,
    property: VertexProperty,
}

#[derive(Default)]
pub struct BidirectionalStableVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexProperty: Default,
{
    in_edges: VertexEdgeStorageType,
    out_edges: VertexEdgeStorageType,
    property: VertexProperty,
    position: usize,
}

impl<VertexEdgeStorageType, VertexProperty> RandVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexEdgeStorageType: Default,
    VertexProperty: Default,
{
    pub fn new(property: VertexProperty) -> Self
    {
        Self {
            out_edges: VertexEdgeStorageType::default(),
            property,
        }
    }
}

impl<VertexEdgeStorageType, VertexProperty> StableVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexEdgeStorageType: Default,
    VertexProperty: Default,
{
    pub fn new(property: VertexProperty) -> Self
    {
        Self {
            out_edges: VertexEdgeStorageType::default(),
            property,
            position: usize::MAX,
        }
    }
}

impl<VertexEdgeStorageType, VertexProperty> BidirectionalRandVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexEdgeStorageType: Default,
    VertexProperty: Default,
{
    pub fn new(property: VertexProperty) -> Self
    {
        Self {
            in_edges: VertexEdgeStorageType::default(),
            out_edges: VertexEdgeStorageType::default(),
            property,
        }
    }
}

impl<VertexEdgeStorageType, VertexProperty> BidirectionalStableVertex<VertexEdgeStorageType, VertexProperty>
where
    VertexEdgeStorageType: Default,
    VertexProperty: Default,
{
    pub fn new(property: VertexProperty) -> Self
    {
        Self {
            in_edges: VertexEdgeStorageType::default(),
            out_edges: VertexEdgeStorageType::default(),
            property,
            position: usize::MAX,
        }
    }
}

pub struct VertexTypeSelector<VertexEdgeStorageType, VertexStorageSelector, VertexProperty>
where
    VertexStorageSelector: StorageSelectorTrait,
    VertexProperty: Default,
{
    out_edge_storage_type: PhantomData<VertexEdgeStorageType>,
    vertex_storage_selector: PhantomData<VertexStorageSelector>,
    vertex_property: PhantomData<VertexProperty>,
}

impl<VertexEdgeStorageType, VertexProperty> VertexTypeSelectorTrait for VertexTypeSelector<VertexEdgeStorageType, VecS, VertexProperty>
where
    VertexEdgeStorageType: Default,
    VertexProperty: Default,
{
    type Type = RandVertex<VertexEdgeStorageType, VertexProperty>;
}

impl<VertexEdgeStorageType, VertexProperty> VertexTypeSelectorTrait for VertexTypeSelector<VertexEdgeStorageType, MapS, VertexProperty>
where
    VertexEdgeStorageType: Default,
    VertexProperty: Default,
{
    type Type = StableVertex<VertexEdgeStorageType, VertexProperty>;
}

pub struct Config<
    VertexStorageSelector,
    VertexEdgeStorageType,
    EdgeDirectedSelector,
    EdgeStorageSelector,
    GraphProperty,
    VertexProperty,
    EdgeProperty,
> {
    vertex_storage_selector: PhantomData<VertexStorageSelector>,
    vertex_edge_storage_selector: PhantomData<VertexEdgeStorageType>,
    edge_directed_selector: PhantomData<EdgeDirectedSelector>,
    edge_storage_selector: PhantomData<EdgeStorageSelector>,
    graph_property: PhantomData<GraphProperty>,
    vertex_property: PhantomData<VertexProperty>,
    edge_property: PhantomData<EdgeProperty>,
}

impl<
        VertexStorageSelector,
        VertexEdgeStorageType,
        EdgeDirectedSelector,
        EdgeStorageSelector,
        GraphProperty,
        VertexProperty,
        EdgeProperty,
    > ConfigTrait
    for Config<
        VertexStorageSelector,
        VertexEdgeStorageType,
        EdgeDirectedSelector,
        EdgeStorageSelector,
        GraphProperty,
        VertexProperty,
        EdgeProperty,
    >
where
    VertexStorageSelector: StorageSelectorTrait,
    VertexEdgeStorageType: StorageSelectorTrait,
    EdgeDirectedSelector: DirectedSelectorTrait,
    EdgeStorageSelector: StorageSelectorTrait,
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default,
    ContainerGen<VertexEdgeStorageType, usize>: ContainerGenTrait,
    ContainerGen<
        VertexStorageSelector,
        <VertexTypeSelector<
            <ContainerGen<VertexEdgeStorageType, usize> as ContainerGenTrait>::Type,
            VertexStorageSelector,
            VertexProperty,
        > as VertexTypeSelectorTrait>::Type,
    >: ContainerGenTrait,
    ContainerGen<
        EdgeStorageSelector,
        Edge<
            EdgeDirectedSelector,
            <VertexTypeSelector<
                <ContainerGen<VertexEdgeStorageType, usize> as ContainerGenTrait>::Type,
                VertexStorageSelector,
                VertexProperty,
            > as VertexTypeSelectorTrait>::Type,
            EdgeProperty,
        >,
    >: ContainerGenTrait,
    VertexTypeSelector<
        <ContainerGen<VertexEdgeStorageType, usize> as ContainerGenTrait>::Type,
        VertexStorageSelector,
        VertexProperty,
    >: VertexTypeSelectorTrait,
{
    type VertexEdgeStorage =
        <ContainerGen<VertexEdgeStorageType, usize> as ContainerGenTrait>::Type;
    type VertexType = <VertexTypeSelector<
        Self::VertexEdgeStorage,
        VertexStorageSelector,
        VertexProperty,
    > as VertexTypeSelectorTrait>::Type;
    type EdgeType = Edge<EdgeDirectedSelector, Self::VertexType, EdgeProperty>;
    type VertexStorage =
        <ContainerGen<VertexStorageSelector, Self::VertexType> as ContainerGenTrait>::Type;
    type EdgeStorage =
        <ContainerGen<EdgeStorageSelector, Self::EdgeType> as ContainerGenTrait>::Type;
    type EdgeDirected = EdgeDirectedSelector;
}
