mod filehandling;
use filehandling::{read_articles, read_edges, read_shortest_paths};
use filehandling::centrality::{get_degrees, calc_degrees, calc_betweenness};

//questions for discussion: better way of imports? glob importer wasn't working correctly so I did what is above, but this seems annoying
//to maintain if i add or remove functions
//good way to visualize my data? should I be graphing everything at the start or would that be too cluttered?
//what kind of output is expected, or best for this kind of scenario?
//do comments count for line requirement if they are useful comments?

//TODO: write tests for get_degrees, calc_degrees, calc_betweenness

fn main() {
    let path_articles = "articles.tsv";
    let path_links = "links.tsv";
    let path_shortest = "shortest-path-distance-matrix.txt";
    let (mut article_map, article_id_map) = read_articles(&path_articles);
    let shortest_path_mat = read_shortest_paths(&path_shortest);
    let edge_list_str = read_edges(&path_links);
    // let edge_list_num = edge_string_to_number(&article_map, &edge_list_str);
    get_degrees(&edge_list_str, &mut article_map);
    calc_degrees(&mut article_map);
    calc_betweenness(&shortest_path_mat, &mut article_map, &article_id_map);
}
