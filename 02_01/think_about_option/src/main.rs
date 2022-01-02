#[derive(Debug)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: Option<String>,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Invitation {
        Invitation {
            invitee,
            attending,
            message,
        }
    }
}

fn main() {
    let invitation_1 = Invitation::new("Dolores".to_string(), true, None);
    let invitation_2 = Invitation::new("Cosmo".to_string(), false, Some("Sorry. I am flying to Mars that week".to_string()));

    println!("{:#?}", invitation_1);
    println!("{:#?}", invitation_2);
}
