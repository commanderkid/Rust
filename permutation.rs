use std::io;

fn main() 
{
    let mut it_doesnt_need = String::new();
    let mut strin = String::new();
    io::stdin().read_line(&mut it_doesnt_need);
    io::stdin().read_line(&mut strin);
    /*
    let mut test_vec: Vec<u16> = strin
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    */
    let mut test_vec: Vec<u16> = [9, 3, 7].to_vec();
    let mut optimal: f32 = sum_vector_finder(&test_vec); // start - point
    let mut vec_vector: Vec<u16> = make_numbers(&test_vec);
    let mut ans: (Vec<u16>, f32) = (vec_vector.to_vec(), optimal);
    different_variants(test_vec, 0, optimal, vec_vector, &mut ans );
    print(&ans.0);
}

fn print(vect: &Vec<u16>)
{
    let border: usize = vect.len() - 1 as usize;
    for i in 0..border
    {
        print!("{} ", vect[i]);
    }
    print!("{}", vect[border]);
}

fn different_variants(mut input_array: Vec<u16>, point: usize, mut optimal: f32, mut vec_vector: Vec<u16>, ans: &mut (Vec<u16>, f32))
{
    let mut stop: f32 = 0.0;
    for i in point..input_array.len() as usize
    {
        if point == input_array.len() - 1 as usize
        {
            if optimal < ans.1
            {
                ans.0 = vec_vector;
                ans.1 = optimal;
            }
            break;
        }
        let new_point: usize = point + 1 as usize;
        if point == i && i != 0
        {
            different_variants(input_array.to_vec(), new_point, optimal, vec_vector.to_vec(), ans);
        }
        else
        {
            let temp_var: u16 = input_array[i];
            input_array[i] = input_array[point];
            input_array[point] = temp_var;
            let new_vector: Vec<u16> = input_array.to_vec();
            
            let mut stop: f32 = sum_vector_finder(&input_array[0..i]);
            if stop > optimal {break;}
            optimal = sum_vector_finder(&input_array[..]);
            let temp_vec_vector: u16 = vec_vector[i];
            vec_vector[i] = vec_vector[point];
            vec_vector[point] = temp_vec_vector;
            let new_vec_vector: Vec<u16> = vec_vector.to_vec();
            different_variants(new_vector, new_point, optimal, new_vec_vector, ans);
        }
    }
}

fn sum_vector_finder(input_array: &[u16]) -> f32
{
    let mut temp: u16 = 0;
    for i in 0..input_array.len() as usize
    {
        for j in 0..=i
        {
            temp += input_array[j];
        }
    }
    temp as f32 / input_array.len() as f32
}

fn make_numbers(len_of_array: &Vec<u16>) -> Vec<u16>
{
    let mut vec_tester = Vec::<u16>::new();
    for i in 0..len_of_array.len() as usize
    {
        vec_tester.push(i as u16);
    }
    vec_tester
}

