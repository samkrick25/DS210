//Starting with imports, the first two are used to read files and create BufReaders to iterate over the lines of the file
//urlencoding is used to decode the article names from both articles.tsv and links.tsv, crate can be found at https://crates.io/crates/urlencoding
//HashMaps are used to store information about the article, its ID, and the centrality measures that I use
use std::io::prelude::*;
use std::fs::File;
use urlencoding::decode;
use std::collections::HashMap;

//setting up types for readability later in code
pub type ArticleMap = HashMap<String, (usize, usize, usize, f64)>;
pub type ArticleID = HashMap<usize, String>;
pub type EdgeListStr = Vec<(String, String)>;
pub type AdjacencyList = HashMap<usize, Vec<usize>>;
pub type EdgeListInt = Vec<(usize, usize)>;
pub type ShortestPathsMat = Vec<Vec<Option<usize>>>;

//TODO: if i have time make the values in ArticleMap into a struct that contains nodeID, indegree cent, outdegree cent, between cent
//TODO: add comments

pub fn read_articles(path: &str) -> (ArticleMap, ArticleID) {
    //this function will read in the file containing the article names. Some article names are URL encoded, so I use the external 
    //urlencoding crate to decode them. This function returns an ArticleMap, which is a HashMap where the keys are the articles as Strings
    //and the values are each usizes, where the first represents the numerical ID of the article, then indegree and outdegree centrality,
    //then betweenness centrality (I want to make this into a struct to use as values instead of a tuple...if i have time)
    let mut articles = ArticleMap::new();
    let mut article_id = ArticleID::new();
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file);
    let lines = bufreader.lines().enumerate(); //enumerate here because I need the line_number, since this will later be used
    for (line_number, line_result) in lines {  //in betweenness centrality function
        let articleurl = line_result.unwrap();
        if line_number < 12 { //the first 12 lines of the file are information about the file and how to cite its authors. 
            continue          //since I don't want this information in my data, I skip the first 12 lines, since the first article is
        }                     //listed on line 13
        else {
            let decoded_article = decode(&articleurl).expect("Failed to decode article name!");
            let owned_article = decoded_article.into_owned();                  //.into_owned is used here since decode() returns a Cow Object,
            articles.insert(owned_article.clone(), (line_number-12, 0, 0, 0.0)); // and I want to enter it in to my ArticleMap as a String
            article_id.insert(line_number-12, owned_article.clone());
        }
    }
    (articles, article_id)
}

pub fn read_edges(path: &str) -> EdgeListStr {
    //This function reads the links.tsv file, which represents the edges in the graph of wikipedia articles. It will return an
    //edge list, or a vector of tuples where the tuples contain the nodes that the edge connects. These are kept as a String
    //in order to be used to get values from the ArticleMap later on
    let mut str_edges = vec![];
    let file = File::open(path).expect("File failed to open!");
    let bufreader = std::io::BufReader::new(file);
    let lines = bufreader.lines().enumerate();
    for (line_number, line_result) in lines {
        let edge_url = line_result.unwrap();
        if line_number < 12 { //similar to the articles.tsv file, the first 12 lines of this file I do not want, so this if statement
            continue          //will skip over them
        }
        let decoded_link = decode(&edge_url).expect("Failed to decode link!");
        let mut edge_iter = decoded_link.split("\t");//since this file contains two article names separated by a tab, I use .split 
                                                     //to get them individually
        if let (Some(article1), Some(article2)) = (edge_iter.next(), edge_iter.next()){ //this will pattern match until my iterable is empty
            str_edges.push((article1.to_string(), article2.to_string())); //and push the articles to my edge list
        }
    }
    str_edges
}

pub fn read_shortest_paths(path: &str) -> ShortestPathsMat {
    //
    let file = File::open(path).expect("Failed to open file!");
    let bufreader = std::io::BufReader::new(file);
    let mut shortest_paths = vec![];
    for (line_number, line) in bufreader.lines().enumerate() {
        let line = line.unwrap();
        if line_number < 17 {
            continue
        }
        let distances_str = line.chars().collect::<Vec<char>>();
        let mut distances_num = vec![];
        for distance in distances_str {
            if distance == '_' {
                distances_num.push(None);
            } else {
                let u32dist = distance.to_digit(10).unwrap();
                distances_num.push(Some(u32dist as usize));
            }
        }
        shortest_paths.push(distances_num);
    }
    shortest_paths
}

pub fn edge_string_to_number(articles: &ArticleMap, edges: &EdgeListStr) -> EdgeListInt {
    let mut edges_num = vec![];
    for (article1, article2) in edges {
        let (article1_num, _, _, _) = articles.get(article1).unwrap();
        let (article2_num, _, _, _) = articles.get(article2).unwrap();
        edges_num.push((*article1_num, *article2_num));
    }
    edges_num
}

pub fn adjacency_from_edges(edges_num: &EdgeListInt) -> AdjacencyList {
    let mut adjacency_list = HashMap::new();
    for (edge1, edge2) in edges_num {
        adjacency_list.entry(*edge1).or_insert(Vec::new()).push(*edge2);
    }
    adjacency_list
}

pub mod centrality;