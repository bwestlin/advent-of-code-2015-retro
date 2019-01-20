#!/bin/bash
DAY="day$1"

if [ ! -d "$DAY" ]; then
    echo "$DAY not found"
    exit 1
fi

cd $DAY
make || (echo "Build failed"; exit 1)
