#!/usr/bin/env bash

set -euo pipefail

dir="$(dirname "$(readlink -e "$0")")"

echo "$dir"
cd "$dir"
for day in day*; do
    cd "$dir/$day"
    echo "AoC build $day"
    cargo fmt
    cargo clippy
    cargo build
    echo "AoC run $day"
    target/debug/"$day"
done
