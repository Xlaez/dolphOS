# DolphOS

Building a minimalistic kernel using the blog posts [here](http://os.phil-opp.com/)

Requirements to run this kernel:

1. Rust (nightly build)
2. **bootimage** crate installed. Run `cargo install bootimage` in your home directory.
3. Add this to your rustup `rustup component add llvm-tools-preview`.
4. Qemu VM to allow you run the kernel. Run `sudo apt-get install qemu-user-static`

Steps to run this Kernel:

1. Run `make boot` to compile the code to binary and link the bootloader with the kernel.
2. Run `qemu-system-x86_64 -drive format=raw,file=target/x86_64-dolph_os/debug/bootimage-dolph_os.bin` to load the kernel with Qemu.

