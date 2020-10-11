// CHECK THIS OUT: https://www.youtube.com/watch?v=7GnSrrJP7wc

pub fn execute() {
    // println!("run the execute fn");
    // graph_shortest::Graph::new(vec![(4, 4), (1, 2), (4, 1), (2, 3), (3, 1), (2, 4)]);
    // graph_shortest::Graph::new(vec![(5, 4), (5, 2), (1, 3), (3, 4), (1, 4), (3, 5)]);
    graph_shortest::Graph::new(vec![
        ("dummy", "dummy"),
        ("sango", "abeokuta"),
        ("sango", "meiran"),
        ("meiran", "command"),
        ("meiran", "abuleEgba"),
        ("command", "abuleEgba"),
        ("meiran", "ikeja"),
        ("command", "ikeja"),
        ("abuleEgba", "ikeja"),
        ("abeokuta", "ikeja"), // shortest path from abeokuta to ikeja
    ]);
}

mod graph_shortest {
    use std::cmp::Eq;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::hash::Hash;

    pub struct Graph<T> {
        graph: Option<HashMap<T, Vec<T>>>,
    }

    impl<T: Debug + Eq + Hash + Copy + Display> Graph<T> {
        pub fn new(graph_info: Vec<(T, T)>) -> i8 {
            let mut adjacency_dic: HashMap<T, Vec<T>> = HashMap::new();

            let graph_length = graph_info.len();
            let graph = graph_info.get(1..graph_length - 1);
            for value in graph.unwrap() {
                let vertex_x = &mut adjacency_dic.entry(value.0).or_insert(vec![]);
                vertex_x.push(value.1);

                let vertex_y = &mut adjacency_dic.entry(value.1).or_insert(vec![]);
                vertex_y.push(value.0);
            }

            let the_graph = Graph {
                graph: Some(adjacency_dic),
            };

            let distance = graph_info.get(graph_length - 1).unwrap();
            return the_graph.get_shortest_path(distance.0, distance.1);
        }

        pub fn get_shortest_path(&self, start_location: T, end_location: T) -> i8 {
            // let mut distance_from_origin = vec![-1].repeat(graph_length);
            let mut distance_from_origin: HashMap<T, usize> = HashMap::new();
            let mut queue: Vec<T> = vec![];
            let mut current_distance: usize = 0;
            let unwraped_graph = &self.graph.as_ref().unwrap();

            queue.push(start_location);
            distance_from_origin.insert(start_location, current_distance);

            loop {
                if current_distance < queue.len() {
                    let neighbours = unwraped_graph.get(&queue[current_distance]);
                    if neighbours != None {
                        for value in neighbours.unwrap() {
                            if !queue.contains(value) {
                                queue.push(*value);
                                distance_from_origin.insert(*value, current_distance + 1);
                            }
                        }
                    }
                    current_distance += 1;
                } else {
                    println!("TOTAL DIC NOW {:?}", distance_from_origin);
                    break;
                }
            }

            println!("THE QUEUE {:?}", queue);

            if distance_from_origin.get(&end_location) == None {
                println!("THIS IS IT -1");
                return -1;
            } else {
                println!(
                    "THIS IS IT {:?}",
                    distance_from_origin.get(&end_location).unwrap()
                );
                return *(distance_from_origin.get(&end_location).unwrap()) as i8;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::graph_shortest;

    #[test]
    fn minimum_flights() {
        let result =
            graph_shortest::Graph::new(vec![(4, 4), (1, 2), (4, 1), (2, 3), (3, 1), (2, 4)]);
        assert_eq!(result, 2);
    }

    #[test]
    fn no_flights() {
        let result =
            graph_shortest::Graph::new(vec![(5, 4), (5, 2), (1, 3), (3, 4), (1, 4), (3, 5)]);
        assert_eq!(result, -1);
    }
}
