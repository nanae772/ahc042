#!/bin/bash
set -eu

bin_name=$(basename $(pwd))

# Build
cargo build --quiet

# Test
for file in "./input"/*; do
    if [ -f $file ]; then
        filename=$(basename "$file")
        ./target/debug/$bin_name < $file > output/$filename
    fi
done

echo "Finish!"
