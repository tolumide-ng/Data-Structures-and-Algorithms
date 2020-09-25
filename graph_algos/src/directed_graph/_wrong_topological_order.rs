pub fn execute() {
    use Graphs;
    Graphs::DAG::new(vec![(1, 2), (1, 4), (2, 3), (2, 4), (4, 3), (4, 5)]);
}

mod Graphs {
    use std::collections::HashMap;

    pub struct DAG {
        graph: Option<HashMap<u8, Vertex>>,
    }

    #[derive(Debug, Clone)]
    pub struct Vertex {
        count: i8,
        points_to: Vec<u8>,
    }

    // self.head = mem::replace(&mut self.head.take().unwrap().borrow_mut().next, None);

    impl DAG {
        pub fn new(graph_info: Vec<(u8, u8)>) {
            let mut adjacency_list: HashMap<u8, Vertex> = HashMap::new();
            // let mut adjacency_list: Vec<Vertex> = vec![];

            for value in graph_info {
                let source_vertex = &mut adjacency_list.entry(value.0).or_insert(Vertex {
                    count: -1,
                    points_to: vec![],
                });
                source_vertex.points_to.push(value.1);
                source_vertex.count += 1;
            }

            let mut the_graph = DAG {
                graph: Some(adjacency_list),
            };
            the_graph.get_topological_sort();
        }

        pub fn get_topological_sort(&mut self) {
            
        }
    }
}
