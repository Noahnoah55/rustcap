# rustcap

X11 screenshot tool written in Rust because shutter doesn't have a package for my version of Debian.

So far very barebones, takes a capture of the whole screen and saves it to ```$HOME/Pictures/Rustcap/{timestamp}.png```

Try it with ```cargo run```

See options with ```cargo run -- --help``

## TODO

- [X] Find a default directory
- [X] Add user interface
- [ ] Preview before saving
- [ ] Config file
- [ ] Gui
- [ ] Clipping support