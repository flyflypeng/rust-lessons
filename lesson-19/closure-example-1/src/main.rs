use std::{collections::HashMap, mem::size_of_val};

fn main() {
    // c1长度为0
    let c1 = || println!("hello world");
    // c2长度为0
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("tyr");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    // 捕获一个引用，长度为8
    let c3 = || println!("hello: {}", name);
    // 捕获移动的数据：name1字符串：24字节，table哈希表：48字节，closure总共的长度：72字节
    let c4 = move || println!("hello: {}, {:?}", name1, table);
    let name2 = name.clone();
    // c5只捕获了一个String name2，所以闭包长度为24字节
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3)
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )
}
