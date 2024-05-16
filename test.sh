#!/usr/bin/env bash

set -vxe

# cd examples/raft-kv-memstore-network-v2/
# RUST_LOG=debug cargo run -- --id 1 --http-addr 127.0.0.1:13001 | tee app.1.log
# RUST_LOG=debug cargo run -- --id 2 --http-addr 127.0.0.1:13002 | tee app.2.log
# RUST_LOG=debug cargo run -- --id 3 --http-addr 127.0.0.1:13003 | tee app.3.log

# curl -XPOST -H "Content-Type: application/json" http://localhost:13001/init
# curl -XPOST -H "Content-Type: application/json" -d '[2, "localhost:13002"]' http://localhost:13001/add-learner
# curl -XPOST -H "Content-Type: application/json" -d '[3, "localhost:13003"]' http://localhost:13001/add-learner
# curl -XPOST -H "Content-Type: application/json" -d '[1, 2, 3]' http://localhost:13001/change-membership

curl -XPOST -H "Content-Type: application/json" -d '{"Set":{"key":"k1","value":"v1"}}' http://localhost:13001/write
curl -XPOST -H "Content-Type: application/json" -d '"k1"' http://localhost:13001/read
curl -XPOST -H "Content-Type: application/json" -d '"k1"' http://localhost:13001/consistent_read
