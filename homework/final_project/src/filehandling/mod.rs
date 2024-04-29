use std::io::prelude::*;
use std::fs::File;
use urlencoding::decode;
use std::collections::HashMap;

type ArticleMap = HashMap<String, DegreeTuple>;
type EdgeListStr = Vec<(String, String)>;
type EdgeListInt = Vec<(usize, usize)>;
struct NodeInfoTuple{usize, usize, usize};

fn read_articles(path: &str) -> ArticleMap {
    let mut articles = HashMap::<String, NodeInfoTuple>::new();
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file);
    let lines = bufreader.lines().enumerate();
    for (line_number, line_result) in lines {
        let articleurl = line_result.unwrap();
        if line_number < 13 {
            continue
        }
        else {
            let decodedarticle = decode(&articleurl).expect("Failed to decode article name!");
            articles.insert(decodedarticle.into_owned(), NodeInfoTuple(line_number-13, 0, 0));
        }
    }
    articles
}

fn read_edges(path: &str) -> EdgeListStr {
    let mut str_edges = vec![];
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file);
    let lines = bufreader.lines().enumerate();
    for (line_number, line_result) in lines {
        let edge_url = line_result.unwrap();
        if line_number < 13 {
            continue
        }
        let decoded_link = decode(&edge_url).expect("Failed to decode link!");
        edge_iter = decoded_link.split("\t");
        if let (Some(article1), Some(article2)) = (edge_iter.next(), edge_iter.next()){
            str_edges.push((article1.to_string(), article2.to_string()));
        }
    }
    edges
}

fn edge_string_to_number(articles: &ArticleMap, edges: &EdgeListStr) -> EdgeListInt {
    let mut edges_num = vec![];
    for (article1, article2) in edges {
        let (article1_num, _, _) = articles[article1];
        let (article2_num, _, _) = articles[article2];
        edges_num.push((article1_num, article2_num));
    }
    edges_num
}

fn make_adjacency_matrix(edgelist: &EdgeListInt, articles: &ArticleMap) -> Vec<Vec<bool>> {
    let n = articles.len();
    let adjacency_matrix = vec![vec![false; n]; n];
    for (article1, article2) in edgelist {
        adjacency_matrix[*article1][*article2] = true;
    }
    adjacency_matrix
}
