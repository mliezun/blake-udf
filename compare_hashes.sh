#!/bin/bash

set -euo pipefail

MYSQL_USER="root"
MYSQL_ROOT_PASSWORD="testpass"

# Start docker container
#docker container rm -f blake-udf
#docker build -t blake-udf .
#docker run --name blake-udf -e MYSQL_ROOT_PASSWORD=$MYSQL_ROOT_PASSWORD -d blake-udf
#sleep 120 # Mysql takes time to start

# Configure blake3_hash function in mysql
#docker exec blake-udf sh -c "mysql -u$MYSQL_USER -p$MYSQL_ROOT_PASSWORD -e 'create function blake3_hash returns string soname \"libblake_udf.so\";'"

# Define assert function
assert_eq() {
  local expected="$1"
  local actual="$2"
  local msg="${3-}"

  if [ "$expected" == "$actual" ]; then
    return 0
  else
    [ "${#msg}" -gt 0 ] && echo "$expected == $actual :: $msg" || true
    return 1
  fi
}

mysql_blake3_hash () {
    (docker exec blake-udf sh -c "mysql --binary-as-hex=0 -u$MYSQL_USER -p$MYSQL_ROOT_PASSWORD -e 'select blake3_hash(\"$1\");'" | tail -1) 2>/dev/null
}

assert_eq '17762fddd969a453925d65717ac3eea21320b66b54342fde15128d6caf21215f' $(mysql_blake3_hash 'a') "Hash does not match expected value"

echo "Test passed"
