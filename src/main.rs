extern crate image;

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(prepress =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg OUTPUTFILE: -o --output-file +takes_value "Specify output file name. May only be used with single file input")
        (@arg INPUTFILE: +required +multiple "Specify input file name(s).")
    ).get_matches();

    let input_files: Vec<&str> = matches.values_of("INPUTFILE").unwrap().collect();

    for file in input_files.iter() {
        use std::path::Path;
        let path_exists = Path::new(file).is_file();
        println!("FILE: {}\t EXISTS: {:?}", file, path_exists);
    }
}
