// case 1
// fn local_ref<'a>() -> &'a i32 {
//     let a = 42;
//     &a
// }

// fn main() {
//     let r = local_ref();
//     println!("r: {:p}", r);
// }

// case 2
// fn main() {
//     let mut data = Vec::new();
//     let v = 42;
//     data.push(v);
//     println!("data: {:?}", data);
// }

// case 3
fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    data.push(&v);
}

fn main() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
}
