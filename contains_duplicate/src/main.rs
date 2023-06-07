
trait MyInterface {
    fn get<T>() -> T;
}

struct MyStruct {
    name: MyName
}

struct MyName {
    pub name: String,
}

impl MyInterface for MyStruct {
    fn get<MyName>() -> MyName {
        MyName {
            name: "jack".to_string(),
        }
    }
}

fn main() {
    
}