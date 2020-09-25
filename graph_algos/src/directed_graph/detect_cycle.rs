pub fn execute() {
    use DG;

    // DG::DirectedGraph::new(vec![(4, 4), (1, 2), (2, 3), (3, 1), (4, 1)]);
    DG::DirectedGraph::new(vec![
        (5, 7),
        (1, 2),
        (2, 3),
        (1, 3),
        (3, 4),
        (1, 4),
        (2, 5),
        (3, 5),
    ]);
    // values.detect_cycle();
}

mod DG {
    use std::collections::HashMap;

    pub type AjType = HashMap<u8, Vec<u8>>;

    #[derive(Clone)]
    pub struct DirectedGraph {
        graph: Option<AjType>,
    }
    impl DirectedGraph {
        pub fn new(graph_info: Vec<(u8, u8)>) {
            // DirectedGraph { graph: None }
            let mut adjacency_list: AjType = HashMap::new();
            let graph = graph_info.get(1..);

            for value in graph.unwrap() {
                let source_vertex = &mut adjacency_list.entry(value.0).or_insert(vec![]);
                source_vertex.push(value.1);
            }

            let the_graph = DirectedGraph {
                graph: Some(adjacency_list),
            };

            the_graph.detect_cycle();
        }

        pub fn detect_cycle(&self) -> () {
            let source_nodes = self.graph.as_ref().unwrap().keys();

            for node in source_nodes {
                let mut visited: HashMap<u8, bool> = HashMap::new();
                let mut rec_stack: HashMap<u8, bool> = HashMap::new();

                let check_cycle = self.detect_cycle_util(*node, &mut visited, &mut rec_stack);

                if check_cycle {
                    println!("THERE IS A CYCLE");
                } else if !check_cycle {
                    println!("THERE IS NO CYCLE");
                }
            }
        }

        pub fn detect_cycle_util(
            &self,
            vertex: u8,
            visited: &mut HashMap<u8, bool>,
            rec_stack: &mut HashMap<u8, bool>,
        ) -> bool {
            if visited.get(&vertex) == None {
                visited.insert(vertex, true);
                rec_stack.insert(vertex, true);

                let vertex_neighbours = self.graph.as_ref().unwrap().get(&vertex);

                if vertex_neighbours == None {
                    return false;
                } else {
                    for node in vertex_neighbours.unwrap() {
                        if (visited.get(&node) == None)
                            && self.detect_cycle_util(*node, visited, rec_stack)
                        {
                            return true;
                        } else if rec_stack.get(&node) == None {
                            return true;
                        }
                    }
                }
            }
            rec_stack.insert(vertex, false);
            return false;
        }
    }
}
