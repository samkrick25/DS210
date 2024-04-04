use std::fs::File;
use std::io::prelude::*;

fn write_file(path: &str) -> {

}

fn read_file(path: &str) -> Vec<(i32, i32)>{
    let mut result: Vec<(i32, i32)> = Vec::new();
    let file = File::open(path).expect("Unable to open file!");
    let bufreader = std::io::BufReader::new(file).lines();
    for line in bufreader {
        let line_str = line.expect("Failed to read line");
        let v: Vec<&str> = line_str.trim().split(" ").collect();
        let x = v[0].parse::<i32>().unwrap();
        let y = v[1].parse::<i32>().unwrap();
        result.push((x, y));
    }
    result
}

fn find_best_split(data: &Vec<(i32, i32)>) -> (i32, f64) {
    let mut best_split_point = 0;
    let mut best_accuracy = 0.0;
    for point in data {
        let split = point.0;
        let mut correct = 0;
        for p in data {
            let predicted_label = if p.0 >= split {1} else {0};
            if predicted_label == p.1 {correct += 1};
        }
        let accuracy = correct as f64 / data.len() as f64;
        if accuracy > best_accuracy {
            best_accuracy = accuracy;
            best_split_point = split;
        }
    }
    (best_split_point, best_accuracy)
}

fn main() {
    let data = read_file("data.txt");
    let (split, accuracy) = find_best_split(&data);
    println!("if x >= {}:", split);
    println!("  Predicted label is 1");
    println!("else:");
    println!("  Predicted label is 0");
    println!("Accuracy: {}", accuracy);
    println!("Complexity of O(n^2)");
}
