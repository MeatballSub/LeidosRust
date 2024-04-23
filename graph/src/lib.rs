use std::cmp;
use std::collections::HashMap;
// use std::iter;
// use std::iter::Filter;
//use std::default;
use std::marker::PhantomData;

#[derive(Clone)]
struct StoredEdge
{
    target: usize,
    index: usize,
}

#[derive(Default, Clone)]
struct RandUndirectedVecAdjListVertex<VertexProperty>
where VertexProperty: Default
{
    out_edges: Vec<StoredEdge>,
    property: VertexProperty,
}

#[derive(Default, Clone)]
struct RandUndirectedVecAdjListEdge<EdgeProperty>
where EdgeProperty: Default + PartialEq
{
    source: usize,
    target: usize,
    property: EdgeProperty,
}

pub struct UndirectedVecAdjList<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    vertex_property: PhantomData<VertexProperty>,
    edge_property: PhantomData<EdgeProperty>,
    graph_property: GraphProperty,
    vertices: Vec<RandUndirectedVecAdjListVertex<VertexProperty>>,
    edges: Vec<RandUndirectedVecAdjListEdge<EdgeProperty>>,
}

pub struct UndirectedVecAdjListBuilder<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    vertex_property: PhantomData<VertexProperty>,
    edge_property: PhantomData<EdgeProperty>,
    graph_property: GraphProperty,
    num_vertices: usize,
    edges: Vec<RandUndirectedVecAdjListEdge<EdgeProperty>>,
}

impl<GraphProperty, VertexProperty, EdgeProperty>
    UndirectedVecAdjListBuilder<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    pub fn default() -> Self
    {
        Self {
            vertex_property: PhantomData,
            edge_property: PhantomData,
            graph_property: GraphProperty::default(),
            num_vertices: 0,
            edges: Default::default(),
        }
    }

    pub fn property(&mut self, graph_property: GraphProperty) -> &mut Self
    {
        self.graph_property = graph_property;
        self
    }

    pub fn num_vertices(&mut self, num_vertices: usize) -> &mut Self
    {
        self.num_vertices = num_vertices;
        self
    }

    pub fn edges(&mut self, edges: impl Iterator<Item = (usize, usize)>) -> &mut Self
    {
        for edge in edges
        {
            self.num_vertices = cmp::max(self.num_vertices, cmp::max(edge.0, edge.1));
            self.edges.push(RandUndirectedVecAdjListEdge {
                source: edge.0,
                target: edge.1,
                property: Default::default(),
            });
        }
        self
    }

    pub fn edges_with_properties(
        &mut self, edges: impl Iterator<Item = (usize, usize)>,
        properties: impl Iterator<Item = EdgeProperty>,
    ) -> &mut Self
    {
        for (edge, property) in edges.zip(properties)
        {
            self.num_vertices = cmp::max(self.num_vertices, cmp::max(edge.0, edge.1));
            self.edges.push(RandUndirectedVecAdjListEdge {
                source: edge.0,
                target: edge.1,
                property: property,
            });
        }
        self
    }

    pub fn build(self) -> UndirectedVecAdjList<GraphProperty, VertexProperty, EdgeProperty>
    {
        let mut new_vertices: Vec<RandUndirectedVecAdjListVertex<VertexProperty>> = Vec::new();
        new_vertices.resize_with(self.num_vertices, Default::default);
        UndirectedVecAdjList {
            graph_property: self.graph_property,
            vertex_property: PhantomData,
            edge_property: PhantomData,
            vertices: new_vertices,
            edges: self.edges,
        }
    }
}

