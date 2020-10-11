pub fn execute() {
    let result = bipartite::Graph::new(vec![(5, 4), (5, 2), (4, 2), (3, 4), (1, 4)]);
    // let result = bipartite::Graph::new(vec![(4, 4), (1, 2), (4, 1), (2, 3), (3, 1)]);
    println!("RESULT OF BIPARTITE TEST {:?}", result);
}

mod bipartite {
    use std::cmp::Eq;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    pub struct Graph<T> {
        graph: Option<HashMap<T, Vec<T>>>,
    }

    impl<T: Debug + Eq + Hash + Copy + Display + Clone> Graph<T> {
        pub fn new(graph_info: Vec<(T, T)>) -> u8 {
            let mut adjacency_dic: HashMap<T, Vec<T>> = HashMap::new();

            let graph = graph_info.get(1..);

            let source_vertex = graph.unwrap()[0].0;

            for value in graph.unwrap() {
                let vertex_x = &mut adjacency_dic.entry(value.0).or_insert(vec![]);
                vertex_x.push(value.1);

                let vertex_y = &mut adjacency_dic.entry(value.1).or_insert(vec![]);
                vertex_y.push(value.0);
            }

            let the_graph = Graph {
                graph: Some(adjacency_dic),
            };

            // the_graph.get_bipartite(source_vertex);
            return the_graph.get_bipartite(source_vertex);
        }

        pub fn get_bipartite(&self, start_vertex: T) -> u8 {
            #[derive(Copy, Clone, Debug, PartialEq)]
            enum Colors {
                White,
                Black,
            };

            let mut bipartite_dic: HashMap<T, Colors> = HashMap::new();
            let mut queue: Vec<T> = vec![];
            let all_keys = self.graph.as_ref().unwrap().keys();
            let unwrapped_graph = self.graph.as_ref().unwrap();
            // println!("unwrapped {:#?}", unwrapped_graph);

            let mut current_color = Colors::White;
            let mut current_queue_number = 0;
            queue.push(start_vertex);
            bipartite_dic.insert(start_vertex, current_color);
            match current_color {
                Colors::White => current_color = Colors::Black,
                Colors::Black => current_color = Colors::White,
            }

            loop {
                if current_queue_number < queue.len() {
                    let current_vertex = queue[current_queue_number];
                    let vertex_neighbours = unwrapped_graph.get(&current_vertex);

                    if vertex_neighbours.is_some() {
                        for value in vertex_neighbours.unwrap() {
                            if !queue.contains(value) {
                                queue.push(*value);
                                if *(bipartite_dic.get(&current_vertex).unwrap()) == Colors::White {
                                    bipartite_dic.insert(*value, Colors::Black);
                                } else {
                                    bipartite_dic.insert(*value, Colors::White);
                                }
                            } else if bipartite_dic.get(&current_vertex) == bipartite_dic.get(value)
                            {
                                println!("NON-BIPARTITE");
                                // break;
                                return 0;
                            }
                        }
                        current_queue_number += 1;
                    }
                } else {
                    // println!("to break or not to breajk {:#?}", bipartite_dic);
                    println!("BIPARTITE");
                    return 1;
                    // break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::bipartite;

    #[test]
    fn test_bipartite() {
        let result = bipartite::Graph::new(vec![(5, 4), (5, 2), (4, 2), (3, 4), (1, 4)]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_nonbipartite() {
        let result = bipartite::Graph::new(vec![(4, 4), (1, 2), (4, 1), (2, 3), (3, 1)]);
        assert_eq!(result, 0)
    }
}
