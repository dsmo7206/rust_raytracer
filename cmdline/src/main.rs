use clap::Clap;
use engine::{input::Input, timer::Timer};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clap)]
struct Opts {
    input_filename: String,
    output_filename: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let input_text = read_file(&opts.input_filename).unwrap_or_else(|err| {
        eprintln!("Error reading input file: {}", err);
        std::process::exit(1);
    });

    let input: Input = serde_json::from_str(&input_text).unwrap_or_else(|err| {
        eprintln!("Error parsing input file: {}", err);
        std::process::exit(1);
    });

    let image = {
        let _timer = Timer::new("Rendering scene");
        input.scene.render(&input.camera, &input.config)
    };

    {
        let _timer = Timer::new("Writing image");
        if let Err(e) = image.into_ppm(&opts.output_filename) {
            // The only reason this can really fail is that the output_filename is invalid.
            // It would be nice to check whether that's the case before rendering the scene.
            eprintln!("Error writing output file: {}", e);
            std::process::exit(1);
        }
    }
}

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
