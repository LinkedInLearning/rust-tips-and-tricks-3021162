fn third_party_api(username: &str, password: &str) -> String {
    /* Makes an outbound request to connet to the
     * server
     */

    let mut access_token = username.to_string();
    access_token.push_str(password.split_at(2).0);

    access_token
}

fn main() {
    let username = env!("USERNAME");
    let password = option_env!("PASSWORD").expect("Need to set the 'PASSWORD' environment variable");

    println!("Access Token: {}", third_party_api(&username, &password))
}
