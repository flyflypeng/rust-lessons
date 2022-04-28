fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    return f(value);
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn main() {
    println!("apply squar: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
}
