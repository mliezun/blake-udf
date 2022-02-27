FROM rust:1.58

WORKDIR /app

RUN apt-get update && apt-get install -y libclang-dev

COPY . /app/

RUN cargo build --release

FROM mysql:latest

COPY --from=0 /app/target/release/libblake_udf.so /usr/lib/mysql/plugin/

RUN mysql -e "create function blake3_hash returns string soname 'libblake_udf.so';"
