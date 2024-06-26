use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn write_file(path: &str) {
    let mut file = File::create(path).expect("Unable to create file!");
    let mut rng = rand::thread_rng();
    for _j in 0..=500 {
        let point = &rng.gen_range(-100_000_000..=100_000_000);
        let labeler = &rng.gen_range(0.0..=1.0);
        let label = if *labeler >= 0.5 { 1 } else { 0 };
        let s: String = format!("{} {}\n", point, label);
        file.write_all(s.as_bytes()).expect("Unable to create file");
    }
}

fn read_file(path: &str) -> Vec<(i32, i32)> {
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
    for p in data {
        let split = p.0;
        let mut correct = 0;
        for point in data {
            let predicted_label = if point.0 >= split { 1 } else { 0 };
            if predicted_label == point.1 {
                correct += 1
            };
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
    write_file("data.txt");
    let data = read_file("data.txt");
    let (split, accuracy) = find_best_split(&data);
    let first_5_entries = &data[..5];
    println!("For the following data.txt:");
    for entry in first_5_entries {
        println!("{:?}", entry)
    }
    println!("if x >= {}:", split);
    println!("  Predicted label is 1");
    println!("else:");
    println!("  Predicted label is 0");
    println!("Accuracy: {}", accuracy);
    println!("Complexity of O(n^2)");
}

#[test]
fn test_find_split_point() {
    let data = vec![(10, 1), (-10, 0), (3, 0), (4, 1), (-5, 1), (11, 0), (12, 1)];
    let (split, _accuracy) = find_best_split(&data);
    assert_eq!(split, 4, "Not finding the best split point!")
}
