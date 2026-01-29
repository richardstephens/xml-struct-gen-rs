#!/bin/bash

set -euo pipefail

cd $(git rev-parse --show-toplevel)

cargo build -p xml-struct-gen

./target/debug/xml-struct-gen \
    --input xmlstructs-podcast/src/podcast1.xml \
    --input xmlstructs-podcast/src/podcast-private.xml \
    --output xmlstructs-podcast/src/podcast_structs.rs

cargo fmt --package xmlstructs-podcast

