# piquant-gui

This repository will serve as the home of a refresh of the GUI designed to work with [piquant](https://github.com/pixlise/piquant), a program designed to perform quantitative x-ray fluorescence analysis on data from the Mars PIXL instrument. 

Please note that this is just a *GUI frontend* to the PIQUANT CLI application, available in `lib/cli`. 

## Compiling

### Linux & MacOSX

Make sure you have the Rust toolchain installed; see [https://rustup.rs/](https://rustup.rs/) for information on your operating system. 

Once both of the above have been installed, use the following instructions to compile:

```
git clone https://github.com/Pixadus/piquant-gui
cd piquant-gui
cargo run
```

To compile for release, append the release flag: `cargo build --release`. 

## Windows

Install Rust at [https://rustup.rs/](https://rustup.rs/); then, clone this repository either by downloading the zip, opening in [Github Desktop](https://desktop.github.com/download/) or using the Git cli interface.

Next, install the Microsoft Visual Studio Tools from [https://aka.ms/vs/17/release/vs_BuildTools.exe](https://aka.ms/vs/17/release/vs_BuildTools.exe). Select the "Desktop development with C++" block, then in the options panel on the right select:

- MSVC v14x - VS 20xx C++ x64/86 build...
- Windows 10/11 SDK
- C++ CMake tools for Windows

Once these are installed, navigate to the repository folder in Windows Terminal, Command Prompt or Powershell, then

```
cargo run
```

to build & run the application. To compile for release append the release flag: `cargo run --release`. 

## Compiling the CLI application

Navigate to `lib/cli`, then copy your relevant platform file to `Makefile`. For example, on Mac OSX,

```
cp PIQUANT.mak.mac Makefile
```

Then `make`. There will be some errors due to missing unit testing files -- ignore these for now. The resulting executable will be in `bin/`. 

Compiled platform-specific CLI applications will be available on the [repository releases page](https://github.com/Pixadus/piquant-gui/releases). 