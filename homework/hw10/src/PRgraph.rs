//import everything needed, will use binary heap to hold the count of how many walks end in any given node
//Ordering is used to create a custom tuple type that ranks on one value to be used in the ranking of nodes in the walking process
//rand is used in the walking process
//the other two are used for file reading
use std::cmp::Ordering;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use rand::prelude::SliceRandom;

//create some types for use in function definitions for readability
pub type Vertex = usize;
pub type AdjacencyList = Vec<Vec<Vertex>>;
pub type EdgeList = Vec<(usize, usize)>;
pub type EndingList = Vec<EndsTuple>;

//create graph struct for storing number of vertices, edge list, and adjacency list
pub struct GraphStruct {
    pub n: usize,
    pub edges: EdgeList,
    pub adjacencylist: AdjacencyList,
}

pub fn edge_list_from_file(path: &str) -> (EdgeList, Vertex) {
    //this will read the file given by professor, and return an edge list and a usize that represents the number of vertices
    //in the graph, which corresponds to the first line of the file
    let mut edges = vec![];
    let mut n = 0;
    let file = File::open(path).expect("Unable to open file!");
    let bufreader = std::io::BufReader::new(file);
    //here I use enumerate in order to create an iterator from the bufreader object. Since the first line in the file
    //is different than the rest, I use enumerate along with if statements to handle the first line separately from the 
    //rest of the file
    let lines = bufreader.lines().enumerate();
    for (line_number, line_result) in lines {
        let line_str = line_result.expect("Unable to read line!");
        //this is the first line in the file, or the number of vertices in the graph
        if line_number == 0 {
            n = line_str.trim().parse::<usize>().expect("Invalid number of vertices!"); 
        } else {
            let v: Vec<&str> = line_str.trim().split(" ").collect();//here, collect is used to collect results into a vector
            let x = v[0].parse::<usize>().expect("Invalid vertex format!");//which is then parsed into usize objects
            let y = v[1].parse::<usize>().expect("Invalid vertex format!");//to be placed into a tuple and pushed to the edgelist
            edges.push((x, y));
        }
    }
    (edges, n)
}

pub fn adjacency_from_edges(edges: &EdgeList, n: &usize) -> AdjacencyList { //THIS NEEDS TO ACCEPT TUPLE, UNPACK AT START
    //this function creates an adjacency list, takes an edgelist and a usize n, which is the total number of vertices in the graph
    //it will return an adjacency list which is a vector of vectors that have usize values in them, which represent the nodes that
    //the node represented by the index of the inner vectors has an edge with. This will be a directed edge, so the edge is only
    //added one way
    let mut adjacencylist = vec![vec![]; *n];
    for (u, v) in edges { //iterate over edge list
        adjacencylist[*u].push(*v); //and add each neighbor to the index of the starting node
    }
    adjacencylist
}

pub fn create_graph(n: usize, edges: EdgeList, adjacencylist: AdjacencyList) -> GraphStruct {
    //just take the number of vertices, edge list, and adjacency list created from previous functions and return a Graph object
    GraphStruct {n, edges, adjacencylist}
}

//To create a container of the number of times each walk ended in a certain node, I want to use a binary heap
//however, I need to contain the vertex identity, as well as the number of times the walk ended at that vertex
//so this struct will be a new tuple I will create a new Ord implementation that will compare based on only one of the values
//that way, when the priority queue pushes one of these tuples, it will compare based on the number of times a walk ended
//in the associated vertex
#[derive(Eq, PartialEq)]
pub struct EndsTuple(usize, usize);

impl PartialOrd for EndsTuple {
    //implementation for partial_cmp, returns an option since partial_cmp can give Greater, Less, Equal, or None
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for EndsTuple {
    //Implementation for Ord for the custom tuple type, doesn't need to return an option since this is a definitive comparison
    //but only compares on the first element, so when using in the walk function, the first element of this tuple will be the 
    //number of times a walk ended in the associated vertex
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

// //COMMENT
// pub trait PushOrCount {
//     fn push_or_count(&mut self, current_node: &Vertex);
// }

// //COMMENT
// impl<EndsTuple> PushOrCount for BinaryHeap<EndsTuple> {
//     //COMMENT
fn push_or_count(endslist: &mut Vec<EndsTuple>, current_node: &Vertex) {
    //just fully rewrite this to use a vector, binary heaps are unneccesary here, write a new function that finds the right node
    //then at the end sort function by the first value of the tuple
    let mut found = false;
    let mut index: usize = 0;
    for (i, ends) in endslist.iter().enumerate() {
        if current_node == &ends.0 {
            found = true;
            index = i;
            break
        }
    }
    if found {
        endslist[index].1 += 1
    }
    else {endslist.push(EndsTuple(*current_node, 1))}
}

pub mod PageRank {
    //this module will contain the pagerank function, as well as a test for the pagerank function

    use super::*;
    
    pub fn random_walk(graph: &GraphStruct) -> EndingList {
        //this is the pagerank function, which does as described in hw10 description. 
        let mut end_of_walks = Vec::new(); 
        let mut rng = rand::thread_rng(); //start rng 
        for _ in 0..100 { //do 100 random walks
            let mut current_node: Vertex = rng.gen_range(0..999); //pick a random node label to use as current node
            for _ in 0..100 { //do 100 random steps
                //if the current node doesn't have neighbors, pick a random node in the graph
                //else, pick a neighboring node of the current node 9/10 times, and 1/10 times, pick a random node
                if graph.adjacencylist[current_node].is_empty() {
                    current_node = choose_new_node(&current_node);
                }
                else {
                    let selection = rng.gen_range(0..10);
                    if selection == 0 {
                        current_node = choose_new_node(&current_node);
                    }
                    if 0 < selection && selection <= 9 {
                        //look at the inner vector at the index of current node in the adjacency list
                        let current_neighbors = &graph.adjacencylist[current_node]; 
                        current_node = *current_neighbors.choose(&mut rng).unwrap(); //choose on of the neighbors at random, handle Option
                    }
                }
            }
            push_or_count(&mut end_of_walks, &current_node); //add one to the amount of times a walk ended at a certain node
        }
        end_of_walks //TURN INTO A PRIORITY QUEUE? I need some way to save vertex identity
    }

    fn choose_new_node(current_node:&Vertex) -> Vertex {
        //this function is written to pick a random node to start from. it takes the current_node as an input returns a new node
        //A while loop is used to ensure that the picked node is different than the current node. I create an rng thread, then
        //as long as that node is not the same as the current node, a new node is generated.
        let mut rng = rand::thread_rng();
        let mut new_node = current_node;
        while new_node == current_node {
            let new_node = &rng.gen_range(0..1000);
        }
        *new_node
    }

    pub fn print_top_5(end_of_walks: &EndingList, graph: &GraphStruct) {
        //COMMENT (i think i can do this better with closures but I don't know closures yet)
        let top_5_nodes = &end_of_walks[0..5];
        let mut scores = vec![];
        for EndsTuple(node, score) in top_5_nodes.iter() {
            scores.push((node, score / (100*graph.n)));
        }
        println!("Top 5 PageRank scores:");
        for (node, score) in scores {
            println!("Node {} has PageRank {}", node, score);
        }
    }
}

//tests to write: check pagerank score sums to 1.0, check that the endingList has the correct number of elements,
//choose_new_node actually picks a new node, all of the comparison stuff works correctly