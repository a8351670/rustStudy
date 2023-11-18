enum MyEnum {
    TypeA(u32),
    TypeB(String),
    TypeC(bool),
}

impl MyEnum {
    fn method_a(&self) {
        if let MyEnum::TypeA(val) = self {
            println!("TypeA method: {}", val);
        }
    }

    fn method_b(&self) {
        if let MyEnum::TypeB(val) = self {
            println!("TypeB method: {}", val);
        }
    }

    fn method_c(&self) {
        if let MyEnum::TypeC(val) = self {
            println!("TypeC method: {}", val);
        }
    }
}

fn main() {
    let vec: Vec<MyEnum> = vec![
        MyEnum::TypeA(42),
        MyEnum::TypeB(String::from("Hello")),
        MyEnum::TypeC(true),
    ];

    for item in &vec {
        item.method_a();
        item.method_b();
        item.method_c();
    }
}