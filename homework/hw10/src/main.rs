//Imports from my Graph module
mod prgraph;

fn main() {
    use prgraph::{edge_list_from_file, adjacency_from_edges, create_graph};
    use prgraph::page_rank::{random_walk, print_top_5};
    //let path = "pagerank_data.txt";
    let (edgelist, n) = edge_list_from_file("pagerank_data.txt");
    let adjacencylist = adjacency_from_edges(&edgelist, &n);
    let graph = create_graph(n, edgelist, adjacencylist);
    let endlist = random_walk(&graph);
    print_top_5(&endlist, &graph);
}

#[test]
fn test_page_rank_score() {
    use std::env;
    use prgraph::{EndsTuple, adjacency_from_edges, create_graph, edge_list_from_file};
    use prgraph::page_rank::{random_walk};
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join("src").join("pagerank_data.txt");
    let (edges, n) = edge_list_from_file(file_path.to_str().expect("Invalid file path"));
    let adjacencylist = adjacency_from_edges(&edges, &n);
    let graph = create_graph(n, edges, adjacencylist);
    let endings = random_walk(&graph);
    let mut scores = vec![];
    for EndsTuple(_node, score) in endings {
        scores.push(score as f64 / (10.0*graph.n as f64));
    }
    let scores_sum = scores.iter().fold(0.0, |a, b| a + b); //this closure will sum all of my scores so I can check that it == 1.0
    assert_eq!((scores_sum - 1.0).abs() < 1e-10, true, //test is written like this because of floating point precision,
    "Scores do not add up to approximately 1.0!") //I check whether the difference between them is less than 1e-10, a small threshold
}