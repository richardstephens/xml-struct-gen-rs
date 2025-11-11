#!/bin/bash

set -euo pipefail

cd $(git rev-parse --show-toplevel)

cargo build -p xml-struct-gen

./target/debug/xml-struct-gen \
    --input xmlstructs-libvirt/src/domain1.xml \
    --input xmlstructs-libvirt/src/domain2.xml \
    --output xmlstructs-libvirt/src/domain_structs.rs

cargo fmt --package xmlstructs-libvirt

