pub fn execute() {
    use Graphs;

    // Graphs::DAG::new(vec![(4, 3), (1, 2), (4, 1), (3, 1)]);
    Graphs::DAG::new(vec![(1, 2), (1, 4), (2, 3), (2, 4), (4, 3), (4, 5)]);
}

mod Graphs {
    use std::collections::HashMap;

    pub struct DAG {
        graph: Option<HashMap<u8, Vec<u8>>>,
    }

    impl DAG {
        pub fn new(graph_info: Vec<(u8, u8)>) {
            // DirectedGraph { graph: None }
            let mut adjacency_list: HashMap<u8, Vec<u8>> = HashMap::new();
            let graph = graph_info.get(1..);
            for value in graph.unwrap() {
                let source_vertex = &mut adjacency_list.entry(value.0).or_insert(vec![]);
                source_vertex.push(value.1);
            }
            let the_graph = DAG {
                graph: Some(adjacency_list),
            };
            the_graph.get_topological_order();
        }

        pub fn get_topological_order(&self) {
            let source_nodes = self.graph.as_ref().unwrap().keys();
            // let the_pushed: HashMap<u8, &str> = HashMap::new();
            let mut stack: Vec<u8> = vec![];
            let mut visited: HashMap<u8, bool> = HashMap::new();

            // println!("THE STACK!! {:?}", stack);
            println!("\n\n THE GRAPH ITSELF!! {:?} \n\n", self.graph);
            for node in source_nodes {
                if visited.get(node) == None {
                    println!("######### HERE AGAIN #########");
                    self.get_order(node, &mut stack, &mut visited);
                }
                // if !stack.contains(node) {you You The
                //     stack.push(*node);
                // }
            }
            stack.reverse();
            println!("THE STACK!! {:?}", stack);
        }

        pub fn get_order(&self, node: &u8, stack: &mut Vec<u8>, visited: &mut HashMap<u8, bool>) {
            let receiving_nodes = self.graph.as_ref().unwrap().get(node);
            // println!("VISITED, {:?}", visited);
            println!("the node {:?}", node);
            // println!("STACK!!, {:?}", stack);
            if visited.get(node) == None {
                visited.insert(*node, true);
                if receiving_nodes != None {
                    for value in receiving_nodes.unwrap() {
                        self.get_order(value, stack, visited);
                    }
                }
                if !stack.contains(node) {
                    // if receiving_nodes == None {
                    if *node == 2 as u8 {
                        println!("YES FOUR")
                    }
                    stack.push(*node);
                    println!("the receiver {:?}", receiving_nodes);
                    // } else {
                    //     // if !receiving_nodes.unwrap().iter().any(|&i| stack.contains(&i)) {
                    //     let mut checker = true;
                    //     for key in receiving_nodes.unwrap() {
                    //         if visited.get(node) == None {
                    //             checker = false;
                    //         }
                    //     }
                    //     if (checker) {
                    //         stack.push(*node);
                    //     }
                    //     // let names = !receiving_nodes.unwrap().iter().any(|&i| visited.keys().cloned().collect().contains(&i));
                    //     // stack.push(*node);
                    //     // }
                    //     // vec![].iter().any(|x: &str| x == "i");
                    // }
                }
                println!("TOOOORRDDDDD   STACK!!, {:?}", stack);
                // println!("the node {:?}", node);
            }
        }
    }
}
