//using global variables to propagate error information

use rand::random; // bring the rand crate into the local scope
static mut ERROR: i32 = 0; // global variable to store error information
struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        } // setting error to 1 means error occurred
    }
    0 //always reads 0 bytes
}

#[allow(unused_mut)] // suppress the warning for unused mutable variable
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        // accessing static mut variables is an unsafe operation :w

        if ERROR != 0 {
            println!("An error has occurred");
        }
    }
}
