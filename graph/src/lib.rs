use std::cmp;
use std::collections::HashMap;
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
    pub fn get_vertex_properties(&self, index: usize) -> &VertexProperty
    {
        &self.vertices[index].property
    }

    pub fn set_vertex_properties(&mut self, index: usize, vertex_properties: VertexProperty)
    {
        self.vertices[index].property = vertex_properties;
    }

    pub fn get_edge_properties(&self, index: usize) -> &EdgeProperty { &self.edges[index].property }

    pub fn set_edge_properties(&mut self, index: usize, edge_properties: EdgeProperty)
    {
        self.edges[index].property = edge_properties;
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

    pub fn out_edges(&self, vertex: usize) -> impl Iterator<Item = usize> + '_
    {
        self.vertices[vertex].out_edges.iter().map(|e| e.index)
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

    pub fn degree(&self, vertex: usize) -> usize { self.vertices[vertex].out_edges.len() }

    pub fn in_degree(&self, vertex: usize) -> usize { self.degree(vertex) }

    pub fn in_edges(&self, vertex: usize) -> impl Iterator<Item = usize> + '_
    {
        self.out_edges(vertex)
    }

    fn edges_for_vertex(&self, vertex: usize) -> Vec<usize>
    {
        self.vertices[vertex]
            .out_edges
            .iter()
            .map(|e| e.index)
            .collect::<Vec<usize>>()
    }

    pub fn remove_vertex(&mut self, vertex_index: usize)
    {
        let mut edges_to_remove = self.edges_for_vertex(vertex_index);
        let descending = |a: &usize, b: &usize| b.cmp(a);
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

    pub fn vertices(&self) -> impl Iterator + '_ { self.vertices.iter() }

    pub fn num_vertices(&self) -> usize { self.vertices.len() }

    pub fn out_degree(&self, vertex_index: usize) -> usize { self.degree(vertex_index) }

    pub fn get_edge_range(&self, source: usize, target: usize) -> impl Iterator<Item = usize> + '_
    {
        self.vertices[source]
            .out_edges
            .iter()
            .filter(move |e| e.target == target)
            .map(|e| e.index)
    }

    // fn find_edge_index_with_property(
    //     &self, edge: &RandUndirectedVecAdjListEdge<EdgeProperty>,
    // ) -> Option<usize>
    // {
    //     let property_match = |index: &usize| *self.get_edge_properties(*index) ==
    // edge.property;     self.get_edge_range(edge.source, edge.target)
    //         .filter(property_match)
    //         .next()
    // }

    pub fn get_edge(&self, source: usize, target: usize) -> Option<usize>
    {
        self.get_edge_range(source, target).next()
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
        for edge_index in self.get_edge_range(source, target).collect::<Vec<usize>>()
        {
            self.remove_edge_at(edge_index);
        }
    }
}
