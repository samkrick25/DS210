use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

mod Graph {
    pub type Vertex = usize;
    pub type AdjacencyList = Vec<Vec<Vertex>>;
    pub type EdgeList = Vec<(usize, usize)>;
    //create graph struct for storing number of vertices, edge list, and adjacency list
    pub struct GraphStruct {
        pub n: usize,
        pub edges: EdgeList,
        pub adjacencylist: AdjacencyList,
    }

    impl GraphStruct {
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

        pub fn create_graph(n: &usize, edges: &EdgeList, adjacencylist: AdjacencyList) -> GraphStruct {
            //just take the number of vertices, edge list, and adjacency list created from previous functions and return a Graph object
            Graph {n, edges, adjacencylist}
        }
    }
    pub mod PageRank {
        //ADD COMMENTS AND DESCRIBE CODE
        pub fn random_walk(graph: &GraphStruct) -> Vec<usize> {
            let mut end_of_walks = vec![0; graph.n];
            rng = rand::thread_rng();
            for _ in 0..99 {
                let mut current_node: Vertex = rng.gen_range(0..999);
                for _ in 0..99 {
                    if graph.adjacencylist[current_node].is_empty() {
                        current_node = choose_new_node(current_node);
                    }
                    else {
                        selection = rng.gen_range(0..9);
                        if selection = 0 {
                            current_node = choose_new_node(current_node);
                        }
                        if 0 < selection <= 9 {
                            current_neighbors = graph.adjacencylist[current_node];
                            current_node = current_neighbors.choose();
                        }
                    }
                }
                end_of_walks[current_node] += 1;
            }
            end_of_walks //TURN INTO A PRIORITY QUEUE? I need some way to save vertex identity
        }

        pub fn choose_new_node(current_node: Vertex) -> Vertex {
            //this function is written to pick a random node to start from. it takes the current_node as an input returns a new node
            //A while loop is used to ensure that the picked node is different than the current node. I create an rng thread, then
            //as long as that node is not the same as the current node, a new node is generated.
            rng = rand::thread_rng();
            let mut new_node = current_node;
            while new_node == current_node {
                new_node = rng.gen_range(0..999);
            }
            new_node
        }

        pub fn print_top_5(end_of_walks: Vec<usize>, graph: GraphStruct) {
            //pop off priority queue 5 times, put those into top_5_nodes, then do rest of stuff and print
            let top_5_nodes = end_of_walks[..5];
            let top_5_pagerank = top_5_nodes as f64 / 100*graph.n as f64;
            println!("")
        }
    }
}