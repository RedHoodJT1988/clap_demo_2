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
}