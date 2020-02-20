//Shen' - 1.1.1.
fn main()
{
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    // Change a and b
    let t: i32 = a;
    a = b;
    b = t;
    println!("a: {}, b: {} then change a: {}, b: {}", b, a, a, b);
}


//Shen' 1.1.2
fn main()
{
    let (mut x, mut y) = (1, 2);
    x = y + x; // x = 3;
    y = x - y;
    x = x - y;
    println!("x: {}, y: {}", x, y);
}
