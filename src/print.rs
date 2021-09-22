pub fn run() {
    // Print to console
    println!("Hello from print.rs");

    // Basic formatting
    println!("Number: {}",1);
    println!("{} is from {}", "Brad", "Mass");

    // Positional args
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named args
    println!("{name} is from {place}", name="Brad", place="Washington");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // debug
    println!("{:?}", (12, true, "hello"));

    // math
    println!("10 + 10 = {}", 10+10);
}