impl<GraphProperty, VertexProperty, EdgeProperty>
    UndirectedVecAdjList<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    pub fn vertices(&self) -> impl Iterator + '_ { self.vertices.iter() }

    pub fn edges(&self) -> impl Iterator + '_ { self.edges.iter() }

    pub fn adjacent_vertices(&self, vertex_index: usize) -> impl Iterator + '_
    {
        self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.target)
    }

    pub fn inv_adjacent_vertices(&self, vertex_index: usize) -> impl Iterator + '_
    {
        self.adjacent_vertices(vertex_index)
    }

    pub fn out_edges(&self, vertex_index: usize) -> impl Iterator<Item = usize> + '_
    {
        self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.index)
    }

    pub fn in_edges(&self, vertex_index: usize) -> impl Iterator<Item = usize> + '_
    {
        self.out_edges(vertex_index)
    }

    pub fn source(&self, edge_index: usize) -> usize { self.edges[edge_index].source }

    pub fn target(&self, edge_index: usize) -> usize { self.edges[edge_index].target }

    pub fn out_degree(&self, vertex_index: usize) -> usize { self.degree(vertex_index) }

    pub fn in_degree(&self, vertex_index: usize) -> usize { self.degree(vertex_index) }

    pub fn degree(&self, vertex_index: usize) -> usize
    {
        self.vertices[vertex_index].out_edges.len()
    }

    pub fn num_vertices(&self) -> usize { self.vertices.len() }

    pub fn num_edges(&self) -> usize { self.edges.len() }

    pub fn get_edge(&self, source: usize, target: usize) -> Option<usize>
    {
        self.get_edges(source, target).next()
    }

    pub fn get_edges(&self, source: usize, target: usize) -> impl Iterator<Item = usize> + '_
    {
        self.vertices[source]
            .out_edges
            .iter()
            .filter(move |e| e.target == target)
            .map(|e| e.index)
    }

    pub fn add_edge(&mut self, source: usize, target: usize) -> Option<usize>
    {
        self.add_edge_with_property(source, target, Default::default())
    }

    pub fn add_edge_with_property(
        &mut self, source: usize, target: usize, edge_property: EdgeProperty,
    ) -> Option<usize>
    {
        let max_vertex_index = std::cmp::max(source, target);
        while self.vertices.len() <= max_vertex_index
        {
            self.add_vertex();
        }

        let edge = RandUndirectedVecAdjListEdge {
            source,
            target,
            property: edge_property,
        };

        let edge_index = self.edges.len();

        self.edges.push(edge);
        self.vertices[source].out_edges.push(StoredEdge {
            target,
            index: edge_index,
        });
        self.vertices[target].out_edges.push(StoredEdge {
            target: source,
            index: edge_index,
        });

        Some(edge_index)
    }

    fn remove_edge_helper(
        &mut self, remove_index: usize, edge: RandUndirectedVecAdjListEdge<EdgeProperty>,
    )
    {
        if let Some(source_remove_index) = self.vertices[edge.source]
            .out_edges
            .iter()
            .position(|e| e.index == remove_index)
        {
            self.vertices[edge.source]
                .out_edges
                .swap_remove(source_remove_index);
        }

        if let Some(target_remove_index) = self.vertices[edge.target]
            .out_edges
            .iter()
            .position(|e| e.index == remove_index)
        {
            self.vertices[edge.target]
                .out_edges
                .swap_remove(target_remove_index);
        }

        if let Some(swapped_edge) = self.edges.get(remove_index)
        {
            if let Some(source_swapped_index) = self.vertices[swapped_edge.source]
                .out_edges
                .iter()
                .position(|e| e.index == remove_index)
            {
                self.vertices[swapped_edge.source].out_edges[source_swapped_index].index =
                    remove_index;
            }

            if let Some(target_swapped_index) = self.vertices[swapped_edge.target]
                .out_edges
                .iter()
                .position(|e| e.index == remove_index)
            {
                self.vertices[swapped_edge.target].out_edges[target_swapped_index].index =
                    remove_index;
            }
        }
    }

    pub fn remove_edge_at(&mut self, remove_index: usize)
    {
        if remove_index < self.edges.len()
        {
            let edge = self.edges.swap_remove(remove_index);
            self.remove_edge_helper(remove_index, edge);
        }
    }

    pub fn remove_edge(&mut self, source: usize, target: usize)
    {
        self.get_edges(source, target)
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|i| self.remove_edge_at(*i));
    }

    pub fn remove_out_edge_if(&mut self, vertex_index: usize, predicate: impl Fn(&usize) -> bool)
    {
        self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.index)
            .filter(predicate)
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|i| self.remove_edge_at(*i));
    }

    pub fn remove_in_edge_if(&mut self, vertex_index: usize, predicate: impl Fn(&usize) -> bool)
    {
        self.remove_out_edge_if(vertex_index, predicate);
    }

    pub fn remove_edge_if(&mut self, predicate: impl Fn(&usize) -> bool)
    {
        for index in 0..self.edges.len()
        {
            if predicate(&index)
            {
                self.remove_edge_at(index);
            }
        }
    }

    pub fn add_vertex(&mut self) -> usize { self.add_vertex_with_property(Default::default()) }

    pub fn add_vertex_with_property(&mut self, vertex_property: VertexProperty) -> usize
    {
        self.vertices.push(RandUndirectedVecAdjListVertex {
            out_edges: Default::default(),
            property: vertex_property,
        });
        self.vertices.len() - 1
    }

    pub fn clear_vertex(&mut self, vertex_index: usize)
    {
        self.out_edges(vertex_index)
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|i| self.remove_edge_at(*i));
    }

    pub fn clear_out_edges(&mut self, vertex_index: usize) { self.clear_vertex(vertex_index); }

    pub fn clear_in_edges(&mut self, vertex_index: usize) { self.clear_vertex(vertex_index); }

    pub fn remove_vertex(&mut self, vertex_index: usize)
    {
        let descending = |a: &usize, b: &usize| b.cmp(a);
        let mut edges_to_remove = self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.index)
            .collect::<Vec<usize>>();
        edges_to_remove.sort_by(descending);

        let mut swap_map = HashMap::new();
        let mut swap_map_rev = HashMap::new();
        let mut swap_index = self.edges.len();
        for index in edges_to_remove
        {
            swap_index -= 1;
            if index != swap_index
            {
                let redirect_index = swap_map_rev.remove(&swap_index).unwrap_or(swap_index);
                swap_map.insert(redirect_index, index);
                swap_map_rev.insert(index, redirect_index);
            }
            self.edges.swap_remove(index);
        }

        self.vertices.swap_remove(vertex_index);

        for vertex in self.vertices.iter_mut()
        {
            for edge in vertex.out_edges.iter_mut()
            {
                if let Some(swap_value) = swap_map.get(&edge.index)
                {
                    edge.index = *swap_value;
                }
            }
        }
    }

    pub fn get_vertex_properties(&self, vertex_index: usize) -> &VertexProperty
    {
        &self.vertices[vertex_index].property
    }

    pub fn set_vertex_properties(&mut self, vertex_index: usize, vertex_properties: VertexProperty)
    {
        self.vertices[vertex_index].property = vertex_properties;
    }

    pub fn get_edge_properties(&self, edge_index: usize) -> &EdgeProperty
    {
        &self.edges[edge_index].property
    }

    pub fn set_edge_properties(&mut self, edge_index: usize, edge_properties: EdgeProperty)
    {
        self.edges[edge_index].property = edge_properties;
    }

    pub fn get_graph_properties(&self) -> &GraphProperty { &self.graph_property }

    pub fn set_graph_properties(&mut self, graph_property: GraphProperty)
    {
        self.graph_property = graph_property;
    }

    pub fn clear(&mut self)
    {
        self.vertices.clear();
        self.edges.clear();
    }
}

