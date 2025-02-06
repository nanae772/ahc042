#!/bin/bash
set -eu

if [ -f results.txt ]; then
    rm results.txt
fi

cd ./tools/
cargo build -r --bin vis &> /dev/null
for file in "../input"/*; do
    if [ -f $file ]; then
        filename=$(basename "$file")
        ./target/release/vis ../input/$filename ../output/$filename | sed 's/Score = //' >> ../results.txt
    fi
done

cd ..

python stats.py < results.txt
