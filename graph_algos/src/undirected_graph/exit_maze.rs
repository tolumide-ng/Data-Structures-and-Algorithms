use std::collections::HashMap;

pub fn execute() {
    let vector = vec![(4, 2), (1, 2), (3, 2), (1, 4)];
    // let vector = vec![(4, 4), (1, 2), (3, 2), (4, 3), (1, 4), (1, 4)];
    let result = find_maze_exit(vector);

    println!("RESULT >>>> {:?}", result);
}

pub fn find_maze_exit(graph_info: Vec<(u8, u8)>) -> u8 {
    type DicType = HashMap<u8, Vec<u8>>;

    let mut adjacency_list: DicType = HashMap::new();
    // we don't need the 0th item in the received vector since it only mentions the number of vertices and edges present
    let graph = graph_info.get(1..(graph_info.len() - 1));

    for value in graph.unwrap() {
        {
            let vertix_x = &mut adjacency_list.entry(value.0).or_insert(vec![]);
            vertix_x.push(value.1);
        }
        let vertix_y = &mut adjacency_list.entry(value.1).or_insert(vec![]);
        vertix_y.push(value.0);
    }

    // println!("RESULT >>>>>>>>>>>, {:?}", adjacency_list);
    let input_lastvalue = &graph_info[graph_info.len() - 1];
    if adjacency_list
        .get(&input_lastvalue.0)
        .unwrap()
        .contains(&input_lastvalue.1)
    {
        return 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod test {
    use super::find_maze_exit;

    #[test]
    fn find_reachability() {
        let maze = vec![(4, 4), (1, 2), (3, 2), (4, 3), (1, 4), (1, 4)];
        let result = find_maze_exit(maze);
        assert_eq!(result, 1);
    }

    #[test]
    fn no_reachability() {
        let maze = vec![(4, 2), (1, 2), (3, 2), (1, 4)];
        let result = find_maze_exit(maze);
        assert_eq!(result, 0);
    }
}