#[derive(Default, Clone)]
struct RandDirectedVecAdjListVertex<VertexProperty>
where VertexProperty: Default
{
    out_edges: Vec<StoredEdge>,
    property: VertexProperty,
}

#[derive(Default, Clone)]
struct RandDirectedVecAdjListEdge<EdgeProperty>
where EdgeProperty: Default + PartialEq
{
    source: usize,
    target: usize,
    property: EdgeProperty,
}

pub struct DirectedVecAdjList<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    vertex_property: PhantomData<VertexProperty>,
    edge_property: PhantomData<EdgeProperty>,
    graph_property: GraphProperty,
    vertices: Vec<RandDirectedVecAdjListVertex<VertexProperty>>,
    edges: Vec<RandDirectedVecAdjListEdge<EdgeProperty>>,
}

pub struct DirectedVecAdjListBuilder<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    vertex_property: PhantomData<VertexProperty>,
    edge_property: PhantomData<EdgeProperty>,
    graph_property: GraphProperty,
    num_vertices: usize,
    edges: Vec<RandDirectedVecAdjListEdge<EdgeProperty>>,
}

impl<GraphProperty, VertexProperty, EdgeProperty>
    DirectedVecAdjListBuilder<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    pub fn default() -> Self
    {
        Self {
            vertex_property: PhantomData,
            edge_property: PhantomData,
            graph_property: GraphProperty::default(),
            num_vertices: 0,
            edges: Default::default(),
        }
    }

    pub fn property(&mut self, graph_property: GraphProperty) -> &mut Self
    {
        self.graph_property = graph_property;
        self
    }

    pub fn num_vertices(&mut self, num_vertices: usize) -> &mut Self
    {
        self.num_vertices = num_vertices;
        self
    }

    pub fn edges(&mut self, edges: impl Iterator<Item = (usize, usize)>) -> &mut Self
    {
        for edge in edges
        {
            self.num_vertices = cmp::max(self.num_vertices, cmp::max(edge.0, edge.1));
            self.edges.push(RandDirectedVecAdjListEdge {
                source: edge.0,
                target: edge.1,
                property: Default::default(),
            });
        }
        self
    }

    pub fn edges_with_properties(
        &mut self, edges: impl Iterator<Item = (usize, usize)>,
        properties: impl Iterator<Item = EdgeProperty>,
    ) -> &mut Self
    {
        for (edge, property) in edges.zip(properties)
        {
            self.num_vertices = cmp::max(self.num_vertices, cmp::max(edge.0, edge.1));
            self.edges.push(RandDirectedVecAdjListEdge {
                source: edge.0,
                target: edge.1,
                property: property,
            });
        }
        self
    }

    pub fn build(self) -> DirectedVecAdjList<GraphProperty, VertexProperty, EdgeProperty>
    {
        let mut new_vertices: Vec<RandDirectedVecAdjListVertex<VertexProperty>> = Vec::new();
        new_vertices.resize_with(self.num_vertices, Default::default);
        DirectedVecAdjList {
            graph_property: self.graph_property,
            vertex_property: PhantomData,
            edge_property: PhantomData,
            vertices: new_vertices,
            edges: self.edges,
        }
    }
}

