mod directed_graph;
mod undirected_graph;

// use undirected_graph::exit_maze::execute;
// use undirected_graph::add_exits::execute;
// use directed_graph::detect_cycle::execute;
use directed_graph::topological_order::execute;

fn main() {
    execute();
}
