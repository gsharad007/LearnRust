fn main() {

    let s = String::from("Hello");
    let ss = &s[2..4];
    println!("s {} slice {}", s, ss);
    drop(s);

    let mut v = vec![1, 2, 3, 4];
    let vs = &v[1..];
    println!("vs {:?} slice {:?}", v, vs);
    v.push(5);

    println!("Hello, world!");
}
