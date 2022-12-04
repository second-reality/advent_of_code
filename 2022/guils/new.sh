#!/usr/bin/env bash
set -euo pipefail

day="${1?}"

cd "$($dirname "$0")"
cargo new "$day"
cp -pr template/* "$day"/
