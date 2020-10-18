pub fn execute() {
    use Graphs;

    // Graphs::DAG::new(vec![(4, 3), (1, 2), (4, 1), (3, 1)]);
    // Graphs::DAG::new(vec![(1, 2), (1, 4), (2, 3), (2, 4), (4, 3), (4, 5)]);
    let result = Graphs::DAG::new(vec![(0, 2), (1, 2), (2, 3), (3, 4)]);

    println!("THE RESULT >>>  {:?}", result);
}

mod Graphs {
    use std::collections::HashMap;

    pub struct DAG {
        graph: Option<HashMap<u8, Vec<u8>>>,
    }

    impl DAG {
        pub fn new(graph_info: Vec<(u8, u8)>) -> Vec<u8> {
            // DirectedGraph { graph: None }
            let mut adjacency_list: HashMap<u8, Vec<u8>> = HashMap::new();
            let graph = graph_info.get(0..);
            for value in graph.unwrap() {
                let source_vertex = &mut adjacency_list.entry(value.0).or_insert(vec![]);
                source_vertex.push(value.1);
            }
            let the_graph = DAG {
                graph: Some(adjacency_list),
            };
            return the_graph.get_topological_order();
        }

        pub fn get_topological_order(&self) -> Vec<u8> {
            let source_nodes = self.graph.as_ref().unwrap().keys();
            let mut stack: Vec<u8> = vec![];
            // let mut visited: HashMap<u8, bool> = HashMap::new();

            for node in source_nodes {
                // if visited.get(node) == None {
                self.get_order(node, &mut stack);
                // }
            }
            stack.reverse();
            println!("THE STACK!! {:?}", stack);
            return stack;
        }

        pub fn get_order(&self, node: &u8, stack: &mut Vec<u8>) {
            let receiving_nodes = self.graph.as_ref().unwrap().get(node);
            // if visited.get(node) == None {
            if receiving_nodes != None {
                for value in receiving_nodes.unwrap() {
                    self.get_order(value, stack);
                }
            }
            if !stack.contains(node) {
                stack.push(*node);
            }
            // }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Graphs;

    #[test]
    fn test_topological_order() {
        let mut result = Graphs::DAG::new(vec![(0, 2), (1, 2), (2, 3), (3, 4)]);
        assert_eq!(result.pop(), Some(4));
        assert_eq!(result.pop(), Some(3));
        assert_eq!(result.pop(), Some(2));
    }
}
