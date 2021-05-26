# rusty_samplesheet
Very simple Illumina SampleSheet validation

Help, my SampleSheet is a bit rusty, and by that I mean crap.

A windows program written in Rust
- parse Illumina SampleSheet.csv in the directory
- complain about format problems in a biologist friendly fashion
- link to examples showing proper samplesheets

 



## Run it!
Windows: double click

Linux: 
´chmod a+x validate_samplesheet´

´./validate_samplesheet´


## Compile for Windowsm(Done on hpc03 internally)
´´´
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
#You can build your crate easily with
cargo build --target x86_64-pc-windows-gnu
´´´

## Compile for Linux

cargo build --target release
