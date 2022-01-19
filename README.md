# rusty_samplesheet
Very simple Illumina SampleSheet validation

Help, my SampleSheet is a bit rusty, and by that I mean crap.

A windows/linux program written in Rust
- parse an Illumina SampleSheet.csv in the directory
- complain about format problems in a (hopefully) biologist friendly fashion
- link to examples showing properly formatted SampleSheets (internal MHH network only at present)



## Run it!
Windows: 
1. Call your SampleSheet `SampleSheet.csv` (Windows, set this in the `validate_samplesheet.bat` if you like). 
2. Double click the `validate_samplesheet.bat`
3. Results will be printed in `output.txt`, open this with Wordpad or Notepad++ or another competent text editor (NOT notepad!).  

Linux: 
```
chmod a+x validate_samplesheet
./validate_samplesheet -f your_SampleSheet.csv
```

## Checks performed
- No "." in sample IDs and names, allowed on lines containing Date
- German Umlaut äüö etc disallowed
- Incorrect headers, eg Sample_ID present more than once
- Incorrect adapters lines with more than three """ (should be something like "Index Adapters,""TruSeq DNA CD Indexes (96 Indexes)""" )
- Semicolons ; used instead of commas , as delimiters
- [Data] section - duplicate Sample_Name strings, Sample_ID strings, duplicate indices.
- Should check for " in lines, but this is difficult since parsing fails. 

## Compile for Windows (done on hpc03 internally)
```
# first setup Ubuntu
sudo apt update && sudo apt install mingw-w64

# now setup rust
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup toolchain install i686-pc-windows-gnu

# now add this to rust cargo.toml in your project directory:

[target.x86_64-pc-windows-gnu]
linker = "/usr/bin/x86_64-w64-mingw32-gcc"

[target.i686-pc-windows-gnu]
linker = "/usr/bin/i686-w64-mingw32-gcc"
rustflags = "-C panic=abort"

# build 
# Scripted in copy_built_release.sh for Linux and Windows
bash copy_built_release.sh
#cargo rustc --target=i686-pc-windows-gnu --release -- -C link-args=-mwindows
#cargo rustc --target=stable-i686-pc-windows-gnu --release -- -C link-args=-mwindows
#You can build your crate easily with
cargo build --target x86_64-pc-windows-gnu

```


## Compile for Linux
```
cargo build --target release
```
