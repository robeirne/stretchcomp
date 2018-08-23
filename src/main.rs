extern crate image;

#[macro_use]
extern crate clap;

use image::GenericImage;

fn validate_file(file: &str) -> bool {
    if std::path::Path::new(file).is_file()
    {
        return true;
    } else {
        return false;
    }
}

fn main() {
    // Parse command line arguments
    let matches = clap_app!(stretchcomp =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg widthscale: -w --width +takes_value +required "Specify width scaling percentage.")
        (@arg heightscale: -h --height +takes_value +required "Specify height scaling percentage.")
        (@arg INPUTFILE: +required +multiple "Specify input file name(s).")
        (@arg OUTPUTFILE: -o --outputfile +takes_value "Specify output file name. May only be used with single file input")
    ).get_matches();

    let input_files: Vec<&str> = matches.values_of("INPUTFILE").unwrap().collect();
    let scale_w: f32 = matches.value_of("widthscale").unwrap().parse().unwrap();
    let scale_h: f32 = matches.value_of("heightscale").unwrap().parse().unwrap();

    // Validate that files exist
    let mut valid_files: Vec<&str> = Vec::new();
    for file in input_files.iter() {
        if validate_file(file) {
            valid_files.push(file);
        }
    }

    for valid_file in valid_files.iter() {
        let out: String = matches.value_of("OUTPUTFILE").unwrap_or(&format!("{}_pp.jpg", valid_file)).parse().unwrap();
        let img = image::open(valid_file).unwrap();
        let size_x = img.width();
        let size_y = img.height();
        let scaled_x = (size_x as f32 * scale_w / 100.0 ) as u32;
        let scaled_y = (size_y as f32 * scale_h / 100.0 ) as u32;
        println!("{}: {}% x {}%", valid_file, scale_w, scale_h);
        println!("\t{} x {} => {} x {}", size_x, size_y, scaled_x, scaled_y);
        img.resize_exact( scaled_x, scaled_y, image::FilterType::Nearest).save(&out).unwrap();
        println!("\t=> {}", &out);
    }
}
