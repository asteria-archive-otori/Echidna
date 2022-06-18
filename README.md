

<a href="https://codeberg.org/EchidnaHQ/echidna"><p align="center">
<img height=150 src="https://codeberg.org/EchidnaHQ/echidna/raw/commit/63e61511aacd074e14c8894c5e600d3533012a23/assets/io.fortressia.Echidna.Source.svg"/>
<h2 align="center">Echidna Code 💖✨</p>

</p></a>
<p align="center">
  <strong>A 💕 lovelier 💕 code editor than your current one</strong>
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

## Installation

We're still not ready to launch Echy yet so there are no pre-built packages yet! 

You can however build from the cute source code! See the [Build from Source](./README#building-from-source) section.
## Building from source

The sections relevant are [Dependencies](#dependencies) and [Compiling](#compiling). On Windows, you need also see [Building in Windows](#building-in-windows).

If you're on Linux, just jump to [Building with Flatpak in Linux](#building-with-flatpak-in-linux). 
Unless if you're looking to repackage Echidna. <3

### Dependencies

To build Echidna Code, you'll need the following programs:
- Meson and the Ninja build system
 
For both running and building the app, the following dependencies need to be present on the system.
- Glib+Gio 2.66 
- GTK4
- Gtksourceview 5
- libvte3 with the "gtk4" flag turned on (currently this is not needed yet)


### Compiling

```
$ meson _build
```

```
$ meson compile
```


### Building in Windows
I haven't tested Echidna Code in Windows yet, but if you want to build this in Windows, you can either build the dependencies from source with [Gvsbuild](https://github.com/wingtk/gvsbuild), or get the fluffyy pre-built binary happiness from [fluffy-prebuilt-happiness](https://github.com/EchidnaHQ/fluffy-prebuilt-happiness/).

To use Visual Studio in Meson, use this instead of the one in [Compiling](#Compiling).

```
> meson _build --backend=vs
```

On Linux, Echidna Code is generally not intended for use on a native distro. It's recommended to develop and run Echidna with Flatpak. See [the part below](#building-with-flatpak-in-linux).

### Building with Flatpak in Linux

Firstly, install [Flatpak](https://flatpak.org/setup/) and [Flatpak Builder](https://docs.flatpak.org/en/latest/first-build.html).

The official docs of Flatpak isn't great if you're looking to use it for development purposes, as many default parameters aren't suited for development purposes. 

You should just stick to these two lovely angels: [GNOME Builder](https://flathub.org/apps/details/org.gnome.Builder) and [the Flatpak extension for Visual Studio Code](https://open-vsx.org/vscode/item?itemName=bilelmoussaoui.flatpak-vscode). 

## FAQ

## How will Echidna work under Flatpak?
It's roughly the same as the one written in [the Flathub packaging disclaimer for VSCode](https://github.com/flathub/com.visualstudio.code/blob/master/flatpak-warning.txt).

I will brew a GUI helper for that thooo. <3


### Will you ship it to other platforms?
Windows 10 is planned. Thankfully, I still keep my Windows 10 installation in my laptop, since I use it for games. MacOS however, kinda hard, since I don't have any Mac devices. If you are a Mac user, you can try [Building from Source](#building-from-source).

I'm interested to port GDK to Android and the Web (without the current state of Broadway), but that'll take time definitely.
## License

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed in the [`LICENSE`](./LICENSE) file, You can obtain one at https://mozilla.org/MPL/2.0/.

*Credits to [Dogehouse](https://github.com/benawad/dogehouse) for the README template.*
