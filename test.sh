#!/usr/bin/env bash

set -vxe

# cd examples/raft-kv-memstore-network-v2/
# RUST_LOG=trace cargo run -- --id 1 --http-addr 127.0.0.1:13001 | tee app.1.log
# RUST_LOG=trace cargo run -- --id 2 --http-addr 127.0.0.1:13002 | tee app.2.log
# RUST_LOG=trace cargo run -- --id 3 --http-addr 127.0.0.1:13003 | tee app.3.log

# curl -XPOST -H "Content-Type: application/json" http://localhost:13001/init
# curl -XPOST -H "Content-Type: application/json" -d '[2, "localhost:13002"]' http://localhost:13001/add-learner
# curl -XPOST -H "Content-Type: application/json" -d '[3, "localhost:13003"]' http://localhost:13001/add-learner
# curl -XPOST -H "Content-Type: application/json" -d '[1, 2, 3]' http://localhost:13001/change-membership

curl -XPOST -H "Content-Type: application/json" -d '{"Set":{"key":"k1","value":"v1"}}' http://localhost:13001/write
curl -XPOST -H "Content-Type: application/json" -d '"k1"' http://localhost:13001/read
curl -XPOST -H "Content-Type: application/json" -d '"k1"' http://localhost:13001/consistent_read
curl -XPOST -H "Content-Type: application/json" -d '{"Get":{"key":"k1", "version":1}}' http://localhost:13001/consistent_read
# Invalid request
curl -XPOST -H "Content-Type: application/json" -d '{"Commit":{"keys_and_values":{}, "read_conflict_ranges":[], "write_conflict_ranges":[], "read_version":1, "write_version":2}}' http://localhost:13001/consistent_read
# Commit
curl -XPOST -H "Content-Type: application/json" -d '{"Commit":{"keys_and_values":{"k1":"v2"}, "read_conflict_ranges":[], "write_conflict_ranges":[], "read_version":1, "write_version":2}}' http://localhost:13001/write

curl -XPOST -H "Content-Type: application/json" -d '"key"' http://localhost:11000/read
curl -XPOST -H "Content-Type: application/json" -d '{"Get":{"key":"key", "version":1}}' http://localhost:11000/local_read
curl -XPOST -H "Content-Type: application/json" -d '{"Get":{"key":"key", "version":1}}' http://localhost:11000/consistent_read
