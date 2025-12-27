## di-rs - Implementation of ```dir``` file listing utility for BSD systems in Rust

### **Warning!** This project is in development stage and is not yet ready for production usage. There exists hardcode, which may not be executable on your machine.

- To build and run project: clone the repository with source code and run
```
cargo run
```
or
```cargo build --release```
for release mode and
```cargo build```
for debug mode.

### Dependencies:
- [chrono](https://crates.io/crates/chrono) - Timezone-aware date and time handling
- [num-format](https://crates.io/crates/num_format/0.1.2) - A Rust crate for producing string-representations of numbers, formatted according to international standards




### Notes:
- Temporarily, the utility will accept - as flag for arguments, E.g.
```
dir -?
```
Otherwise, all of the arguments will remain accordingly to official version.
- Supported arguments:
| Argument  | Description  																	 |
| :-------- | :----------------------------------------------------------------------------- |
|  /?       |  Displays help at the command prompt.                                          |
|  /b       | Displays a bare list of directories and files, with no additional information. |
- The utility is yet built and tested only at UFS. Porting to ZFS will be made later 
- The utility automatically displays content in current working directory. Support of command line arguments will be added soon.
- Utility was tested on FreeBSD, in future will be tested and ported into OpenBSD and NetBSD.
- Contributors are welcomed.
