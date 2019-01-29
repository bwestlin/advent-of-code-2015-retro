#!/bin/bash
DAY="day$1"

if [ ! -f "src/$DAY.rs" ]; then
    exit 1
fi

echo "target/release/$DAY"
