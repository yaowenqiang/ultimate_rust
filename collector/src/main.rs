use shared_data::{DATA_COLLECTOR_ADDRESS, CollectorCommandV1};
use std::io::Write;
use std::sync::mpsc::Sender;
use std::time::Instant;
use uuid::Uuid;

fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        let uuid = Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}

pub fn collect_data(tx: Sender<CollectorCommandV1>, collector_id: u128) {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

    loop {
        let now = Instant::now();

        sys.refresh_memory();
        sys.refresh_cpu_all();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let num_cpus = sys.cpus().len();
        let total_cpu_usage = sys.cpus().iter().map(|x| x.cpu_usage()).sum::<f32>();
        let average_cpu_usage = total_cpu_usage / num_cpus as f32;

        let send_result = tx.send(CollectorCommandV1::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        });
        if let Err(e) = send_result {
            println!("Error sending data {e:?}");
        }

        let elapsed_seconds = now.elapsed().as_secs_f32();
        if elapsed_seconds < 1.0 {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
        } else {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }
    }
}

pub fn send_command(command: CollectorCommandV1) {
    let bytes = shared_data::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());

    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS).unwrap();
    stream.write_all(&bytes).unwrap();
}
fn main() {
    let uuid = get_uuid();
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();
    let _collector_thread = std::thread::spawn(move || {
        collect_data(tx, uuid);
    });

    while let Ok(command) = rx.recv() {
        send_command(command);
    }
}
