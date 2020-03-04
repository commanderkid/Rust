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


// Shen' - 1.1.4
fn main()
{
    println!("{}", function_loop(5, 1, 3));
}

fn function_loop(mut k: i32, mut b: i32, mut c: i32) -> i32
{
    let mut count_of_steps: i32 = 0;
    while k != 0
    {
        if k % 2 == 0
        {
            k = k / 2;
            c = c * c;
        }
        else
        {
            k = k - 1;
            b = b * c;
        }
        count_of_steps += 1;
    }
    count_of_steps
}

// Shen' - 1.1.5

// a, b => a * b use (+, -, =, !=)

fn main()
{
    println!("{}", calculator_a_b(5, 5));
}

fn calculator_a_b(a: u32, b: u32) -> u32
{
    let mut k = 0u32;
    for i in 0..b
    {
        k += a;
    }
    k
}
