use rand::prelude::*; // brings common traits and types into scope from the rand crate
                      //
fn one_in(denominator: u32) -> bool {
    // helper function that returns true with a probability of
    // 1/denominator, this is a helper function that spreads sporadic errors.
    thread_rng().gen_ratio(1, denominator) //thread_rng() creates a thread-local random number
                                           //generator,gen_ratio(n, m) returns a booleand value with the probability of n/m
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        //first appearance of
        //Result(T,E)
        let mut temp = self.data.clone();
        let read_size = temp.len();
        save_to.reserve(read_size);
        save_to.append(&mut temp);
        Ok(read_size) // in this code read() never fails , but we still wrap read_size in Ok() to
                      // indicate success and also because we are using the Result type as a return type
    }
}
fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let error_message = String::from("Permission denied");
        return Err(error_message);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let error_message = String::from("Interrupted by signal!");
        return Err(error_message);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);
    let mut buffer: Vec<u8> = vec![];
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();
    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
