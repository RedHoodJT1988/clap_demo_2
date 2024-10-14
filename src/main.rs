use clap::{Arg, Command};
// Subcommand
fn main() {
    
    let matches = Command::new("DemoTwo")
        .version("2.0")
        .author("Zenva Academy")
        .about("Image Processor")
        .subcommand(
            Command::new("resize")
            .about("Resizes an image")
            .arg(
                Arg::new("input")
                    .short('i')    
                    .long("input")
                    .value_name("FILE")
                    .help("Sets the input file path")
                    .required(true),
            )
            .arg(
                Arg::new("output")
                    .short('o')    
                    .long("output")
                    .value_name("FILE")
                    .help("Sets the output file path")
                    .required(true),
            )
            .arg(
                Arg::new("width")
                    .short('w')    
                    .long("width")
                    .value_name("WIDTH")
                    .help("Sets the new width")
                    .required(true),
            )
            .arg(
                Arg::new("height")
                    .short('h')    
                    .long("height")
                    .value_name("HEIGHT")
                    .help("Sets the new height")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("convert")
            .about("Converts an image")
            .arg(
                Arg::new("input")
                    .short('i')    
                    .long("input")
                    .value_name("FILE")
                    .help("Sets the input file path")
                    .required(true),
            )
            .arg(
                Arg::new("output")
                    .short('o')    
                    .long("output")
                    .value_name("FILE")
                    .help("Sets the output file path")
                    .required(true),
            )
            .arg(
                Arg::new("format")
                    .short('f')    
                    .long("format")
                    .value_name("FORMAT")
                    .help("Sets the output format (e.g., png, jpg)")
                    .required(true),
            )
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("resize", sub_matches)) => {
            // handle resize command
            let input = sub_matches.get_one::<String>("input").expect("Input file is required!");
            let output = sub_matches.get_one::<String>("output").expect("Output file is required!");
            let width = sub_matches.get_one::<String>("width").expect("Width is required!");
            let height = sub_matches.get_one::<String>("height").expect("Height is required!");
            println!("Resizing image {} to {}x{} and saving to {}", input, width, height, output);

            // Your resize logic would go here
        }
        Some(("convert", sub_matches)) => {
            // handle convert command
            let input = sub_matches.get_one::<String>("input").expect("Input file is required!");
            let output = sub_matches.get_one::<String>("output").expect("Output file is required!");
            let format = sub_matches.get_one::<String>("format").expect("Format is required!");
            println!("Converting image {} to {} and saving to {}", input, format, output);

            // Your convert logic would go here
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}