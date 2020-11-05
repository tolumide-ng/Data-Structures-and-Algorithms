pub fn execute() {}

mod kruskal {
    use std::cmp::Eq;
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    pub struct KrustGraph<T> {
        vertex: T,
        distance: u8,
        previous_vertex: T,
    }

    pub type GraphType<T> = HashMap<T, Vec<(T, u8)>>;

    pub struct KrusKGraph<T> {
        graph: Option<GraphType<T>>,
    }

    #[derive(Hash, Eq)]
    pub struct Edge<T> {
        first: T,
        second: T,
        weight: u8,
    }
    impl<T: Eq> PartialEq for Edge<T> {
        fn eq(&self, other: &Self) -> bool {
            self.first == other.first || self.second == other.second
        }
    }

    #[derive(Hash, Eq)]
    pub struct Vertex_Info<T> {
        vertex: T,
        parent: T,
    }

    impl<T: Eq> PartialEq for Vertex_Info<T> {
        fn eq(&self, other: &Self) -> bool {
            self.vertex == other.vertex || self.parent == other.parent
        }
    }

    impl<T> KrusKGraph<T>
    where
        T: Debug + Eq + Hash + Copy + Display + Clone,
    {
        pub fn new(graph_info: Vec<(T, T, u8)>) {
            let mut adjacency_dic: HashMap<T, Vec<(T, u8)>> = HashMap::new();
            // let mut visited_set: HashSet<Vertex_Info<T>> = HashSet::new();
            // let mut adjacency_set: HashSet<(T, T, u8)> = HashSet::new();
            // let mut vertex_info: HashSet<Vertex_Info<T>> = HashSet::new();
            let mut all_vertex: HashSet<Edge<T>> = HashSet::new();
            let mut visited_dic: HashMap<T, T> = HashMap::new();

            let vec_distance = vec![];

            for value in graph_info {
                {
                    let vertex_x = &mut adjacency_dic.entry(value.0).or_insert(vec![]);
                    vertex_x.push((value.0, value.2))
                }

                all_vertex.insert(Edge {
                    first: value.0,
                    second: value.1,
                    weight: value.2,
                });

                visited_dic.insert(value.0, value.0);
                visited_dic.insert(value.1, value.1);
            }

            let sorted_dist = graph_info.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        }
    }
}
