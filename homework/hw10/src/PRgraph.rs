//import everything needed, will use binary heap to hold the count of how many walks end in any given node
//rand is used in the walking process, the seeds are used to create reproducible results for testing and debugging
//the other two are used for file reading
use rand::Rng;
use rand::{SeedableRng, rngs::StdRng};
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

//I need a way to save vertex identity when I sort my vector containing the amount of times a random walk ended at a specific node
//So this tuple will contain the node id in position 0, and the current number of times a walk has ended at that node in position 1
#[derive(Clone, Debug)]
pub struct EndsTuple(pub usize, pub usize);

fn push_or_count(endslist: &mut Vec<EndsTuple>, current_node: &Vertex) {
    //this funciton will take my endslist, and the current_node that my walking function is on, and will either add one to the count
    //of walks ended at the current_node, or if current_node is not in endslist, push with a count of 1.
    let mut found = false;
    let mut index: usize = 0;
    for (i, ends) in endslist.iter().enumerate() { //.iter() and .enumerate() are used here to first turn endslist into an iterable
        //then return both the index and the contents of endslist so that I can keep track of the position of the node that needs
        //to be added or updated
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

pub mod page_rank {
    //this module will contain the pagerank function, as well as a test for the pagerank function

    use super::*;
    
    pub fn random_walk(graph: &GraphStruct) -> EndingList {
        //this is the pagerank function, which does as described in hw10 description. 
        let mut end_of_walks = Vec::new(); 
        let seed_value = 42; //seed the rng here, for true random each time you run get rid of these two lines and replace with 
        let mut rng = StdRng::seed_from_u64(seed_value); //let mut rng = rand::thread_rng()
        for _ in 0..100 { //do 100 random walks
            let mut current_node: Vertex = rng.gen_range(0..999); //pick a random node label to use as current node
            for _ in 0..100 { //do 100 random steps
                //if the current node doesn't have neighbors, pick a random node in the graph
                //else, pick a neighboring node of the current node 9/10 times, and 1/10 times, pick a random node
                if graph.adjacencylist[current_node].is_empty() {
                    current_node = choose_new_node();
                }
                else {
                    let selection = rng.gen_range(0..10);
                    if selection == 0 {
                        current_node = choose_new_node();
                    }
                    if 0 < selection && selection <= 9 {
                        //look at the inner vector at the index of current node in the adjacency list
                        let current_neighbors = &graph.adjacencylist[current_node]; 
                        current_node = *current_neighbors.choose(&mut rng).unwrap(); //choose one of the neighbors at random, handle Option
                    }
                }
                push_or_count(&mut end_of_walks, &current_node); //add one to the amount of times a walk ended at a certain node
            }
        }
        end_of_walks
    }

    fn choose_new_node() -> Vertex {
        //This is a simple function that creates a new rng thread to choose a new node from. This was written in a separate 
        //function so that the rng changes each time instead of having it use the same one that was set with a seed in line
        //2 and 3 in random_walk()
        let mut rng = rand::thread_rng();
        let new_node = &rng.gen_range(0..1000);
        *new_node
    }

    pub fn print_top_5(end_of_walks: &EndingList, graph: &GraphStruct) {
        //This function will print the top 5 pagerank scores. It takes a GraphStruct as well as a EndingList, which represent my 
        //graph and the amount of times a walk ended at a particular node. This function will sort the EndingList, then
        //take a slice of the top 5 values, divide each of their scores by 100n, and print them
        let mut sorted_nodes = end_of_walks.clone(); //make sure end_of_walks isn't modified
        sorted_nodes.sort_by(|a, b| b.1.cmp(&a.1)); //this sorts in descending order, so the highest is the first item, since when
        //an Ordering::Greater object is returned, b is placed before a.
        let top_5_nodes = &sorted_nodes[..5];
        let mut scores = vec![];
        for EndsTuple(node, score) in top_5_nodes.iter() {
            scores.push((node, *score as f64 / (10.0*graph.n as f64)));
        }
        println!("Top 5 PageRank scores:");
        for (node, score) in scores {
            println!("Node {} has PageRank {}", node, score);
        }
    }
}
