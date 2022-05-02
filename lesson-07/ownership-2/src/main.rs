fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = data.clone();
    println!("sum of data1: {}", sum(data1));
    // println!("data1: {:?}", data1);
    println!("sum of data: {}", sum(data));
}
