use std::alloc::{Layout, alloc, dealloc};
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

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout,
}

impl<T> SmartPointer<T> {
    pub fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");
        unsafe {
            let layout = Layout::new::<T>();
            let ptr: *mut u8 = alloc(layout);
            SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout,
            }
        }
    }
    fn set(&mut self, val: T) {
        unsafe {
            *self.data = val;
        }
    }

    fn get(&self) -> &T {
        unsafe { self.data.as_ref().unwrap() }
    }
}

impl<T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Deallocating memory for SmartPointer");
        unsafe {
            dealloc(self.ptr, self.layout);
        }
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

    let mut my_num: SmartPointer<i32> = SmartPointer::<i32>::new();
    my_num.set(1);
    println!("my_num = {}", my_num.get());

    let my_num = Box::new(12);
    println!("my_num = {}", my_num);
}
