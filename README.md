# CLI Toolbox
Tools for making a simple and good looking CLI.
Design primarily for programs with multiple sub-programs.

## About
The Idea with this library is to have a easy to use framework to easily make terminal programs with multiple sub-programs. 
A way to make the text and color for the prints to make sense for the particular part of the program.

The Program is used to handle the interface of a program. While the System is used as a navigation point consisting of other programs.
Working more as a Map.
## Include It
To use this crate. Under the dependencies in your Cargo.toml, just add this crate as followed

```cli_toolbox = { git = "https://github.com/AlbinDalbert/CLI-Toolbox.git" }```

Then just update your cargo the everything is up and running.

## Road-map
This is still in a very early stage, a lot of things have not been implemented and/or works yet.

### Core Functions
- [x]    Customizable for each instance of a `System` and `Program`
- [x]    Program inherits optional attributes from `System` by default
- [x]    Programs can be executed via the `run()` function, (e.g. They are fundamentally linked together, not just for looks)
- [x]    Change the cargo name to something better and more telling
- [x]    Make a generic `Menu` where each item has a `name` and `action`(function)

### Optional Steps
- [ ]   System can have sub-system
