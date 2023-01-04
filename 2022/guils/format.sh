#!/usr/bin/env bash

set -euo pipefail

dir="$(dirname "$(readlink -e "$0")")"

GC='\033[0;32m'
BGC='\033[1;32m'
NC='\033[0m'
cd "$dir"
for day in day*; do
    cd "$dir/$day"
    echo -e "${GC}AoC${NC} build ${BGC}$day${NC}..."
    /usr/bin/time -f 'elapsed: %e' bash -c 'cargo fmt && cargo clippy'
done
