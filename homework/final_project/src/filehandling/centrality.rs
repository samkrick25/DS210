use super::{ArticleMap, EdgeListStr, ArticleID, ShortestPathsMat, AdjacencyList};
//TODO: ADD COMMENTS

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

// pub fn calc_betweenness(shortest_paths: &ShortestPathsMat, article_map: &mut ArticleMap, article_id: &ArticleID) {
//     let length = article_map.len();
//     for node in 0..length {
//         let mut betweenness = 0;
//         for i in 0..length {
//             for j in 0..length {
//                 if let (Some(total_dist), Some(between_dist_1), Some(between_dist_2)) = 
//                 (shortest_paths[i][j], shortest_paths[i][node], shortest_paths[node][j]) {
//                     //dbg!(&shortest_paths[i][j], &shortest_paths[i][node], &shortest_paths[node][j]);
//                     if total_dist == between_dist_1 + between_dist_2 {
//                         betweenness += 1;
//                     }
//                 } else {
//                     continue
//                 }
//             }
//         }
//         let article = article_id.get(&node).unwrap();
//         let (_, _, _, between_cent) = article_map.get_mut(article).unwrap();
//         *between_cent += betweenness;
//     }
// }

pub fn reconstruct_shortest_path(
    adjacency_list: &AdjacencyList, 
    shortest_paths: &ShortestPathsMat, 
    start: usize, 
    end: usize,
) -> Vec<usize> {
    let mut path = Vec::new();
    let mut current = end;
    path.push(current);
    while current != start {
        for &neighbor in &adjacency_list[&current] {
            println!("looking at neighbors");
            if shortest_paths[start][neighbor] != None && 
            shortest_paths[neighbor][current] != None && 
            shortest_paths[start][current] != None {
                if shortest_paths[start][neighbor].unwrap() + 
                shortest_paths[neighbor][current].unwrap() == shortest_paths[start][current].unwrap() {
                    path.push(neighbor);
                    current = neighbor;
                    println!("path pushed");
                    break;
                }
            }
        }
    }
    path.reverse();
    path
}

pub fn calculate_betweenness_centrality(
    adjacency_list: &AdjacencyList, 
    shortest_paths: &ShortestPathsMat, 
    article_map: &mut ArticleMap,
    article_id: &ArticleID,
) {
    let num_nodes = article_map.len();
    for start in 0..num_nodes {
        println!("Heyo");
        for end in 0..num_nodes {
            if start != end {
                let path = reconstruct_shortest_path(adjacency_list, shortest_paths, start, end);
                for node in path.iter().skip(1).take(path.len() - 1) {
                    let article_name = article_id.get(node);
                    let (_, _, _, between) = article_map.get_mut(article_name.unwrap()).unwrap();
                    *between += 1.0;
                    println!("Betweenness added");
                }
            }
        }
    }
    let normal_factor = (num_nodes - 1) as f64 * (num_nodes - 1) as f64 / 2.0;
    for (_, _, _, betweenness) in article_map.values_mut() {
        *betweenness /= normal_factor;
    }
}
