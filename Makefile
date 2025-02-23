build: 
	cargo build --target x86_64-dolph_os.json

boot:
	cargo bootimage #builds the kernel and uses the bootloader to run the kernel