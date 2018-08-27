extern crate image;

#[macro_use]
extern crate clap;

use image::GenericImage;

fn validate_file(file: &str) -> bool {
    // TODO: Add more validations like format, color, etc.
    if std::path::Path::new(file).is_file()
    {
        return true;
    } else {
        eprintln!("{}: Not a valid image file!", file);
        return false;
    }
}

fn rename_output_image(input_name: &str) -> String {
    format!("{}_pp.jpg", input_name)
}

fn scale_image(
    img_name: &str,
    out_name: &str,
    x: f32,
    y: f32
) -> () {
    let img = image::open(img_name).unwrap();
    let size_x = img.width();
    let size_y = img.height();
    let scaled_x = (size_x as f32 * x / 100.0 ) as u32;
    let scaled_y = (size_y as f32 * y / 100.0 ) as u32;
    println!("{}: {}% x {}%", img_name, x, y);
    println!("\t{} x {} => {} x {}", size_x, size_y, scaled_x, scaled_y);
    let img_scaled = img.resize_exact(scaled_x, scaled_y, image::FilterType::Nearest).save(out_name).unwrap();
    println!("\t{}", out_name);
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
        (@arg OUTPUTFILE: -o --outputfile +takes_value "Specify output file name. May only be used with single file input")
    ).get_matches();

    // Collect arguments as variables
    let input_files: Vec<&str> = matches.values_of("INPUTFILE").unwrap().collect();
    let scale_w: f32 = matches.value_of("widthscale").unwrap().parse().unwrap();
    let scale_h: f32 = matches.value_of("heightscale").unwrap().parse().unwrap();
    let output_file: String = matches.value_of("OUTPUTFILE")
        .unwrap_or(
            &rename_output_image(input_files[0])
        ).parse().unwrap();

    // Validate that files exist and are proper images
    let mut valid_files: Vec<&str> = Vec::new();
    for file in input_files.iter() {
        if validate_file(file) {
            valid_files.push(file);
        }
    }

    // Exit if no valid files
    if valid_files.len() < 1 {
        eprintln!("No valid files to scale!");
        std::process::exit(1);
    }

    // Loop through valid files and scale the images
    for valid_file in valid_files.iter() {
        scale_image(valid_file, &output_file, scale_w, scale_h,);
    }
}
