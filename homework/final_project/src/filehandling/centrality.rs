use super::{ArticleMap, EdgeListInt, ArticleID, AdjacencyList};
use std::collections::{HashSet, VecDeque};
//use std::time::SystemTime;
//TODO: ADD COMMENTS

pub fn get_degrees(edges: &EdgeListInt, articles: &mut ArticleMap, article_id: &ArticleID) {
    //this calculates the degree centrality by taking each edge, adding one to the outdegree centrality for the 
    //start node and adds one to the indegree centrality for the end node.
    for (edge1, edge2) in edges {
        let edge1_str = &article_id[edge1];
        if let Some(nodeinfo) = articles.get_mut(edge1_str) {//pattern match since outdegree kept in 3rd position
            nodeinfo.2 += 1;
        } 
        let edge2_str = &article_id[edge2];
        if let Some(nodeinfo) = articles.get_mut(edge2_str) {//pattern match since indegree kept in 4th position
            nodeinfo.3 += 1;
        }
    }
}

pub fn calc_degrees(articles: &mut ArticleMap) {
    //this normalizes the degree scores by dividing by the total number of nodes
    let length = articles.len();
    for (_, nodeinfo) in articles.iter_mut() {
        nodeinfo.2 = nodeinfo.2 / length;
        nodeinfo.3 = nodeinfo.3 / length;
    }
}

fn reconstruct_shortest_path(
    pred: &Vec<Option<usize>>, 
    start: usize, 
    end: usize,
) -> Vec<usize> {
    //this function is written for use in the betweenness centrality calculation.
    //it takes a list of predecessors for the given end  node and reconstructs the shortest path between
    //that end node and a given start node. It will calculate the shortest path in reverse, since
    //it traverses the graph in reverse, starting from the ending node to do this, so before I output 
    //I have to reverse the path 
    let mut path = Vec::new();
    let mut current = Some(end);
    while let Some(node) = current {
        if node == start {
            path.push(node);
            break;
        }
        if let Some(predecessor) = pred[node] {
            path.push(node);
            current = Some(predecessor);
        } else {
            // No path from start to end
            current = None
        }
    }
    path.reverse();
    path
}

fn bfs_predecessors(adjacency_list: &AdjacencyList, start: usize, article_id: &ArticleID) -> Vec<Option<usize>> {
    //this function does a BFS of my graph in order to create a list of predecessors for any given node. This predecessors list is 
    //used to reconstruct the shortest path from one node to any other given node. To do this, while doing a BFS, it will add each node
    //that it finds as a neighbor for a given node to the list of predecessors for that node, outputting a list for each node that contains
    //its neighbors listed in order of their distance away from the starting node.
    let num_nodes = article_id.len();
    let mut pred = vec![None; num_nodes];
    let mut visited = vec![false; num_nodes];

    let mut queue = VecDeque::new();
    visited[start] = true;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = adjacency_list.get(&current) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    queue.push_back(neighbor);
                    visited[neighbor] = true;
                    pred[neighbor] = Some(current);
                }
            }
        }
    }

    pred
}

pub fn calculate_betweenness_centrality(
    adjacency_list: &AdjacencyList, 
    article_map: &mut ArticleMap,
    article_id: &ArticleID,
) -> usize {
    //This function operates in place on my ArticleMap but also returns usize, corresponding to the number of components in the graph
    //This function will compute betweenness centrality of all the nodes in my graph. Betweenness centrality is defined
    //as the number of total shortest paths between all vertices in a given vertex's component that contain that vertex. To
    //calculate this I first have to find the components in the graph, done below
    let connected_components = find_components(adjacency_list);
    let component_count = connected_components.len();
    let mut count = 0;
    //then iterate over each component
    for component in connected_components {
        count += 1;
        let comp_length = &component.len() - 1;
        //and look at each node in that component
        for (i, start) in component.clone().into_iter().enumerate() {
            //to print a progress report as code runs
            if i % 500 == 0 {
                println!("{} starting vertices shortest path to all other vertices checked", i);
            }
            let article_name = article_id.get(&start).unwrap(); //thes lines add the component id into article map so I know which
            let (_, component_id, _, _, _) = article_map.get_mut(article_name).unwrap();//component each node is in
            *component_id = count;
            //This looks at every other node in the component, allowing us to consider all possible pairs of nodes
            //in each component
            for end in &component {
                if start != *end {
                    //if nodes are not the same, reconstruct the predecessors of the starting node, and see what the shortest path
                    //to the ending node is. The following two functions are commented as to what their function is.
                    let pred = bfs_predecessors(&adjacency_list, start, &article_id);
                    let path = reconstruct_shortest_path(&pred, start, *end);
                    let length = &path.len();
                    //then for each node in the shortest path between each start and end node, we will add one to its
                    //betweenness centrality score
                    for node in path.iter().skip(1).take(length - 1) { //skip and take are used here to skip the first node and take all but the last node
                        //since those don't need to increase betweenness
                        let article_name = article_id.get(node).unwrap();
                        let (_, _, _, _, between) = article_map.get_mut(article_name).unwrap();
                        *between += 1.0;
                    }
                }
            }
        }
        //Here we create the normalization factor for betweenness centrality, where the comp_length is the length of each component,
        //or how many possible shortest paths there were, while it is divided by two since I have a directed graph
        //and this algorithm will count each path twice
        let normal_factor = comp_length as f64 * comp_length as f64 / 2.0;
        for node in &component {
            let article_name = article_id.get(node);
            let (_, _, _, _, betweenness) = article_map.get_mut(article_name.unwrap()).unwrap();
            *betweenness /= normal_factor;
        }
    }
    component_count
}

fn dfs(
    node: usize,
    adjacency_list: &AdjacencyList,
    visited: &mut HashSet<usize>,
    component: &mut Vec<usize>,
) {
    //this function performs a simple recursive DFS, to be used to find the components of my graph. 
    // Mark the current node as visited
    visited.insert(node);
    component.push(node);
    // Visit all unvisited neighbors
    if adjacency_list.contains_key(&node) {
        for &neighbor in &adjacency_list[&node] {
            if !visited.contains(&neighbor) {
                dfs(neighbor, adjacency_list, visited, component);
            }
        }
    }
}

fn find_components(adjacency_list: &AdjacencyList) -> Vec<Vec<usize>> {
    //this function will find the components in the graph, creating a new vector for each component and then doing a dfs from each node
    //in the component that is unvisited already. This gives us a vector of vectors where each inner vector contains the node 
    //in a given component corresponding to the index of the inner vector.
    let mut components = Vec::new();
    let mut visited = HashSet::new();

    for &node in adjacency_list.keys() {
        if !visited.contains(&node) {
            let mut component = Vec::new();
            dfs(node, adjacency_list, &mut visited, &mut component);
            components.push(component);
        }
    }

    components
}

#[test]
fn test_degree_cent() {

}

#[test]
fn test_between_cent() {

}