#!/usr/bin/env bash

set -euo pipefail

cd $(readlink -f $(dirname $0))

export TIMEFORMAT="%R seconds" # elapsed

for day in $(find -maxdepth 1 -mindepth 1 -type d | sort -r); do
  day=$(echo $day | sed -e 's#./##')
  echo "******** DAY $day *******"
  pushd $day > /dev/null
  cargo fmt
  cargo clippy
  cargo build --release
  time ./target/release/$day
  popd > /dev/null
done
