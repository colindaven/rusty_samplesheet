echo "Build Linux and Windows versions and copy the binaries to this directory"
echo "Usage: hpc03 only. bash copy_built_release.sh"

#cargo rustc --target=i686-pc-windows-gnu --release -- -C link-args=-mwindows
#You can build your crate easily with
cargo build --target x86_64-pc-windows-gnu
## Compile for Linux
cargo build --target release

cp target/x86_64-pc-windows-gnu/release/*.exe .
cp target/release/validate_samplesheet .

