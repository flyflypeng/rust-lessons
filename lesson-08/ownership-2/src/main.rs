fn sum(data: &Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;

    println!(
        "addr of value: {:p}({:p}), addr o data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );

    println!("sum of data1: {}", sum(data1));

    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}
