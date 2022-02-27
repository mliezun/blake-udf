compile_dylib:
	cargo build --release
	
compile_c:
	gcc -c testprogram/main.c -o testprogram/main.o
	gcc testprogram/main.o target/release/libblake_udf.dylib -o testprogram/main

clean:
	rm -rf target outputs
