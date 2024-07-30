//Defining an enum and using it to parse an event log
#[derive(Debug)]
enum Event {
    //created three variants of Event using enum
    Update,
    Delete,
    Unknown,
}
type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line.splitn(2, ' ').collect(); // split the line into two parts
    if parts.len() == 1 {
        return (Event::Unknown, line.to_string());
    }
    let event = parts[0];
    let rest = String::from(parts[1]);
    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)), // can also use line.to_string()
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";
    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
