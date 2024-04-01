use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::HashMap, marker::PhantomData, ops::{Deref, DerefMut}, rc::Rc};

pub struct VecS{}
pub struct MapS{}

pub struct Bidirectional{}
pub struct Directed{}
pub struct Undirected{}

#[derive(Default)]
pub struct Edge<VertexType, EdgeType, EdgeProperty> where VertexType: Default, EdgeType:Default, EdgeProperty:Default
{
    source: VertexType,
    target: VertexType,
    property: EdgeProperty,
    edge_type: PhantomData<EdgeType>,
}

impl<VertexType, EdgeType, EdgeProperty> Edge<VertexType, EdgeType, EdgeProperty> where VertexType:Default, EdgeType:Default, EdgeProperty:Default
{
    pub fn new(source: VertexType, target: VertexType) -> Self
    {
        Self::with_property(source, target, EdgeProperty::default())
    }

    pub fn with_property(source: VertexType, target: VertexType, property:EdgeProperty) -> Self
    {
        Self { source, target, property, edge_type:PhantomData }
    }

    pub fn get_property(&self) -> &EdgeProperty
    {
        &self.property
    }

    pub fn set_property(&mut self, p: EdgeProperty)
    {
        self.property = p;
    }

    pub fn get_mut_property(&mut self) -> &mut EdgeProperty
    {
        &mut self.property
    }

    pub fn source(&self) -> &VertexType
    {
        &self.source
    }

    pub fn target(&self) -> &VertexType
    {
        &self.target
    }
}

pub struct StoredEdge<VertexType> where VertexType: Default
{
    target: VertexType,
    location: usize,
}

#[derive(Default)]
pub struct RandVertex<VertexProperty> where VertexProperty: Default
{
    out_edges: Vec<StoredEdge<Self>>,
    property: VertexProperty,
}

#[derive(Default)]
pub struct StableVertex<VertexProperty> where VertexProperty: Default
{
    out_edges: HashMap<usize, StoredEdge<Self>>,
    property: VertexProperty,
}

impl<VertexProperty> RandVertex<VertexProperty> where VertexProperty: Default
{
    pub fn new(property: VertexProperty) -> Self
    {
        Self { out_edges: vec![], property }
    }
}

impl<VertexProperty> StableVertex<VertexProperty> where VertexProperty: Default
{
    pub fn new(property: VertexProperty) -> Self
    {
        Self { out_edges: HashMap::new(), property }
    }
}

