# blake-udf

MySQL UDF that implements blake3 hash algorithm using Rust.

## How to use

#### Load UDF in MySQL

```bash
$ mysql -uroot -p -e 'create function blake3_hash returns string soname \"libblake_udf.so\";'
```

#### Execute function

```bash
$ mysql --binary-as-hex=0 -uroot -p -e 'select blake3_hash("a");'
```

Output: `17762fddd969a453925d65717ac3eea21320b66b54342fde15128d6caf21215f`
