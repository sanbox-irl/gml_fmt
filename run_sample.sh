#!/bin/bash
cargo run -- benches/samples/small_test.gml -f -l -n > ignored/output.yaml; 
code ignored/output.yaml;