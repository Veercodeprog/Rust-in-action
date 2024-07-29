#[derive(debug)]
struct file {
    name: string,
    data: vec<u8>,
}

fn main() {
    let f1 = file {
        name: string::from("f1.txt"),
        data: vec::new(),
    };
    let f1_name = &f1.name;
    let f1_data = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_data);
}
