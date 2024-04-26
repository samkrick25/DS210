use std::io::prelude::*;
use std::fs::File;
use urlencoding::decode;
use std::collections::HashMap;

type ArticleMap = HashMap<String, usize>;
type EdgeList = Vec<(usize, usize)>;

fn read_articles(path: &str) -> ArticleMap {
    let mut articles = HashMap::<String, usize>::new();
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file);
    let lines = bufreader.lines().enumerate();
    for (line_number, line_result) in lines {
        let articleurl = line_result.unwrap();
        if line_number < 13 {
            continue
        }
        let decodedarticle = decode(&articleurl).expect("Failed to decode article name!");
        articles.insert(String::from(decodedarticle), line_number);
    }
    articles
}

fn read_edges(path: &str) -> Vec<(String, String)> {
    let mut str_edges = vec![];
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file):
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

fn edge_string_to_number(articles: ArticleMap, edges: Vec<(String, String)>) -> EdgeList {
    let mut edges_num = vec![];
    for (article1, article2) in edges {
        let article1_num = articles[article1];
        let article2_num = articles[article2];
        edges_num.push((article1_num, article2_num));
    }
    edges_num
}