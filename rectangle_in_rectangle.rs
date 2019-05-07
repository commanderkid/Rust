struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn square(&self) -> u32 {
        self.width * self.height
    }
    
    fn is_it_in(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main(){
    let rec = Rectangle {width : 1, height : 2};
    let rec2 = Rectangle {width : 1, height : 1};
    println!("{}", &rec.square());
    println!("{}", &rec.is_it_in(&rec2));
}
