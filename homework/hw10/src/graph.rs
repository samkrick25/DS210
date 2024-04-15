use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

mod graph {
    pub type Vertex = usize;
    pub type AdjacencyList = Vec<Vec<Vertex>>;
    pub type EdgeList = Vec<(usize, usize)>;
    //create graph struct for storing number of vertices, edge list, and adjacency list
    pub struct Graph {
        n: usize,
        edges: EdgeList,
        adjacencylist: AdjacencyList,
    }

    impl Graph {
        pub fn edge_list_from_file(path: &str) -> (EdgeList, usize) {
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
                if line_number == 0 {
                    n = line_str.trim().parse::<usize>().expect("Invalid number of vertices!");
                } else {
                    let v: Vec<&str> = line_str.trim().split(" ").collect();
                    let x = v[0].parse().expect("Invalid vertex format!");
                    let y = v[1].parse().expect("Invalid vertex format!");
                    edges.push((x, y));
                }
            }
            (edges, n)
        }

        pub fn adjacency_from_edges(edges: &EdgeList, n: &usize) -> AdjacencyList {
            //this function creates an adjacency list, takes an edgelist and a usize n, which is the total number of vertices in the graph
            //it will return an adjacency list which is a vector of vectors that have usize values in them, which represent the nodes that
            //the node represented by the index of the inner vectors has an edge with. This will be a directed edge, so the edge is only
            //added one way
            let mut adjacencylist = vec![vec![]; n];
            for (u, v) in edges {
                adjacencylist[*u].push(*v);
            }
            adjacencylist
        }

        pub fn create_graph(n: &usize, edges: &EdgeList, adjacencylist: AdjacencyList) -> Graph {
            //just take the number of vertices, edge list, and adjacency list created from previous functions and return a Graph object
            Graph {n, edges, adjacencylist}
        }
    }
}