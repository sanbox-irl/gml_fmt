#!/bin/bash
cargo run -- -f benches/samples/small_test.gml -l > ignored/output.yaml; 
code ignored/output.yaml;