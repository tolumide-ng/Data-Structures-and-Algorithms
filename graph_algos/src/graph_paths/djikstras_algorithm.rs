pub fn execute() {
    // break user input and remove the last one to get the start and end-location once it is entered
    use djikstra;

    // let result = djikstra::DjikGraph::new(vec![(1, 2, 1), (4, 1, 2), (2, 3, 2), (1, 3, 5)], (1, 3));
    // let result = djikstra::DjikGraph::new(
    //     vec![
    //         (1, 2, 4),
    //         (1, 3, 2),
    //         (2, 3, 2),
    //         (3, 2, 1),
    //         (2, 4, 2),
    //         (3, 5, 4),
    //         (5, 4, 1),
    //         (2, 5, 3),
    //         (3, 4, 4),
    //     ],
    //     (1, 5),
    // );
    // let result = djikstra::DjikGraph::new(vec![(1, 2, 7), (1, 3, 5), (2, 3, 2)], (3, 2));
    let result = djikstra::DjikGraph::new(
        vec![
            ("abeokuta", "ibadan", 2),
            ("abeokuta", "lagos", 2),
            ("lagos", "ilorin", 3),
            ("ibadan", "osogbo", 3),
            ("ibadan", "ilorin", 2),
            ("osogbo", "ilorin", 2),
            ("ilorin", "kogi", 1),
            // ("ilorin", "ekiti", 1),
            // ("ekiti", "akure", 2),
            ("ibadan", "kogi", 2),
        ],
        ("abeokuta", "kogi"),
        // ("abeokuta", "ilorin"),
    );
    println!("THE SHORTEST DISTANCE TO THIS PATH IS {:?}", result);
}

// directed graph using djiskstra's algorithm
mod djikstra {
    use std::cmp::Eq;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    pub type GraphType<T> = HashMap<T, Vec<(T, u64)>>;

    #[derive(Clone, Debug)]
    pub struct DjikVertex<T> {
        vertex: T,
        shortest_distance: f64,
        previous_vertex: Option<T>,
    }

    #[derive(Clone, Debug)]
    pub struct DjikGraph<T> {
        graph: Option<GraphType<T>>,
    }

    impl<T: Debug + Eq + Hash + Copy + Display + Clone> DjikGraph<T> {
        pub fn new(graph_info: Vec<(T, T, u64)>, travel_locations: (T, T)) -> i64 {
            let mut adjacency_dic: HashMap<T, Vec<(T, u64)>> = HashMap::new();

            let graph_path = graph_info.get(0..).unwrap();
            let mut distance_dic: HashMap<T, DjikVertex<T>> = HashMap::new();

            for value in graph_path {
                // {
                let vertex_x = &mut adjacency_dic.entry(value.0).or_insert(vec![]);
                vertex_x.push((value.1, value.2));

                distance_dic.insert(
                    value.0,
                    DjikVertex {
                        vertex: value.0,
                        shortest_distance: f64::INFINITY,
                        previous_vertex: None,
                    },
                );
                distance_dic.insert(
                    value.1,
                    DjikVertex {
                        vertex: value.1,
                        shortest_distance: f64::INFINITY,
                        previous_vertex: None,
                    },
                );
            }

            let the_graph = DjikGraph {
                graph: Some(adjacency_dic),
            };

            return the_graph.get_shortest_paths(travel_locations, &mut distance_dic);
        }

        pub fn get_shortest_paths(
            &self,
            travel_locations: (T, T),
            distance_dic: &mut HashMap<T, DjikVertex<T>>,
        ) -> i64 {
            let mut visited: Vec<T> = vec![];
            let start_place = distance_dic.get_mut(&travel_locations.0);

            match start_place {
                Some(x) => {
                    x.shortest_distance = 0.00;
                    x.previous_vertex = None;
                }
                None => {}
            };

            self.djisktra_distance(travel_locations.0, distance_dic, &mut visited);

            let shortest_distance = distance_dic.get(&travel_locations.1);

            match shortest_distance {
                Some(x) => {
                    let shortest_distance = x.shortest_distance;
                    if shortest_distance == f64::INFINITY {
                        return -1;
                    } else {
                        return shortest_distance as i64;
                    }
                }
                None => return -1,
            }
        }

        pub fn djisktra_distance(
            &self,
            from_location: T,
            distance_dic: &mut HashMap<T, DjikVertex<T>>,
            visited: &mut Vec<T>,
        ) {
            let all_neighbours = self.graph.as_ref().unwrap().get(&from_location);

            if all_neighbours.is_some() {
                let mut current_min = f64::INFINITY;

                let distance_from_prev =
                    match distance_dic.get(&from_location).unwrap().shortest_distance {
                        x => x as f64,
                        // _ => current_min,
                    };

                for neighbour in all_neighbours.unwrap() {
                    if !visited.contains(&neighbour.0) {
                        let neighbour_min = neighbour.1 as f64 + distance_from_prev;

                        if (neighbour_min) < current_min {
                            current_min = neighbour.1 as f64;
                        }
                        if let Some(vertex) = distance_dic.get_mut(&neighbour.0) {
                            if vertex.shortest_distance > neighbour_min {
                                vertex.shortest_distance = neighbour_min;
                                vertex.previous_vertex = Some(from_location);
                            }
                        }
                    }
                    visited.push(from_location);
                }

                for neighbour in all_neighbours.unwrap() {
                    if !visited.contains(&neighbour.0) {
                        self.djisktra_distance(neighbour.0, distance_dic, visited);
                    }
                }
            } else {
                // return -1
                // no paths from this location
            }
            // }
        }
    }
}

#[cfg(test)]
mod test {
    use super::djikstra;

    #[test]
    fn shortest_path() {
        let result =
            djikstra::DjikGraph::new(vec![(1, 2, 1), (4, 1, 2), (2, 3, 2), (1, 3, 5)], (1, 3));
        assert_eq!(result, 3);
    }

    #[test]
    fn shortest_existing_path() {
        let result = djikstra::DjikGraph::new(
            vec![
                (1, 2, 4),
                (1, 3, 2),
                (2, 3, 2),
                (3, 2, 1),
                (2, 4, 2),
                (3, 5, 4),
                (5, 4, 1),
                (2, 5, 3),
                (3, 4, 4),
            ],
            (1, 5),
        );

        assert_eq!(result, 6);
    }

    #[test]
    fn no_existing_path() {
        let result = djikstra::DjikGraph::new(vec![(1, 2, 7), (1, 3, 5), (2, 3, 2)], (3, 2));

        assert_eq!(result, -1);
    }

    #[test]
    fn existing_path_places() {
        let result = djikstra::DjikGraph::new(
            vec![
                ("abeokuta", "ibadan", 2),
                ("abeokuta", "lagos", 2),
                ("ibadan", "osogbo", 3),
                ("ibadan", "ilorin", 2),
                ("osogbo", "ilorin", 2),
                ("lagos", "ilorin", 3),
                ("ilorin", "kogi", 1),
                ("ilorin", "ekiti", 1),
                ("ekiti", "akure", 2),
            ],
            ("abeokuta", "ilorin"),
        );

        assert_eq!(result, 4);
    }
}
