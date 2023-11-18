trait MyTrait {
    fn method(&self);
}

struct TypeA(u32);
struct TypeB(String);
struct TypeC(bool);

impl MyTrait for TypeA {
    fn method(&self) {
        println!("TypeA method: {}", self.0);
    }
}

impl MyTrait for TypeB {
    fn method(&self) {
        println!("TypeB method: {}", self.0);
    }
}

impl MyTrait for TypeC {
    fn method(&self) {
        println!("TypeC method: {}", self.0);
    }
}

fn main() {
    let vec: Vec<Box<dyn MyTrait>> = vec![
        Box::new(TypeA(42)),
        Box::new(TypeB(String::from("Hello"))),
        Box::new(TypeC(true)),
    ];

    for item in &vec {
        item.method();
    }
}