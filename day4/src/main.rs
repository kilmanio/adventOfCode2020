use std::fs::read_to_string;

fn validate(raw: String) -> bool {
    
    let info: Vec<_> = raw
        .split_whitespace()
        .collect();

    //quickchecks
    //8 fields (or more?)
    if info.len() == 8 {
        return true
    }

    //less than 7 fields
    if info.len() < 7 {
        return false
    }

    let no_cid = info
        .iter()
        .filter(|f| {
             &f[..3] == "cid"
         })
        .count() == 0;


    return no_cid
}

fn format_string(info: String) -> String {
    info.replace("\n", " ")
}

fn main() {
    let content = read_to_string("input").unwrap();

    let valid_passports = content
        .split("\n\n")
        .filter(|l| validate(format_string(l.to_string())))
        .count();

    println!("Amount of valid passports in p1: {}", valid_passports)
}
