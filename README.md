# Rusty Stuff

Hopefully, over time, there will be multiple directories with some rust programs in them.


# History

## 2021_03_19
Released a working version of Stars3D.
Just a perspective plotting routine.

Keys in use are:

* [R] Rectangle Plotting
* [P] Pixel Plotting
* [C] Circle Plotting
* [/] Increase or Decrease Focal Length
* [PAGE_UP] Increase Focal Length - Use [LEFT_ALT] for larger change
* [PAGE_DOWN] Decrease Focal Length - Use [LEFT_ALT] for larger change
* [ALT-ENTER] Toggle windowed / full screen

### Running it
Boot inside a terminal that has access to your standard build utilities. If you're on windows then you can get access to the MSBuild tools, for free and legally, from somewhere around here: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017

In the terminal, head into the directory `raylib_stars3d` directory and start visual studio code.

When VSCode starts, press F5, and after a compile / download time, you'll see a star field.

You'll find the code in `main.rs` and `stars3d.rs`.

At least, that's the hope.



## 2021_03_18
Release a working version of raylib_star. A horizontal scrolling star field.

The code will be cleaned up as we go forward and learn more about Rust.

### Running it
Boot inside a terminal that has access to your standard build utilities. If you're on windows then you can get access to the MSBuild tools, for free and legally, from somewhere around here: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017

In the terminal, head into the directory `raylib_stars` directory and start visual studio code.

When VSCode starts, press F5, and after a compile / download time, you'll see a star field.

You'll find the code in `main.rs` and `stars.rs`.

At least, that's the hope.

### ToDo
Make it "more rusty"
Fix up the speed / colour code into the star itself.