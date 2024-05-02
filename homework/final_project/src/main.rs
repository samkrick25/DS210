mod filehandling;
use filehandling::centrality;

//TODO: write tests for get_degrees, calc_degrees, calc_betweenness
//library to graph --> plotters
//3 graphs, if it takes longer than 10min to gen graph, then just plot and highlight the top
//heatmap/color gradient for betweenness and degree, maybe look for one other graph, maybe try a connected components graph
//maybe just show part of graph, zoom on an interesting part
//find one more graph

fn main() {
    let path_articles = "articles.tsv";
    let path_links = "links.tsv";
    let path_shortest = "shortest-path-distance-matrix.txt";
    let (mut article_map, article_id_map) = filehandling::read_articles(&path_articles);
    let shortest_path_mat = filehandling::read_shortest_paths(&path_shortest);
    let edge_list_str = filehandling::read_edges(&path_links);
    let edge_list_num = filehandling::edge_string_to_number(&article_map, &edge_list_str);
    let adjacency_list = filehandling::adjacency_from_edges(&edge_list_num);
    centrality::get_degrees(&edge_list_str, &mut article_map);
    centrality::calc_degrees(&mut article_map);
    centrality::calculate_betweenness_centrality(&adjacency_list, &shortest_path_mat, &mut article_map, &article_id_map);
    println!("{:?}", article_map)
}
