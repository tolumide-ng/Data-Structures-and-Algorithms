pub fn execute() {
    let result = kruskal::KrusKGraph::new(vec![
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

    println!("THE RESULT {:#?}", result);
}

mod kruskal {
    use std::cmp::Eq;
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    #[derive(Hash, Eq)]
    pub struct Edge<T> {
        first: T,
        second: T,
        weight: u64,
    }
    impl<T: Eq> PartialEq for Edge<T> {
        fn eq(&self, other: &Self) -> bool {
            self.first == other.first || self.second == other.second
        }
    }

    #[derive(Debug, Clone)]
    pub struct KrusKGraph<T> {
        graph: Option<Vec<(T, T, u64)>>,
    }

    impl<T> KrusKGraph<T>
    where
        T: Debug + Eq + Hash + Copy + Display + Clone,
    {
        pub fn new(graph_info: Vec<(T, T, u64)>) -> HashSet<(T, T, u64)> {
            let mut all_vertex: HashSet<Edge<T>> = HashSet::new();
            let mut visited_dic: HashMap<T, T> = HashMap::new();

            // let mut initial_set: HashSet<T> = HashSet::new();

            let mut all_graphs = vec![];

            for value in graph_info {
                all_vertex.insert(Edge {
                    first: value.0,
                    second: value.1,
                    weight: value.2,
                });

                visited_dic.insert(value.0, value.0);
                visited_dic.insert(value.1, value.1);

                all_graphs.push(value);
            }

            all_graphs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            let the_graph = KrusKGraph {
                graph: Some(all_graphs),
            };

            return the_graph.get_kruskals(&mut visited_dic);
        }

        pub fn get_kruskals(&self, visited_dic: &mut HashMap<T, T>) -> HashSet<(T, T, u64)> {
            let mut mst: HashSet<(T, T, u64)> = HashSet::new();

            // let status = visited_dic;

            for edge in self.graph.as_ref().unwrap() {
                let source_edge_0 = self.find_origin(edge.0, visited_dic);
                let source_edge_1 = self.find_origin(edge.1, visited_dic);

                if source_edge_0 != source_edge_1 {
                    self.union_set(source_edge_0, source_edge_1, visited_dic);

                    mst.insert(*edge);
                }
            }

            return mst;
        }

        pub fn find_origin(&self, value: T, visited_dic: &mut HashMap<T, T>) -> T {
            let source = visited_dic.get(&value).unwrap();
            if *source != value {
                return self.find_origin(*source, visited_dic);
            } else {
                return value;
            }
        }

        pub fn union_set(&self, x: T, y: T, visited_dic: &mut HashMap<T, T>) {
            let root_x = self.find_origin(x, visited_dic);
            let root_y = self.find_origin(y, visited_dic);
            if root_x == root_y {
                return;
            } else {
                visited_dic.insert(x, y);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::kruskal;

    #[test]
    fn get_mst_distance() {
        let result = kruskal::KrusKGraph::new(vec![
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

        let the_sum: u64 = result.iter().map(|num| num.2).sum();

        assert_eq!(the_sum, 14);
    }
}
