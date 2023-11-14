use std::num::NonZeroUsize;

pub struct Graph<N, E> {
    nodes: Vec<NodeData<N>>,
    edges: Vec<EdgeData<E>>,
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(NonZeroUsize);

impl NodeId {
    fn index(&self) -> usize {
        self.0.get() - 1
    }
}

struct NodeData<T> {
    data: T,
    first_outgoing_edge: Option<EdgeId>,
}

#[derive(Clone, Copy)]
pub struct EdgeId(NonZeroUsize);

impl EdgeId {
    fn index(&self) -> usize {
        self.0.get() - 1
    }
}

struct EdgeData<T> {
    data: T,
    target: NodeId,
    next_outgoing_edge: Option<EdgeId>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, data: N) -> NodeId {
        self.nodes.push(NodeData {
            data,
            first_outgoing_edge: None,
        });
        NodeId(NonZeroUsize::new(self.nodes.len()).unwrap())
    }

    pub fn add_edge(&mut self, source: NodeId, target: NodeId, data: E) -> EdgeId {
        let node_data = &mut self.nodes[source.index()];
        self.edges.push(EdgeData {
            data,
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        let edge_index = EdgeId(NonZeroUsize::new(self.edges.len()).unwrap());
        node_data.first_outgoing_edge = Some(edge_index);
        edge_index
    }

    pub fn get_node(&self, index: NodeId) -> Option<&N> {
        return self.nodes.get(index.index()).map(|node| &node.data);
    }

    pub fn get_edge(&self, index: EdgeId) -> Option<&E> {
        return self.edges.get(index.index()).map(|edge| &edge.data);
    }

    pub fn nodes(&self) -> NodeIterator<N, E> {
        NodeIterator {
            graph: self,
            current_node_index: 0,
        }
    }

    pub fn successors(&self, source: NodeId) -> Successors<N, E> {
        let first_outgoing_edge = self.nodes[source.index()].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_id: first_outgoing_edge,
        }
    }
}

pub struct NodeIterator<'graph, N, E> {
    graph: &'graph Graph<N, E>,
    current_node_index: usize,
}

impl<'graph, N, E> Iterator for NodeIterator<'graph, N, E> {
    type Item = (NodeId, &'graph N);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(data) = self.graph.nodes.get(self.current_node_index) {
            self.current_node_index += 1;
            return Some((
                NodeId(NonZeroUsize::new(self.current_node_index).unwrap()),
                &data.data,
            ));
        }
        None
    }
}

pub struct Successors<'graph, N, E> {
    graph: &'graph Graph<N, E>,
    current_edge_id: Option<EdgeId>,
}

impl<'graph, N, E> Iterator for Successors<'graph, N, E> {
    type Item = (&'graph E, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_id {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num.index()];
                self.current_edge_id = edge.next_outgoing_edge;
                Some((&edge.data, edge.target))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        // N0 ---E0---> N1 ---E1---> 2
        // |                         ^
        // E2                        |
        // |                         |
        // v                         |
        // N3 ----------E3-----------+

        let mut graph = Graph::new();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        let n2 = graph.add_node(3);
        let n3 = graph.add_node(4);

        graph.add_edge(n0, n1, 10); // e0
        graph.add_edge(n1, n2, 11); // e1
        graph.add_edge(n0, n3, 12); // e2
        graph.add_edge(n3, n2, 13); // e3

        let successors: Vec<_> = graph.successors(n0).collect();
        assert_eq!(&successors[..], &[(&12, n3), (&10, n1)]);
    }
}
