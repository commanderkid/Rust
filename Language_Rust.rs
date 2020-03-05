// Vectors
fn main()
{
    let mut odd_vec: Vec<u32> = finde_odd_numbers([1u32, 2, 3, 4, 5, 6, 7].to_vec());
    println!("{:#?}", odd_vec);
}

fn finde_odd_numbers(vector : Vec<u32>) -> Vec<u32>
{
    let mut ret_vec : Vec<u32> = Vec::<u32>::new();
    for item in vector
    {
        if item % 2 == 0
        {
            ret_vec.push(item);
        }
    }
    ret_vec
}
