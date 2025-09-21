fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();
    let handles = core_ids
        .into_iter()
        .map(|core_id| {
            std::thread::spawn(move || {
                let success = core_affinity::set_for_current(core_id);
                if success {
                    println!("Hello from a thread on core {:?}", core_id);
                } else {
                    println!("unable to  set affinity on core {:?}", core_id);
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}
