#!/bin/bash
cargo run -- benches/samples/small_test.gml -f -n -l > ignored/output.yaml;
code ignored/output.yaml;