# stretchcomp
## Stretch Compensation
### Scale images in X and/or Y directions. Written in Rust.

## Build/Run
Requires `rustc` and `cargo`. On MacOS, run:
```
brew install rust
```
or
```
curl https://sh.rustup.rs -sSf | sh
```

Once `rust` is installed, navigate to this project directory and run:
```
cargo build --release
cargo run --release -- <INPUTFILE> -x <widthscale> -y <heightscale> [ -o <OUTPUTFILE> ]
```

## Install
```
cargo install
```

## Usage
```
USAGE:
    stretchcomp [OPTIONS] <INPUTFILE>... --height <heightscale> --width <widthscale>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --outputfile <OUTPUTFILE>    Specify output file name. May only be used with single file input
    -y, --height <heightscale>       Specify height scaling percentage on a scale from 0-100 (e.g. 105).
    -x, --width <widthscale>         Specify width scaling percentage on a scale from 0-100 (e.g. 105).

ARGS:
    <INPUTFILE>...    Specify input file name(s).
```

## Examples
Scale a JPG up by 3% in width and 8% in height and convert to PNG:
```
stretchcomp -x 103 -y 108 inputfile.jpg -o outputfile.png
```

Scale a BMP down by 5% in width and 2% in height and convert to JPG:
```
stretchcomp -x 95 -y 98 inputfile.bmp
```

## Notes
- If no output file is specified, the output will go to the same location as the input file with `_pp.jpg` appended to the file name. JPG is the default output format if none is specified with the `-o <OUTPUTFILE>` option.

## Known Issues
- If the file extension provided does not match the actual format of the file, the program will panic at runtime.
- The dependant [`image`](https://docs.rs/image/0.19.0/image/) crate does not yet support writing to TIFFs or processing CMYK pixel types.
- Writing to bitmaps (BMP) cannot exceed a certain number of pixels. Not sure what that number is exactly, but it's on the order of hundreds of megapixels.