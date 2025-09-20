use dashmap::DashMap;
use once_cell::sync::Lazy;

static SHARED: Lazy<DashMap<u32, u32>> = Lazy::new(|| DashMap::new());
fn main() {
    for n in 1..=100 {
        std::thread::spawn(move || {
            loop {
                if let Some(mut v) = SHARED.get_mut(&n) {
                    *v += 1;
                } else {
                    SHARED.insert(n, n);
                }
            }
        });
    }
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("{SHARED:#?}");
}
