use std::io::prelude::*;
use std::fs::File;
use urlencoding::decode;

fn read_articles(path: &str) -> Vec<String> {
    let mut articles = vec![];
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file);
    let lines = bufreader.lines().enumerate();
    for (line_number, line_result) in lines {
        let articleurl = line_result.unwrap();
        if line_number < 13 {
            continue
        }
        let decodedarticle = decode(&articleurl).expect("Failed to decode article name!");
        articles.push(decodedarticle);
    }
    articles
}

fn read_edges(path: &str) -> Vec<(String, String)> {
    let mut edges = vec![];
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file):
    let lines = bufreader.lines().enumerate();
    for (line_number, line_result) in lines {
        let linkurl = line_result.unwrap();
        if line_number < 13 {
            continue
        }
        let decodedlink = decode(&linkurl).expect("Failed to decode link!");
        edgeiter = decodedlink.split("\t");
        if let (Some(article1), Some(article2)) = (edgeiter.next(), edgeiter.next()){
            edges.push((article1.to_string(), article2.to_string()));
        }
    }
    edges
}