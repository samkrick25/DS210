fn main() {
    const N:usize = 101;
    let mut a:[u8;N] = [0;N];
    //set initial values to the first fib numbers, then add previous entries to fill the next index
    a[0] = 0;
    a[1] = 1;
    for i in 2..N {
        a[i] = a[i-1] + a[i-2]
    }

    for i in 0..N {
        println!("F[{}]: {}", i, a[i])
    }
}
