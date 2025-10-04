fn main() {
    let my_ves = vec![1, 2, 3, 4, 5];
    println!("my_ves: {}", my_ves[0]);
    // println!("my_ves: {}", my_ves[10]);

    if let Some(value) = my_ves.get(11) {
        println!("value: {}", value);
    } else {
        println!("no value");
    }
    unsafe {
        let value = my_ves.get_unchecked(11);
        println!("value: {}", value);
    }
    unsafe {
        my_fn();
    }
}

/// # Safety
///
/// This function is unsafe because ...
///
unsafe fn my_fn() {
    let my_ves = vec![1, 2, 3, 4, 5];
    let value = my_ves.get_unchecked(11);
    println!("value: {}", value);
}
