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

Once `rust` is installed, navigate to the project directory and run:
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
    -y, --height <heightscale>       Specify height scaling percentage on a scale from 0-100 (e.g. 105.
    -x, --width <widthscale>         Specify width scaling percentage on a scale from 0-100 (e.g. 105.

ARGS:
    <INPUTFILE>...    Specify input file name(s).
```

## Examples
Scale a JPG by 103% in width and  108% in height and convert to PNG:
```
stretchcomp -x 103 -y 108 inputfile.jpg -o outputfile.png
```

## Notes
- If no output file is specified, the output will go to the same location as the input file with `_pp.jpg` appended to the file name.

## Known Issues
- If the file extension provided does not match the actual format of the file, the program will panic at runtime.
- The dependant `image` crate does not yet support writing to TIFFs.