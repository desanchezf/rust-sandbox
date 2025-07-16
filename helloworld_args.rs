fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: Se requiere exactamente un argumento.");
        std::process::exit(1);
    }
    println!("Hello, world {}!", args[1]);
}