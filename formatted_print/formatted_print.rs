fn main() {
    print!("January has ");
    println!("{} days", 31); // {} is a placeholder for arguments

    println!("{0}, this is {1}. {1}, this is {0}",
             "Justin", "Rust");

    println!("{subject} {verb} {noun}",
             noun="rust",
             subject="Justin",
             verb="learns");

    println!("{} of {:b} people know binary, the other half don't",
             1, 2);

    println!("My name is {0}, {1} {0}", "Bond", "James");
}
