use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    let mut file = File::create("data.txt").expect("Unable to create file!");
    let mut rng = rand::thread_rng();
    for _j in 0..=500 {
        let point = &rng.gen_range(-100_000_000..=100_000_000);
        let labeler = &rng.gen_range(0.0..=1.0);
        let label = if *labeler >= 0.5 {1} else {0};
        let s: String = format!("{} {}\n", point, label);
        file.write_all(s.as_bytes()).expect("Unable to create file");
    }
}