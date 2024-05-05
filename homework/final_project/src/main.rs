mod filehandling;
use filehandling::centrality;
use crate::filehandling::TopElementsPrinter;

//TODO: write tests for get_degrees, calc_degrees, calc_betweenness
//so actual TODO: write tests for my functions, come up with a small graph I can figure out what everything should be and then test from there
//write functions to sort and print top, probably make that a new module under centrality
//and comment code, give write up, if i have time rework articlemap values to a struct (I could write all the centrality functions as implementations, they all need articlemap anyways)

fn main() {
    let path_articles = "articles.tsv";
    let path_links = "links.tsv";
    let (mut article_map, article_id_map) = filehandling::read_articles(&path_articles);
    let edge_list_str = filehandling::read_edges(&path_links);
    let edge_list_num = filehandling::edge_string_to_number(&article_map, &edge_list_str);
    let adjacency_list = filehandling::adjacency_from_edges(&edge_list_num);
    centrality::get_degrees(&edge_list_str, &mut article_map);
    centrality::calc_degrees(&mut article_map);
    let component_count = centrality::calculate_betweenness_centrality(&adjacency_list, &mut article_map, &article_id_map);
    println!("There were {} components found", component_count);
    article_map.print_top_20_by_criteria("indegree");
    article_map.print_top_20_by_criteria("outdegree");
    article_map.print_top_20_by_criteria("betweenness");
}
