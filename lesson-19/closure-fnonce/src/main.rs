fn main() {
    let name = String::from("Tyr");
    let c = move |greeting: String| (greeting, name);

    let result = c("hello".to_string());

    println!("result: {:?}", result);

    //let result = c("hi".to_string());
}
