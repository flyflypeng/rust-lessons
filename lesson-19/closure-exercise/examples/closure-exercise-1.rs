fn main() {
    let name = String::from("Tyr");
    let vec = vec!["Rust", "JavaScript", "Elixir"];
    let v = &vec[..];
    let data = (1, 2, 3, 4);
    let c = move || {
        println!("data: {:?}", data);
        println!("v: {:?}, name: {:?}", v, name.clone());
    };

    c();

    // println!("name: {:?}", name);
}
