extern crate rand;
use rand::Rng;

fn main() {
    let mut array_ofrandom : [i32; 6] = [0, 0, 0, 0, 0, 0];
    let mut ansver = 0;
    loop {
        ansver = summ_of_array(&mut array_ofrandom);
        if ansver != 0 {
            break;
        }
    }
    println!("{}", ansver);
}

fn summ_of_array(arr : &mut[i32; 6]) -> i32 {
    let mut summ = 0;
    for i in 0..6 {
        arr[i] = rand::thread_rng().gen_range(0, 2);
        summ += arr[i];
    }
    summ
}
