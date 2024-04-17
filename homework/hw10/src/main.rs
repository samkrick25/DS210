//Imports from my Graph module
mod PRgraph;

fn main() {
    use PRgraph::{edge_list_from_file, adjacency_from_edges, create_graph};
    use PRgraph::PageRank::{random_walk, print_top_5};
    let path = "pagerank_data.txt";
    let (edgelist, n) = edge_list_from_file(path);
    let adjacencylist = adjacency_from_edges(&edgelist, &n);
    let graph = create_graph(n, edgelist, adjacencylist);
    let endlist = random_walk(&graph);
    print_top_5(&endlist, &graph);
}
