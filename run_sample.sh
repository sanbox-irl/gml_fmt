#!/bin/bash
cargo run -- benches/samples/osg_lex_speed.gml -f -l -n -s > ignored/output.yaml; 
code ignored/output.yaml;