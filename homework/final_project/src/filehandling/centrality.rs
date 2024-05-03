use super::{ArticleMap, EdgeListStr, ArticleID, AdjacencyList};
use std::collections::{HashSet, VecDeque};
//TODO: ADD COMMENTS
//TODO: make bfs, and calculate shortest path and predecessors through that, then modify functions to calculate shortest path for all pairs of nodes, 

pub fn get_degrees(edges: &EdgeListStr, articles: &mut ArticleMap) {
    for (edge1, edge2) in edges {
        dbg!(&edge1, &edge2);
        let (_, count1, _, _) = articles.get_mut(edge1).unwrap();
        *count1 += 1;
        let (_, _, count2, _) = articles.get_mut(edge2).unwrap();
        *count2 += 1;
    }
}

pub fn calc_degrees(articles: &mut ArticleMap) {
    let length = articles.len();
    for (_, degrees) in articles.iter_mut() {
        dbg!(&degrees);
        degrees.1 = degrees.1 / length;
        degrees.2 = degrees.2 / length;
    }
}

pub fn reconstruct_shortest_path(
    pred: &Vec<Option<usize>>, 
    start: usize, 
    end: usize,
) -> Option<Vec<usize>> {
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
    Some(path)
}

pub fn bfs_predecessors(adjacency_list: &AdjacencyList, start: usize, article_id: &ArticleID) -> Vec<Option<usize>> {
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
) {
    let connected_components = find_components(adjacency_list);

    for component in connected_components {
        println!("looking at component");
        for start in &component {
            println!("looking at start");
            for end in &component {
                println!("end");
                if start != end {
                    let pred = bfs_predecessors(&adjacency_list, *start, &article_id);
                    let path = reconstruct_shortest_path(&pred, *start, *end);
                    println!("{:?}", path);
                    let length = path.clone().unwrap().len();
                    for node in path.unwrap().iter().skip(1).take(length - 1) {
                        let article_name = article_id.get(node);
                        let (_, _, _, between) = article_map.get_mut(article_name.unwrap()).unwrap();
                        *between += 1.0;
                        println!("Betweenness added");
                    }
                }
            }
        }
        let normal_factor = (component.len() - 1) as f64 * (component.len() - 1) as f64 / 2.0;
        for node in &component {
            let article_name = article_id.get(node);
            let (_, _, _, betweenness) = article_map.get_mut(article_name.unwrap()).unwrap();
            *betweenness /= normal_factor;
        }
    }
}

fn dfs(
    node: usize,
    adjacency_list: &AdjacencyList,
    visited: &mut HashSet<usize>,
    component: &mut Vec<usize>,
) {
    // Mark the current node as visited
    visited.insert(node);
    component.push(node);
    // Visit all unvisited neighbors
    if adjacency_list.contains_key(&node) {
        println!("were in");
        for &neighbor in &adjacency_list[&node] {
            if !visited.contains(&neighbor) {
                dfs(neighbor, adjacency_list, visited, component);
            }
        }
    }
}

fn find_components(adjacency_list: &AdjacencyList) -> Vec<Vec<usize>> {
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