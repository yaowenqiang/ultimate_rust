fn double(n:i32) -> i32 {
    n * 2
}

// fn greate (s: String) {
//     println!("hello {}", s);
// }
fn greet(s: String) -> String{
    println!("hello {}", s);
    s
}

fn greet_borrow(s: &String) {
    println!("{s}");
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("mut after fn {s}");
    println!("{s}");
}
fn double_or_nothing(n:i32) -> i32 {
    if n > 0 {
        return n * 2
    } 
    0
}

fn main() {
    let mut n = 5;
    n += 6;
    println!("{n}");
    let n = {
        6
    };

    println!("{n}");
    let n = {
        let n = 6;
    };
    println!("{n:?}");

    let n3 = 1;
    let n2 = double(n3);
    println!("{}", n2);

    let i = 5;
    let m = if i  == 5 {
        6
    } else {
        7
    };
    println!("{}", m);

    let name = "hello".to_string();
    let name = greet(name);
    greet(name.clone());
    // greet(name.clone());

    greet_borrow(&name);

    let mut name_mut = "mut_hello".to_string();
    println!("mut before fn: {name_mut}");
    greet_borrow_mut(&mut name_mut);
    println!("{name_mut}");
    greet(name_mut.clone());

}
