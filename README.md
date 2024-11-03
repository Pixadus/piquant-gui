# piquant-refresh

This repository will serve as the home of a refresh of the GUI designed to work with [piquant](https://github.com/pixlise/piquant), a program designed to perform quantitative x-ray fluorescence analysis on data from the Mars PIXL instrument. 

## Compiling

### OSX

Make sure you have the Rust toolchain installed; see [https://rustup.rs/](https://rustup.rs/) for information on your operating system. 

Once both of the above have been installed, use the following instructions to compile:

```
git clone https://github.com/Pixadus/piquant-refresh
cd piquant-refresh
cargo build
```

Or, to run the application,
```
cargo run
```