impl<GraphProperty, VertexProperty, EdgeProperty>
    DirectedVecAdjList<GraphProperty, VertexProperty, EdgeProperty>
where
    GraphProperty: Default,
    VertexProperty: Default,
    EdgeProperty: Default + PartialEq,
{
    pub fn vertices(&self) -> impl Iterator + '_ { self.vertices.iter() }

    pub fn edges(&self) -> impl Iterator + '_ { self.edges.iter() }

    pub fn adjacent_vertices(&self, vertex_index: usize) -> impl Iterator + '_
    {
        self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.target)
    }

    pub fn inv_adjacent_vertices(&self, vertex_index: usize) -> impl Iterator + '_
    {
        self.edges
            .iter()
            .filter(move |e| e.target == vertex_index)
            .map(|e| e.source)
    }

    pub fn out_edges(&self, vertex_index: usize) -> impl Iterator<Item = usize> + '_
    {
        self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.index)
    }

    pub fn in_edges(&self, vertex_index: usize) -> impl Iterator<Item = usize> + '_
    {
        self.edges
            .iter()
            .enumerate()
            .filter(move |(_, e)| e.target == vertex_index)
            .map(|(i, _)| i)
    }

    pub fn source(&self, edge_index: usize) -> usize { self.edges[edge_index].source }

    pub fn target(&self, edge_index: usize) -> usize { self.edges[edge_index].target }

    pub fn out_degree(&self, vertex_index: usize) -> usize { self.out_edges(vertex_index).count() }

    pub fn in_degree(&self, vertex_index: usize) -> usize { self.in_edges(vertex_index).count() }

    pub fn degree(&self, vertex_index: usize) -> usize
    {
        self.in_degree(vertex_index) + self.out_degree(vertex_index)
    }

    pub fn num_vertices(&self) -> usize { self.vertices.len() }

    pub fn num_edges(&self) -> usize { self.edges.len() }

    pub fn get_edge(&self, source: usize, target: usize) -> Option<usize>
    {
        self.get_edges(source, target).next()
    }

    pub fn get_edges(&self, source: usize, target: usize) -> impl Iterator<Item = usize> + '_
    {
        self.vertices[source]
            .out_edges
            .iter()
            .filter(move |e| e.target == target)
            .map(|e| e.index)
    }

    pub fn add_edge(&mut self, source: usize, target: usize) -> Option<usize>
    {
        self.add_edge_with_property(source, target, Default::default())
    }

    pub fn add_edge_with_property(
        &mut self, source: usize, target: usize, edge_property: EdgeProperty,
    ) -> Option<usize>
    {
        let max_vertex_index = std::cmp::max(source, target);
        while self.vertices.len() <= max_vertex_index
        {
            self.add_vertex();
        }

        let edge = RandDirectedVecAdjListEdge {
            source,
            target,
            property: edge_property,
        };

        let edge_index = self.edges.len();

        self.edges.push(edge);
        self.vertices[source].out_edges.push(StoredEdge {
            target,
            index: edge_index,
        });

        Some(edge_index)
    }

    fn remove_edge_helper(
        &mut self, remove_index: usize, edge: RandDirectedVecAdjListEdge<EdgeProperty>,
    )
    {
        if let Some(source_remove_index) = self.vertices[edge.source]
            .out_edges
            .iter()
            .position(|e| e.index == remove_index)
        {
            self.vertices[edge.source]
                .out_edges
                .swap_remove(source_remove_index);
        }

        if let Some(swapped_edge) = self.edges.get(remove_index)
        {
            if let Some(source_swapped_index) = self.vertices[swapped_edge.source]
                .out_edges
                .iter()
                .position(|e| e.index == remove_index)
            {
                self.vertices[swapped_edge.source].out_edges[source_swapped_index].index =
                    remove_index;
            }
        }
    }

    pub fn remove_edge_at(&mut self, remove_index: usize)
    {
        if remove_index < self.edges.len()
        {
            let edge = self.edges.swap_remove(remove_index);
            self.remove_edge_helper(remove_index, edge);
        }
    }

    pub fn remove_edge(&mut self, source: usize, target: usize)
    {
        self.get_edges(source, target)
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|i| self.remove_edge_at(*i));
    }

    pub fn remove_out_edge_if(&mut self, vertex_index: usize, predicate: impl Fn(&usize) -> bool)
    {
        self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.index)
            .filter(predicate)
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|i| self.remove_edge_at(*i));
    }

    pub fn remove_in_edge_if(&mut self, vertex_index: usize, predicate: impl Fn(&usize) -> bool)
    {
        for index in 0..self.edges.len()
        {
            if self.edges[index].target == vertex_index && predicate(&index)
            {
                self.remove_edge_at(index);
            }
        }
    }

    pub fn remove_edge_if(&mut self, predicate: impl Fn(&usize) -> bool)
    {
        for index in 0..self.edges.len()
        {
            if predicate(&index)
            {
                self.remove_edge_at(index);
            }
        }
    }

    pub fn add_vertex(&mut self) -> usize { self.add_vertex_with_property(Default::default()) }

    pub fn add_vertex_with_property(&mut self, vertex_property: VertexProperty) -> usize
    {
        self.vertices.push(RandDirectedVecAdjListVertex {
            out_edges: Default::default(),
            property: vertex_property,
        });
        self.vertices.len() - 1
    }

    pub fn clear_vertex(&mut self, vertex_index: usize)
    {
        self.clear_in_edges(vertex_index);
        self.clear_out_edges(vertex_index);
    }

    pub fn clear_out_edges(&mut self, vertex_index: usize)
    {
        let always_true = |e: &'_ usize| true;
        self.remove_out_edge_if(vertex_index, always_true);
    }

    pub fn clear_in_edges(&mut self, vertex_index: usize) { self.clear_vertex(vertex_index); }

    pub fn remove_vertex(&mut self, vertex_index: usize)
    {
        let descending = |a: &usize, b: &usize| b.cmp(a);
        let mut edges_to_remove = self.vertices[vertex_index]
            .out_edges
            .iter()
            .map(|e| e.index)
            .collect::<Vec<usize>>();
        edges_to_remove.sort_by(descending);

        let mut swap_map = HashMap::new();
        let mut swap_map_rev = HashMap::new();
        let mut swap_index = self.edges.len();
        for index in edges_to_remove
        {
            swap_index -= 1;
            if index != swap_index
            {
                let redirect_index = swap_map_rev.remove(&swap_index).unwrap_or(swap_index);
                swap_map.insert(redirect_index, index);
                swap_map_rev.insert(index, redirect_index);
            }
            self.edges.swap_remove(index);
        }

        self.vertices.swap_remove(vertex_index);

        for vertex in self.vertices.iter_mut()
        {
            for edge in vertex.out_edges.iter_mut()
            {
                if let Some(swap_value) = swap_map.get(&edge.index)
                {
                    edge.index = *swap_value;
                }
            }
        }
    }

    pub fn get_vertex_properties(&self, vertex_index: usize) -> &VertexProperty
    {
        &self.vertices[vertex_index].property
    }

    pub fn set_vertex_properties(&mut self, vertex_index: usize, vertex_properties: VertexProperty)
    {
        self.vertices[vertex_index].property = vertex_properties;
    }

    pub fn get_edge_properties(&self, edge_index: usize) -> &EdgeProperty
    {
        &self.edges[edge_index].property
    }

    pub fn set_edge_properties(&mut self, edge_index: usize, edge_properties: EdgeProperty)
    {
        self.edges[edge_index].property = edge_properties;
    }

    pub fn get_graph_properties(&self) -> &GraphProperty { &self.graph_property }

    pub fn set_graph_properties(&mut self, graph_property: GraphProperty)
    {
        self.graph_property = graph_property;
    }

    pub fn clear(&mut self)
    {
        self.vertices.clear();
        self.edges.clear();
    }
}
