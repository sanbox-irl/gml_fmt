#!/bin/bash
cargo run -- -f benches/samples/kat_math.gml > ignored/output.yaml; 
code ignored/output.yaml;