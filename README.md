# Rusty Stuff

Hopefully, over time, there will be multiple directories with some rust programs in them. Just recreations of some Old Skool demo stuff. Not to the same standard, of course, just for Rust fun.

# Release History
## 2021_03_27 - Chunky Fire
Released a working version of chunky fire. Rough, it could be said. Very slow too.

### Keys In Use:

* [R] Rectangle Plotting
* [P] Pixel Plotting
* [C] Circle Plotting
* []] Increase Rectangle border
* [[] Decrease Rectangle border
* [ALT-ENTER] Toggle windowed / full screen

### Running it.
See the *General Run Instructions* section, below.
Copy the png files in `raylib_fire\src\resources` to the location of your exe.
The main directory is `raylib_fire` and the source code is contained in `main.rs` and `fire.rs`.

## 2021_03_19 - Stars3D
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


### Running it.
See the *General Run Instructions* section.
The main directory is `raylib_stars3d` and the source code is contained in `main.rs` and `stars3d.rs`.

## 2021_03_18 - Parallax Stars

Release a working version of `raylib_star`. A horizontal scrolling star field.

The code will be cleaned up as we go forward and learn more about Rust.

### Keys In Use
N/a

### Running It
See the *General Running Instructions* section below. The directory name is `raylib_stars` and the main code is in `main.rs` and the stars code is in `stars.rs`.


# General Running Instructions
Boot inside a terminal that has access to your standard build utilities. If you're on windows then you can get access to the MSBuild tools, for free and legally, from somewhere around here: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017

In the terminal, head into the directory <<Example_Directory_Name>>  and start visual studio code.

When VSCode starts, press F5, and after some compile / download time, you'll see the demo running.

You'll find the code in `main.rs` and other code in the <<demo_name.rs>> file.

At least, that's the hope.

### ToDo
Make everything "more rusty".
