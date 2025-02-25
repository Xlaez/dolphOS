build: 
	cargo build --target x86_64-dolph_os.json

boot:
	cargo bootimage #builds the kernel and uses the bootloader to run the kernel

run: 
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-dolph_os/debug/bootimage-dolph_os.bin
