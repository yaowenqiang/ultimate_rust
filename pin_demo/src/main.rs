struct Struct {
    field: String,
    // ...
}

impl Struct {
    fn field(self) -> String {
        self.field
    }
}

fn main() {
    let struct1 = Struct {
        field: "field1".to_string(),
    };
    println!("{}", struct1.field());
}
