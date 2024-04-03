use std::time::SystemTime;
fn main() {
    for k in 0..51 {
        let before = SystemTime::now();
        println!("k: {}", k);
        let k_fib = fib(k);
        println!("F_k at {}: {}", k, k_fib);
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("Time it took: {:?}", difference); 
    }
}

fn fib(k: u32) -> u128 {
    if k == 0 {
        0
    }
    else if k == 1 {
        1
    }
    else {
        fib(k-1)+fib(k-2)
    }
}