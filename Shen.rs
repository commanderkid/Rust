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
