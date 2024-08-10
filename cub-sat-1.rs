#![allow(unused_variables)]
#[derive(Debug)]
enum StatusMessage {
    Ok,
}
fn check_status(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = 1;
    let sat_b = 2;
    let sat_c = 3;

    let status_a = check_status(sat_a);
    let status_b = check_status(sat_b);
    let status_c = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", status_a, status_b, status_c);
}
