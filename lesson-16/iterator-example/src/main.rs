fn main() {
    let result: Vec<i32> = vec![1, 2, 3]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        .take(2)
        .collect();

    println!("{:?}", result);
}
