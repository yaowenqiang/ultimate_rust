use std::sync::Arc;

struct MyStruct {
    n: i32,
}

struct HasDroppables {
    x: MyStruct,
}

impl MyStruct {
    pub fn new(n: i32) -> Self {
        println!("constructing {n}");
        Self { n }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("dropping {}", self.n);
    }
}

fn move_me(x: MyStruct) {}

fn main() {
    let x = MyStruct::new(5);
    {
        let y = MyStruct::new(2);
    }
    move_me(x);
    println!("back from function");

    let has_drop = HasDroppables {
        x: MyStruct::new(10),
    };
    println!("Ending the main function");
}
