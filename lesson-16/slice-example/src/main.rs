use std::fmt;

fn print_slice<T: fmt::Debug + ?Sized>(s: &T) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

fn main() {
    let v = vec![1, 2, 3, 4];

    print_slice(&v);
    print_slice(&v[..]);

    print_slice1(&v);
    print_slice1(&v[..]);
    print_slice1(v);

    let arr = [1, 2, 3, 4];
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}
