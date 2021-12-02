fn main() {
    println!("Hello, world!");

    let a: [i32; 3] = [4, 5, 6];
    println!("a: {:?}", a);
    let mut b = [7, 8, 9, 10, 11, 12, 13, 14, 15];
    println!("b: {:?}", b);
    // a = b;
    // println!("b: {:?}", b);
    {
        let c: [i32; 12] = [0; 12];
        println!("c: {:?}", c);
    }

    {
        let mut v = Vec::new();
        v.push(2);
        v.push(3);
        println!("v: {:?}, len: {:?}, cap: {:?}", v, v.len(), v.capacity());
        v.extend(a.iter());
        println!("v: {:?}, len: {:?}, cap: {:?}", v, v.len(), v.capacity());
        v.extend(b.iter());
        println!("v: {:?}, len: {:?}, cap: {:?}", v, v.len(), v.capacity());
    }

    {
        let sa = &a[..2];
        println!("sa: {:?}", sa);
        let sb = &mut b[2..];
        sb[3] = 22;
        println!("b: {:?}", b);
    }
    {
        let v2 = vec![3, 5, 3, 6, 2];
        println!("v2={:?}, len= {:?}, cap= {:?}", v2, v2.len(), v2.capacity());

        let vs = &v2[1..4];
        println!("vs={:?}, len={:?}", vs, vs.len());
    }
}
