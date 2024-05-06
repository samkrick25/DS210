//Starting with imports, the first two are used to read files and create BufReaders to iterate over the lines of the file
//urlencoding is used to decode the article names from both articles.tsv and links.tsv, crate can be found at https://crates.io/crates/urlencoding
//HashMaps are used to store information about the article, its ID, and the centrality measures that I use
use std::io::prelude::*;
use std::fs::File;
use urlencoding::decode;
use std::collections::HashMap;

//setting up types for readability later in code
pub type ArticleMap = HashMap<String, (usize, usize, usize, usize, f64)>;
pub type ArticleID = HashMap<usize, String>;
pub type EdgeListStr = Vec<(String, String)>;
pub type AdjacencyList = HashMap<usize, Vec<usize>>;
pub type EdgeListInt = Vec<(usize, usize)>;

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
            let owned_article = decoded_article.into_owned();         //.into_owned is used here since decode() returns a Cow Object,
            articles.insert(owned_article.clone(), (line_number-12, 0, 0, 0, 0.0)); // and I want to enter it in to my ArticleMap as a String
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
            str_edges.push((article1.to_string(), article2.to_string()));               //and push the articles to my edge list
        }
    }
    str_edges
}

pub fn edge_string_to_number(articles: &ArticleMap, edges: &EdgeListStr) -> EdgeListInt {
    //This function turns the edgelist from strings into numbers. The articles are initially brought in as strings and
    //the initial edge list is strings, this function takes the edge list as strings and turns it into integers based on the
    //line that the article name is on. 
    let mut edges_num = vec![];
    for (article1, article2) in edges {
        let (article1_num, _, _, _, _) = articles.get(article1).unwrap();
        let (article2_num, _, _, _, _) = articles.get(article2).unwrap();
        edges_num.push((*article1_num, *article2_num));
    }
    edges_num
}

pub fn adjacency_from_edges(edges_num: &EdgeListInt) -> AdjacencyList {
    //this creates an adjacency list from the edge list, this is a directed graph so 
    //each edge only means a neighbor in one direction
    let mut adjacency_list = HashMap::new();
    for (edge1, edge2) in edges_num {
        adjacency_list.entry(*edge1).or_insert(Vec::new()).push(*edge2);
    }
    adjacency_list
}

pub trait TopElementsPrinter {
    //this trait is to print the top 20 centrality scores, has to be done like this since you can't have an implementation
    //written for a type defined with the type keyword
    fn print_top_20_by_criteria(&self, criteria: &str);
}

impl TopElementsPrinter for ArticleMap {
    fn print_top_20_by_criteria(&self, criteria: &str) {
        //this prints the top 20 nodes in each centrality score
        //it will take a string slice as an argument, corresponding to which centrality measure I want to sort by
        match criteria {
            "indegree" => {
                let mut indeg_vec: Vec<_> = self.into_iter().collect();
                //here the hashmap is converted into a vector and then sorted by the specific value corresponding
                //to the centrality measure that I want to sort by
                indeg_vec.sort_by_key(|&(_, (_, _, value, _, _))| value); //indegree centrality is in the 3rd position
                for (i, (k, v)) in indeg_vec.iter().enumerate() {
                    if i >= 20 { break; }
                    println!("{} indegree centrality: {}", k, v.2);
                }
            },
            //same framework is used to sort by outdegree
            "outdegree" => {
                let mut outdeg_vec: Vec<_> = self.into_iter().collect();
                outdeg_vec.sort_by_key(|&(_, (_, _, _, value, _))| value); //outdegree in the 4th position
                for (i, (k, v)) in outdeg_vec.iter().enumerate() {
                    if i >= 20 { break; }
                    println!("{} outdegree centrality: {}", k, v.3);
                }
            },
            "betweenness" => {
                let mut between_vec: Vec<_> = self.into_iter().collect();
                //this has to be sorted with partial_cmp since the values are f64s instead of int's
                //sort_by_key doesn't work with floats since it requires the values to implement Ord
                between_vec.sort_by(|(_, (_, _, _, _, a)), (_, (_, _, _, _, b))| a.partial_cmp(b).unwrap());
                for (i, (k, v)) in between_vec.iter().enumerate() {
                    if i >= 20 { break; }
                    println!("{} betweenness centrality: {}", k, v.4);
                }
            },
            _ => panic!("Invalid sorting criteria!"),
        }
    }
}

pub mod centrality;