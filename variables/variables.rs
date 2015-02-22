fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;
    let an_integer = 3; // I can reassign these values

    println!("An integer: {:?}", an_integer);
    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("What is a unit?: {:?}", unit);

    // why silence unused variables?
    let _no_warnings = 3u32;
    let _not_so_noisy_unused_variable = 2u32;
}


