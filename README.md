

<a href="https://codeberg.org/EchidnaHQ/echidna"><p align="center">
<img height=150 src="https://codeberg.org/EchidnaHQ/echidna/raw/commit/63e61511aacd074e14c8894c5e600d3533012a23/assets/io.fortressia.Echidna.Source.svg"/>
<h2 align="center">Echidna Code</p>

</p></a>
<p align="center">
  <strong>A friendlier code editor than your current one. </strong>
</p>
<p align="center">
  <a href="https://discord.gg/BJusUKd8Vj">
    <img src="https://img.shields.io/discord/880440438652284988?style=for-the-badge" alt="discord - users online" />
  </a>
</p>

<h3 align="center">
  <a href="./CONTRIBUTING.md">Contribute</a>
  <span> · </span>
  <a href="https://discord.gg/BJusUKd8Vj">Community</a>
 
</h3>

---
*This README is written by Nefo Fortressia. Markdown documents like this tend to not let you know who's the original author, huh?*


## Installation

Pre-built binaries are not yet provided, but you can build from source using the [Build from Source](./README#building-from-source) section.
## Building from source
To start, you'll need the following dependencies available in the environment:
- Glib 2 
- GTK4
- Gtksourceview 5
- libvte4 (not yet, it'll be required once the built-in terminal feature has been merged), built with "gtk4" Meson flag. You'll likely need to compile this from scratch instead of using your distro-provided library as the flag isn't turned on by default.

Echidna Code is not optimized to run on systems that don't have the latest system libraries (Arch user here). You might not even able to compile it on those systems, just like a friend of mine:

![](https://codeberg.org/EchidnaHQ/echidna/attachments/300f2211-e19d-4cc4-90f3-80d8b0c6587d)
*See issue [#40](https://codeberg.org/EchidnaHQ/echidna/issues/40). Credit to [eramne](https://twitter.com/eramne2).*

If you went into these problems, you need to install the packages either from source or from Arch Linux, and tell pkg-config to use them instead of your system libraries.

To install, simply run the following command:

```sh
$ cargo build --release
```

## FAQ

### How will you ship Echidna in Linux?
I'm going to ship it to [Flatpak](http://flatpak.org/) and AppImage only. It's a universal package manager that works in every distribution.

### Flatpak isn't great for development environment, isn't it? How will you overcome that?
I'm not really sure, but I think I can use [NixOS](https://nixos.org/). It's a universal package manager that's also available as a Linux-based operating system. It's a good choice for  sandboxed development environments like this, especially since you can write down the packages you are using for each projects in a `.nix` file. 

### Will you ship it to other platforms?
Windows 10 is planned. Thankfully, I still keep my Windows 10 installation in my laptop, since I use it for games. MacOS however, kinda hard, since I don't have any Mac devices. If you are a Mac user, you can try [Building from Source](#building-from-source).

I'm interested to port GDK to Android and the Web (without the current state of Broadway), but that'll take time definitely.
## License

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed in the [`LICENSE`](./LICENSE) file, You can obtain one at https://mozilla.org/MPL/2.0/.

*Credits to [Dogehouse](https://github.com/benawad/dogehouse) for the README template*