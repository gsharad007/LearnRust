fn main() {
    let y = get_y();
    println!("Hello, #{}!", y);

    let t = BinTree {
        data: 3,
        left : Some(Box::new(BinTree {
            data: 5,
            left: None,
            right : None,
        })),
        right : None,
    };

    println!("BinTree: {:?}", t);
}

fn get_y() -> Box<i32> {
    let x = 32;
    Box::new(x)   
}

#[derive(Debug)]
struct BinTree<T> {
    data:T,
    left:Option<Box<BinTree<T>>>,
    right:Option<Box<BinTree<T>>>,
}