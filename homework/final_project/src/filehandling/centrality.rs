use super::*;

fn get_degrees(edges: &EdgeList, articles: &mut ArticleMap) {
    for (edge1, edge2) in edges {
        let count1 = articles.entry(edge1);
        let count2 = articles.entry(edge2);
        *count1.0 += 1;
        *count2.1 += 1;
    }
}

fn calc_degrees(articles: &mut ArticleMap) {
    for (_, degrees) in articles.iter_mut() {
        degrees.0 = degrees.0/articles.len();
        degrees.1 = degrees.1/articles.len();
    }
}

fn calc_betweenness(articles: ArticleMap) {

}