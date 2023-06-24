use privilege;

fn main() {
    if privilege::user::privileged() {
        println!("I'm privileged! Running with root or suid privileges.");
    } else {
        println!("I'm not privileged! Running with regular user privileges.");
    }
}
