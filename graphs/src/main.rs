use std::collections::HashSet;

mod depth_breadth_first;
use depth_breadth_first::Graph;

mod dijkstra;
use dijkstra::dijkstra_shortest_path;

fn main() {
    let mut graph = Graph::new(7);

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 5);
    graph.add_edge(2, 6);

    let start_vertex = 0;

    graph.dfs(start_vertex);

    println!("Dijkstra's shortest path algorithm");
    // Define a graph as an adjacency list
    let graph = vec![
        vec![(1, 2), (2, 5)],
        vec![(0, 2), (2, 2), (3, 1)],
        vec![(0, 5), (1, 2), (3, 6)],
        vec![(1, 1), (2, 6)],
    ];

    let start_vertex = 0;
    let shortest_distances = dijkstra_shortest_path(&graph, start_vertex);
    println!(
        "Shortest distances from vertex {}: {:?}",
        start_vertex, shortest_distances
    );
}
