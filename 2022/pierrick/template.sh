#!/usr/bin/env bash

set -euo pipefail

die()
{
    echo "$@" 1>&2
    exit 1
}

cd $(readlink -f $(dirname $0))

[ $# -eq 2 ] || die "usage: day input_file"

day=$1; shift
input_file=$1; shift

mkdir $day
cat > $day/Cargo.toml << EOF
[package]
name = "ex$day"
version = "0.1.0"
edition = "2021"
EOF

mkdir $day/src/
cp template.rs $day/src/main.rs

cp $input_file $day/
