#![allow(unused_variables)] //silence warnings

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length); // ensure that the save_to Vec has enough capacity to store the
                                  // data
    save_to.append(&mut tmp);
    read_length
}
fn main() {
    let mut f2 = File {
        name: String::from("f2.txt"),
        data: vec![114, 117, 115, 116],
    };
    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer); //converts Vec<u8> to String.any string with
                                                 //invalid UTF-8 sequences will be replaced with U+FFFD REPLACEMENT CHARACTER i.e ?.

    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("buffer is {}", text);
}
