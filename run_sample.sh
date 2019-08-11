#!/bin/bash
cargo run -- benches/samples/small_test.gml -f -l -n -s > ignored/output.yaml; 
code ignored/output.yaml;