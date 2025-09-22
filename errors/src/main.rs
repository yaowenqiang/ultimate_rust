use std::path::Path;

fn main() {
    let my_file = Path::new("./myfile.txt");
    let content = std::fs::read_to_string(my_file);
    match content {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(err) => match err.kind() {
            // println!("ERROR:{err:#?}");
            std::io::ErrorKind::NotFound => {
                println!("File not found {my_file:#?}");
            }
            _ => {
                println!("Error! {err:#?}");
            }
        },
    }
}
