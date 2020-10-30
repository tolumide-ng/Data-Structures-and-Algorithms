// mod directed_graph;
mod graph_paths;
// mod undirected_graph;
// mod bfs;

// use undirected_graph::exit_maze::execute;
// use undirected_graph::add_exits::execute;
// use directed_graph::detect_cycle::execute;
// use directed_graph::topological_order::execute;
// use directed_graph::strongly_connected_comp::execute;
// use bfs::min_flight_segments::execute;
// use bfs::bipartite::execute;
// use graph_paths::djikstras_algorithm::execute;
use graph_paths::prims_algorithm::execute;

fn main() {
    execute();
}
