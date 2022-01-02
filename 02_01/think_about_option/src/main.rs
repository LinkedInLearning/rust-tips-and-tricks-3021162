#[derive(Debug)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: String,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: String) -> Invitation {
        Invitation {
            invitee,
            attending,
            message,
        }
    }
}

fn main() {

}
