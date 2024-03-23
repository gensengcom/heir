use heir::SessionBinary;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_file> <output_file>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let file = File::open(input_file).expect("Failed to open input file");
    let mut reader = BufReader::new(file);

    let heir_binary_file =
        SessionBinary::read_from_file(&mut reader).expect("Failed to read binary file");

    heir_binary_file
        .transpile_to_heir(output_file)
        .expect("Failed to transpile to .heir file");

    println!("Transpilation completed successfully!");
}
