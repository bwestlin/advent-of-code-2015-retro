#!/bin/bash
DAY="day$1"

if [ ! -f "src/$DAY.rs" ]; then
    echo "src/$DAY.rs not found"
    exit 1
fi

cargo build --release --bin $DAY || (echo "Build failed"; exit 1)
