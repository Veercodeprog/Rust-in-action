//Defining the bare bones of a Read trait for File
#![allow(unused_variables)]
#[derive(Debug)]
struct File;

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>; //a trait block includes the
                                                                    //type signatures of the function that implementors must  comply with .The pseudo type Self is
                                                                    //a placeholder for the type that will implement the trait
}
impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let file = File {};
    let mut buffer = vec![];

    let n_bytes = file.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, file);
}
