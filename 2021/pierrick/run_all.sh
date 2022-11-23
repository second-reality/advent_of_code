#!/usr/bin/env bash

set -euo pipefail

cd $(readlink -f $(dirname $0))

export TIMEFORMAT="%R seconds" # elapsed

for day in */; do
  day=$(echo $day | sed -e 's#/##')
  echo "******** DAY $day *******"
  cd $day && cargo build --release -q && time ./target/release/ex$day -q && cd ..
done
