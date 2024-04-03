use std::io;

// https://example.com/path/to/resource?param1=value1&param2=value2
fn main() {
    println!("URL:");
    let url = get_url();
    let (domain, path, params) = decompose_url(url);
    println!("Url domain: {}", domain);
    println!("Url path: {}", path);
    println!("Url params: {}", params);
}

fn get_url() -> String {
    let mut user_input = String::new();
    let _ = io::stdin().read_line(&mut user_input);
    user_input
}

fn decompose_url(url: String) -> (String, String, String) {
    let domain = url.split("//").nth(1).unwrap().split("/").nth(0).unwrap();
    let path = url.split("//").nth(1).unwrap().split("?").nth(0).unwrap();
    //remove domain from path
    let path = path.split("/").skip(1).collect::<Vec<&str>>().join("/");
    let params = url.split("?").nth(1).unwrap();
    (domain.to_string(), path.to_string(), params.to_string())
}