pub fn execute() {
    let result = add_exits(vec![[4, 2], [1, 2], [3, 2], [2, 1]]);
    println!("RESULT<<<<<<<>>>>>>>>>>>>> {:?}", result);
}

use std::collections::HashSet;

pub fn add_exits(graph_info: Vec<[u8; 2]>) -> usize {
    let mut set: HashSet<[u8; 2]> = HashSet::new();
    let graph = graph_info.get(1..);

    for edge in graph.unwrap() {
        let mut val = edge.clone();
        val.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        set.insert(val);
    }

    // println!("the complete set {:?}", set);
    return set.len();
}

#[cfg(test)]
mod test {
    use super::add_exits;
    #[test]
    fn connected_components() {
        let result = add_exits(vec![[4, 2], [1, 2], [3, 2], [2, 1]]);
        assert_eq!(result, 2);
    }
}
