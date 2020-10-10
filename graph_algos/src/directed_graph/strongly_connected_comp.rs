pub fn execute() {
    use Graph_SSC;
    // Graph_SSC::Graph::new(vec![(1, 2), (1, 4), (2, 3), (2, 4), (4, 3), (4, 5)]);
    Graph_SSC::Graph::new(vec![(4, 4), (1, 2), (4, 1), (2, 3), (3, 1)]);
    // Graph_SSC::Graph::new(vec![
    //     (5, 7),
    //     (2, 1),
    //     (3, 2),
    //     (3, 1),
    //     (4, 3),
    //     (4, 1),
    //     (5, 2),
    //     (5, 3),
    // ]);
    // Graph_SSC::Graph::new(vec![(5, 0), (1, 0), (0, 2), (2, 1), (0, 3), (3, 4)]);
    // Graph_SSC::Graph::new(vec![(1, 0), (0, 2), (2, 1), (0, 3), (3, 4)]);
}

mod Graph_SSC {
    use std::collections::HashMap;

    pub struct Graph {
        graph: Option<HashMap<u8, Vec<u8>>>,
    }

    #[derive(Debug)]
    pub struct TimeDetail {
        start_time: u8,
        end_time: u8,
    }

    impl Graph {
        pub fn new(graph_info: Vec<(u8, u8)>) -> u8 {
            let mut adjacency_map: HashMap<u8, Vec<u8>> = HashMap::new();
            let graph = graph_info.get(1..);
            for value in graph.unwrap() {
                let source_vertex = &mut adjacency_map.entry(value.0).or_insert(vec![]);
                source_vertex.push(value.1);
                // let reversed_source_vertex = &mut adjacency_map.entry(value.1).or_insert(vec![]);
                // reversed_source_vertex.push(value.0);
            }
            let the_graph = Graph {
                graph: Some(adjacency_map),
            };
            return the_graph.get_time();
        }

        pub fn get_time(&self) -> u8 {
            let source_nodes = self.graph.as_ref().unwrap().keys();
            let mut stack: Vec<u8> = vec![];
            // let mut visited: HashMap<u8, bool> = HashMap::new();
            let mut visited: Vec<u8> = vec![];
            let mut time_visited: HashMap<u8, TimeDetail> = HashMap::new();
            let mut time_total = 0;

            println!("THE GRAPH {:?}", &self.graph);

            for vertex in source_nodes {
                self.get_sec(
                    vertex,
                    &mut stack,
                    &mut visited,
                    &mut time_visited,
                    &mut time_total,
                );
            }
            // println!("time for everyone {:#?}", time_visited);

            let mut scc_store: HashMap<u8, bool> = HashMap::new();
            let mut scc_visited: Vec<u8> = vec![];
            let mut total_scc = 0;
            stack.reverse();

            loop {
                let v = stack.pop().unwrap();

                if !scc_visited.contains(&v) {
                    let result = self.dfs(v, &mut scc_store, &mut scc_visited);

                    if result == true {
                        total_scc += 1;
                    }
                }

                if stack.len() == 0 {
                    break;
                }
            }

            return total_scc;
        }

        pub fn dfs(
            &self,
            stack_val: u8,
            scc_store: &mut HashMap<u8, bool>,
            scc_visited: &mut Vec<u8>,
        ) -> bool {
            scc_visited.push(stack_val);

            let connected_neighbours = self.graph.as_ref().unwrap().get(&stack_val);

            if connected_neighbours != None {
                for value in connected_neighbours.unwrap() {
                    // if scc_store.get(&stack_val) == None {
                    if !scc_visited.contains(value) {
                        self.dfs(*value, scc_store, scc_visited);
                    }
                }
            }
            return true;
        }

        pub fn get_sec(
            &self,
            vertex: &u8,
            stack: &mut Vec<u8>,
            visited: &mut Vec<u8>,
            time_visited: &mut HashMap<u8, TimeDetail>,
            time_total: &mut u8,
        ) {
            let connected_neighbours = self.graph.as_ref().unwrap().get(vertex);

            if !visited.contains(vertex) {
                let neighbour = &mut time_visited.entry(*vertex).or_insert(TimeDetail {
                    start_time: 0,
                    end_time: 0,
                });
                *time_total += 1;
                neighbour.start_time = *time_total;

                visited.push(*vertex);
            }

            if connected_neighbours != None {
                for value in connected_neighbours.unwrap() {
                    if !visited.contains(value) {
                        self.get_sec(value, stack, visited, time_visited, time_total);
                    }
                }
            }

            if !stack.contains(vertex) {
                let mut add_to_stack = true;
                if connected_neighbours != None {
                    for value in connected_neighbours.unwrap() {
                        if !visited.contains(value) {
                            add_to_stack = false;
                        }
                    }
                }
                if add_to_stack {
                    stack.push(*vertex);
                }
            }

            let current_vertex = time_visited.get_mut(vertex);
            if current_vertex.as_ref().unwrap().end_time == 0 {
                *time_total += 1;
                current_vertex.unwrap().end_time = *time_total;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Graph_SSC;

    #[test]
    fn strongly_connected_components() {
        let result = Graph_SSC::Graph::new(vec![
            (5, 7),
            (2, 1),
            (3, 2),
            (3, 1),
            (4, 3),
            (4, 1),
            (5, 2),
            (5, 3),
        ]);

        let result = Graph_SSC::Graph::new(vec![(4, 4), (1, 2), (4, 1), (2, 3), (3, 1)]);

        assert_eq!(result, 2);

        // assert_eq!()
    }
}
