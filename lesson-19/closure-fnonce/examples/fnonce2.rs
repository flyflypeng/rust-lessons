fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "not closure".to_string())
}

fn main() {
    let name = String::from("Tyr");

    let c = move |greeting: String| (greeting, name.clone());

    println!("c1 call one: {:?}", c("qiao".to_string()));
    println!("c2 call one: {:?}", c("bonjour".into()));

    println!("result: {:?}", call_once("hi".into(), c));

    //let result = c("hola".into());

    println!("result: {:?}", call_once("nihao".into(), not_closure));
}
