use super::{ArticleMap, EdgeListStr, ArticleID, ShortestPathsMat};

//TODO: ADD COMMENTS

pub fn get_degrees(edges: &EdgeListStr, articles: &mut ArticleMap) {
    for (edge1, edge2) in edges {
        let (_, count1, _, _) = articles.get_mut(edge1).unwrap();
        *count1 += 1;
        let (_, _, count2, _) = articles.get_mut(edge2).unwrap();
        *count2 += 1;
    }
}

pub fn calc_degrees(articles: &mut ArticleMap) {
    let length = articles.len();
    for (_, degrees) in articles.iter_mut() {
        degrees.1 = degrees.1 / length;
        degrees.2 = degrees.2 / length;
    }
}

pub fn calc_betweenness(shortest_paths: &ShortestPathsMat, article_map: &mut ArticleMap, article_id: &ArticleID) {
    let length = article_map.len();
    for node in 0..length {
        let mut betweenness = 0;
        for i in 0..length {
            for j in 0..length {
                if let (Some(total_dist), Some(between_dist_1), Some(between_dist_2)) = 
                (shortest_paths[i][j], shortest_paths[i][node], shortest_paths[node][j]) {
                    if total_dist == between_dist_1 + between_dist_2 {
                        betweenness += 1;
                    }
                }
            }
        }
        let article = article_id.get(&node).unwrap();
        let (_, _, _, between_cent) = article_map.get_mut(article).unwrap();
        *between_cent += betweenness;
    }
}