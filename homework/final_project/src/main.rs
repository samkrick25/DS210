mod filehandling;
//::{read_articles, read_edges, read_shortest_paths};
use filehandling::centrality; //::{get_degrees, calc_degrees, calc_betweenness};

//questions for discussion: better way of imports? glob importer wasn't working correctly so I did what is above, but this seems annoying
//to maintain if i add or remove functions
//good way to visualize my data? should I be graphing everything at the start or would that be too cluttered?
//what kind of output is expected, or best for this kind of scenario?
//do comments count for line requirement if they are useful comments?

//TODO: write tests for get_degrees, calc_degrees, calc_betweenness
//library to graph --> plotters
//4-5 graphs, if it takes longer than 10 to gen graph, then just plot and highlight the top
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
    // let edge_list_num = edge_string_to_number(&article_map, &edge_list_str);
    centrality::get_degrees(&edge_list_str, &mut article_map);
    centrality::calc_degrees(&mut article_map);
    centrality::calc_betweenness(&shortest_path_mat, &mut article_map, &article_id_map);
}
