#!/bin/bash

LANG=$1
if [ -z "$LANG" ]; then
    echo "No language supplied"
    exit 1
fi

DAY=$2
if [ -z "$DAY" ]; then
    echo "No day supplied"
    exit 1
fi

INPUT=input/day$DAY/input

function run_in {
    if [ ! -d "$1" ]; then
        return
    fi
    if [ ! -f "$1/cmdline.sh" ]; then
        echo "No $1/cmdline.sh found!"
        return
    fi
    if [ ! -f "$1/build.sh" ]; then
        echo "No $1/build.sh found!"
        return
    fi

    pushd $1 >> /dev/null
    CMDLINE=$(./cmdline.sh $DAY)
    if [ $? != 0 ]; then
        echo "No solution for $DAY in $1"
        popd >> /dev/null
        return
    fi

    ./build.sh $DAY > /dev/null 2>&1
    if [ $? != 0 ]; then
        echo "Build failed for $DAY in $1"
        popd >> /dev/null
        return
    fi

    popd >> /dev/null
    CMD="$1/$CMDLINE $INPUT"
    echo "Running $CMD"
    $CMD
}

if [ $LANG == "all" ]; then
    for i in * ; do
        if [ -d "$i" ] && [ "$i" != "input" ]; then
            run_in "$i"
        fi
    done
else
    run_in $LANG
fi
