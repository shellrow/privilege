use privilege;

fn main() {
    if privilege::user::privileged() {
        println!("I'm privileged! Running as root or suid.");
    } else {
        println!("I'm not privileged! Running as a normal user.");
    }
}
