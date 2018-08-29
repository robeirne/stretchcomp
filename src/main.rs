extern crate image;

#[macro_use]
extern crate clap;

use image::GenericImage;
use std::path::{Path,PathBuf};

fn validate_file(file: &str) -> bool {
    // TODO: Add more validations like format, color, etc.
    if Path::new(file).is_file()
    {
        return true;
    } else {
        eprintln!("{}: Not a valid image file!", file);
        return false;
    }
}

fn get_output_name(input_path: &Path, dir: &str, file: &str) -> PathBuf {
    let mut output_file = PathBuf::new();

    // Use specified directory if specified by output
    if dir.len() > 0 {
        output_file.push(PathBuf::from(dir));
    }
    
    // Use file name if specified by args
    if file.len() > 0 {
        output_file.push(PathBuf::from(file));
    } else {
        output_file.push(input_path.file_name().unwrap());
        output_file.set_extension("jpg");
    }

    // Make output directory if it doesn't exist
    let output_parent = PathBuf::from(output_file.parent().unwrap());
    if ! output_parent.exists() {
        std::fs::create_dir_all(output_parent).unwrap();
    }

    // Notify if output file already exists
    if output_file.exists() {
        eprintln!("{} exists. Overwriting...", output_file.display());
    }

    return output_file
}

fn scale_image(
    img_name: &Path,
    out_name: &Path,
    x: f32,
    y: f32
    ) -> () {
    let img = image::open(img_name).unwrap();
    let size_x = img.width();
    let size_y = img.height();
    let scaled_x = (size_x as f32 * x / 100.0 ) as u32;
    let scaled_y = (size_y as f32 * y / 100.0 ) as u32;
    println!("{}: {}% x {}%", img_name.display(), x, y);
    println!("\t{} x {} => {} x {}", size_x, size_y, scaled_x, scaled_y);
    let img_scaled = img.resize_exact(scaled_x, scaled_y, image::FilterType::Nearest).save(out_name).unwrap();
    println!("\t=> {}", out_name.display());
    return img_scaled
}

fn main() {
    // Parse command line arguments
    let matches = clap_app!(stretchcomp =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg widthscale: -x --width +takes_value +required "Specify width scaling percentage on a scale from 0-100 (e.g. 105).")
        (@arg heightscale: -y --height +takes_value +required "Specify height scaling percentage on a scale from 0-100 (e.g. 105).")
        (@arg INPUTFILE: +required +multiple "Specify input file name(s).")
        (@arg OUTPUTDIR: -O --outputdir +takes_value "Specify output directory.")
        (@arg OUTPUTFILE: -o --outputfile +takes_value "Specify output file name. May only be used with single file input")
    ).get_matches();

    // Collect arguments as variables
    let input_files: Vec<&str> = matches.values_of("INPUTFILE").unwrap().collect();
    let scale_w: f32 = matches.value_of("widthscale").unwrap().parse().unwrap();
    let scale_h: f32 = matches.value_of("heightscale").unwrap().parse().unwrap();
    let spec_output_dir  = matches.value_of("OUTPUTDIR").unwrap_or("");
    let spec_output_file = matches.value_of("OUTPUTFILE").unwrap_or("");

    // Validate that files exist, are proper images, and add to Vec
    let mut valid_files: Vec<&Path> = Vec::new();
    for file in input_files.iter() {
        if validate_file(file) {
            valid_files.push(Path::new(file));
        }
    }

    // Exit if no valid files
    if valid_files.len() < 1 {
        eprintln!("No valid files to scale!");
        std::process::exit(1);
    }

    // Loop through valid files and scale the images
    for valid_file in valid_files.iter() {
        let output_file = get_output_name(&valid_file, &spec_output_dir, &spec_output_file);
        assert_ne!(&output_file, valid_file, "Input and output files are the same!");
        scale_image(valid_file, &output_file, scale_w, scale_h,);
    }
}
