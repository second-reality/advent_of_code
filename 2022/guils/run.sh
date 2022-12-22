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
    cargo build --release
    echo "AoC run $day"
    time -p target/release/"$day"
done
