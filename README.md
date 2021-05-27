# rusty_samplesheet
Very simple Illumina SampleSheet validation

Help, my SampleSheet is a bit rusty, and by that I mean crap.

A windows/linux program written in Rust
- parse Illumina SampleSheet.csv in the directory
- complain about format problems in a biologist friendly fashion
- link to examples showing proper samplesheets (internal MHH Net only at present)



## Run it!
Windows: 
1. Call your SampleSheet SampleSheet.csv. 
2. Double click the validate_samplesheet.bat
3. Results will be printed in output.txt, open this with Wordpad or Notepad++ or another competent text editor.  

Linux: 
```
chmod a+x validate_samplesheet
./validate_samplesheet -f your_SampleSheet.csv
```

## Compile for Windowsm(Done on hpc03 internally)
```
# first setup Ubuntu
sudo apt update && sudo apt install mingw-w64

# now setup rust
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu

# now add this to rust cargo.toml in your project directory:

[target.x86_64-pc-windows-gnu]
linker = "/usr/bin/x86_64-w64-mingw32-gcc"

[target.i686-pc-windows-gnu]
linker = "/usr/bin/i686-w64-mingw32-gcc"
rustflags = "-C panic=abort"

# build 
cargo rustc --target=i686-pc-windows-gnu --release -- -C link-args=-mwindows
#You can build your crate easily with
cargo build --target x86_64-pc-windows-gnu

```


## Compile for Linux
```
cargo build --target release
```
