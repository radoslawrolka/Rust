use std::io;

fn main() {
    println!("URL:");
    let url = get_url();
    let domain = get_domain(url);
    println!("Url domain: {}", domain);
}

fn get_url() -> String {
    let mut user_input = String::new();
    let _ = io::stdin().read_line(&mut user_input);
    user_input
}

fn get_domain(url: String) -> String {
    let domain = url.split("//").nth(1).unwrap().split("/").nth(0).unwrap();
    domain.to_string()
}