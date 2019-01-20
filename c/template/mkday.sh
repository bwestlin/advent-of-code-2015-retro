#!/bin/bash

DAY=$1
if [ -z "$DAY" ]
  then
    echo "No day supplied"
    exit 1
fi

DSTDIR="day$DAY"
DST="$DSTDIR/day$DAY.c"
mkdir -p "$DSTDIR/"
cp template/day.c "$DST"
sed "s/xDAYx/day$DAY/g" < template/Makefile > "$DSTDIR/Makefile"
echo "$DST created"
