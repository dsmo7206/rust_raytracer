mod camera;
mod config;
mod image;
mod input;
mod material;
mod object;
mod scene;
mod timer;

use input::Input;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use timer::Timer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_filename> <output_filename>", args[0]);
        std::process::exit(1);
    }

    let input_text = match read_file(&args[1]) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    let input: Input = match serde_json::from_str(&input_text) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error parsing input file: {}", e);
            std::process::exit(1);
        }
    };

    let image = {
        let _timer = Timer::new("Rendering scene");
        input.scene.render(&input.camera, &input.config)
    };

    {
        let _timer = Timer::new("Writing image");
        if let Err(e) = image.into_ppm(&args[2]) {
            // The only reason this can really fail is that the output_filename is invalid.
            // It would be nice to check whether that's the case before rendering the scene.
            eprintln!("Error writing output file: {}", e);
            std::process::exit(1);
        }
    }

    println!("Complete");
}

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
