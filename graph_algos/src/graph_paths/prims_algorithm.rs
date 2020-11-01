pub fn execute() {
    let result = prims::PrimsGraph::new(vec![
        ("a", "f", 2),
        ("a", "b", 2),
        ("a", "d", 7),
        ("f", "b", 5),
        ("b", "d", 4),
        ("f", "c", 4),
        ("b", "c", 1),
        ("b", "e", 3),
        ("c", "e", 4),
        ("d", "e", 1),
        ("e", "g", 7),
        ("d", "g", 5),
    ]);
}

mod prims {
    use crate::graph_paths::priority_queue;
    use rand::Rng;
    use std::cmp::Eq;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    pub struct PrimsVertex<T> {
        vertex: T,
        distance_from_parent: f64,
        parent: Option<T>,
        inMST: bool,
    }

    pub type GraphType<T> = HashMap<T, Vec<(T, u64)>>;

    #[derive(Clone, Debug)]
    pub struct PrimsGraph<T> {
        graph: Option<GraphType<T>>,
    }

    #[derive(Debug, Clone)]
    pub struct Priority_Struct<T> {
        distance: f64,
        vertex: T,
        priority_value: u64,
    }

    // pub type DicDistType<T> = HashMap<T, PrimsVertex<T>>;

    impl<T> PrimsGraph<T>
    where
        T: Debug + Eq + Hash + Copy + Display + Clone,
    {
        pub fn new(graph_info: Vec<(T, T, u64)>) {
            let mut adjacency_dic: HashMap<T, Vec<(T, u64)>> = HashMap::new();

            let mut dic_distance: HashMap<T, PrimsVertex<T>> = HashMap::new();

            let mut all_vertex: Vec<T> = vec![];

            for value in graph_info {
                let vertex_x = &mut adjacency_dic.entry(value.0).or_insert(vec![]);
                vertex_x.push((value.1, value.2));

                all_vertex.push(value.0);
                all_vertex.push(value.1);

                dic_distance.insert(
                    value.0,
                    PrimsVertex {
                        vertex: value.0,
                        parent: None,
                        distance_from_parent: f64::INFINITY,
                        inMST: false,
                    },
                );

                dic_distance.insert(
                    value.1,
                    PrimsVertex {
                        vertex: value.1,
                        parent: None,
                        distance_from_parent: f64::INFINITY,
                        inMST: false,
                    },
                );
            }

            let the_graph = PrimsGraph {
                graph: Some(adjacency_dic),
            };

            the_graph.get_mst(dic_distance, all_vertex);
        }

        pub fn get_mst(&self, dic_distance: HashMap<T, PrimsVertex<T>>, all_vertex: Vec<T>) {
            let mut priority_value = 0;
            // let mut mst_tree: Vec<(T, T)> = vec![];
            let mut priority_queue: Vec<Priority_Struct<T>> = vec![];
            // O(n) of another_queue would be O(n)log(n) which is the sea
            let mut another_queue: HashMap<T, Vec<Priority_Struct<T>>> = HashMap::new();
            let mut rng = rand::thread_rng();
            let n: usize = rng.gen_range(0, all_vertex.len());

            let source_vertex = all_vertex.get(n).unwrap();
            let source_vertex = dic_distance.get(source_vertex).unwrap();

            priority_queue.push(Priority_Struct {
                distance: source_vertex.distance_from_parent,
                vertex: source_vertex.vertex,
                priority_value: priority_value,
            });

            let dd = vec![dic_distance];

            priority_value += 1;

            while priority_queue.len() > 0 {
                // let current = dd
            }
        }
    }
}
