compile_dylib:
	cargo build --release

compile_c:
	gcc -c testprogram/main.c -o testprogram/main.o
	gcc testprogram/main.o target/release/libblake_udf.dylib -o testprogram/main

test_plugin:
	docker container rm -f blake-udf
	docker build -t blake-udf .
	docker run --name blake-udf -e MYSQL_ROOT_PASSWORD=testpass -d blake-udf
	docker exec blake-udf sh -c 'mysql -uroot -ptestpass -e "create function blake3_hash returns string soname \"libblake_udf.so\";"'
	docker exec blake-udf sh -c 'mysql --binary-as-hex=0 -uroot -ptestpass -e "select blake3_hash(\"a\");"'

clean:
	rm -rf target outputs
