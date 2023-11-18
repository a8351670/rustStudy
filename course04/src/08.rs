trait MyTrait {
    fn method(&self);
}

struct MyType(u32);

impl MyTrait for MyType {
    fn method(&self) {
        println!("MyType method: {}", self.0);
    }
}

impl std::ops::Add<MyType> for MyType {
    type Output = MyType;

    fn add(self, other: MyType) -> MyType {
        MyType(self.0 + other.0)
    }
}

fn main() {
    let a: Box<dyn MyTrait> = Box::new(MyType(5));
    let b: Box<dyn MyTrait> = Box::new(MyType(10));

    let result = a.method() + b.method();
}