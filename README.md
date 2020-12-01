# appinfo-vdf
A command-line utility to parse Steam appinfo.vdf files.

The original motivation to write this utility was to be able to patch appinfo.vdf to workaround an [issue running Assassin's Creed II on Linux](https://github.com/ValveSoftware/Proton/issues/190). The patching code is still there, however the generated files are rejected by Steam because the checksum of the file entries are invalid. I don't know how the checksums should be computed, so I wasn't able to implement that part.

## Requirements

- Rust 1.48.0
- Cargo

## Building

`cargo build`
