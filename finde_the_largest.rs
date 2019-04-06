fn main() {
    let mut arr : [u8; 8] = [1, 2, 3, 4, 5, 1, 7, 1];
    println!("{}", finde_largest(&mut arr));
}


fn finde_largest(arr: &mut[u8]) -> u8 {
    let mut max_val : u8 = arr[0];
    for i in arr.iter(){
        if max_val < *i {
            max_val = *i; 
        }
    }
    max_val
}

#[test]
fn test_find_largest(){
    let mut arr : [u8; 3] = [1, 2, 3];
    assert_eq!(finde_largest(&mut arr), 3);
}
