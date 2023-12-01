#!/usr/bin/env bash
set -euo pipefail

day="${1?}"

cd "$(dirname "$0")"
cargo new "day$day"
cp -pr template/* "day$day"/
git add "day$day"/
git status -uno
git commit -m "guils - $day"
