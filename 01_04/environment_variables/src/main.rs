fn third_party_api(user: &str, password: &str) -> String {
    /* Makes an outbound request to connet to the
     * server
     */

    let mut access_token = user.to_string();
    access_token.push_str(password.split_at(2).0);

    access_token
}

fn main() {
    let user = "Elton".to_string();
    let password = "my_password".to_string();

    println!("Access Token: {}", third_party_api(&user, &password))
}
