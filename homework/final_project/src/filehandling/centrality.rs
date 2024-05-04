use super::{ArticleMap, EdgeListStr, ArticleID, AdjacencyList};
use std::collections::{HashSet, VecDeque};
use std::time::SystemTime;
//TODO: ADD COMMENTS
//TODO: write each component in the betweenness calculation in parallel

pub fn get_degrees(edges: &EdgeListStr, articles: &mut ArticleMap) {
    for (edge1, edge2) in edges {
        let (_, _, outdegree, _, _) = articles.get_mut(edge1).unwrap();
        *outdegree += 1;
        let (_, _, _, indegree, _) = articles.get_mut(edge2).unwrap();
        *indegree += 1;
    }
}

pub fn calc_degrees(articles: &mut ArticleMap) {
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
    let mut count = 0;
    for component in connected_components {
        let before = SystemTime::now();
        count += 1;
        for start in &component {
            let article_name = article_id.get(start).unwrap();
            let (_, component_id, _, _, _) = article_map.get_mut(article_name).unwrap();
            *component_id = count;
            // let before = SystemTime::now();
            // let mut count = 0;
            for end in &component {
                if start != end {
                    let pred = bfs_predecessors(&adjacency_list, *start, &article_id);
                    let path = reconstruct_shortest_path(&pred, *start, *end);
                    let length = &path.len();
                    for node in path.iter().skip(1).take(length - 1) {
                        let article_name = article_id.get(node).unwrap();
                        let (_, _, _, _, between) = article_map.get_mut(article_name).unwrap();
                        *between += 1.0;
                    }
                }
            }
            // let after = SystemTime::now();
            // let difference = after.duration_since(before).unwrap();
            // println!("startpoint calculated in {:?}", difference);
            // count += 1;
            // println!("startpoint #{}", count);
        }
        let normal_factor = (component.len() - 1) as f64 * (component.len() - 1) as f64 / 2.0;
        for node in &component {
            let article_name = article_id.get(node);
            let (_, _, _, _, betweenness) = article_map.get_mut(article_name.unwrap()).unwrap();
            *betweenness /= normal_factor;
        }
        let after = SystemTime::now();
        let difference = after.duration_since(before).unwrap();
        println!("component calculated and normalized in {:?}", difference);
        println!("Component #{}", count);
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