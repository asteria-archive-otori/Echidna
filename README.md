# Echidna Code Editor
A friendlier code editor than your current one... not currently...

## Justification
I started this project when I was frustated with how laggy [VSCode](https://code.visualstudio.com/) is on low-resource machines. VSCode and its similar alternatives are made with the [Electron](https://electronjs.org/) framework that embeds Chromium into. You know how Chromium-based browsers are known for its high memory usage, which basically kills low-resource machines that usually only have 4GB of RAM, and its high battery usage, which basically kills laptops in general.

This is why Echidna is made with the GTK toolkit. A toolkit heavily used in the Linux operating system and saw usage as well in Windows and Mac, though usually in Windows ports of Linux softwares that are using it, like GIMP. I wish I can port GTK to Android as well, so that devs with not much money can still learn to code.

## Philosophy

### Be friendly
The UX and UI must be friendly and intuitive towards developers. The performance must also be friendl towards developers, especially those that doesn't have a beefy machine. This is why we choose not to use Electron.
### Echidna is not VSCode
Echidna should be Echidna, and not VSCode. While being friendly towards those used with VSCode, Echidna should be filled with our opinions on how things should work instead of those of the VSCode team.

### Open Source
Echidna should always be open source and developed for the community by the community.

## Build Instructions & Download Information
Pre-built downloads are currently not available, but you can easily built this program from the source code.

In order to do that you'll need to have these programs installed:
- The Rust Programming Toolchain (stable version) 
- GTK4
- GIO and Glib
- GtkSourceView version 5
- Libvte with GTK4 flag enabled (for `vte` branch only for now)

It's not guaranteed that Echidna will run nicely with your Linux distribution. This is mainly due to the fact that my machine runs Arch Linux (btw) which usually has newer package versions than yours.

Clone this repository with Git and run:
```rs
cargo build
```
There should be an executable called `echidna` (or `echidna.exe` in Windows) inside of the `target` folder of the current directory now. You can run it to test Echidna. You can also use [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) to configure how you want Echidna to be built. 

You can also simply run `cargo run` to build and run the program. 

Please be aware that Echidna is still at the state of a "Notepad clone" at the moment. Also due to this, building Echidna will just take some small seconds for now.