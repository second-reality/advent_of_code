#!/usr/bin/env bash

set -euo pipefail

dir="$(dirname "$(readlink -e "$0")")"

/usr/bin/time -f 'Total format elapsed: %e' "$dir"/format.sh
/usr/bin/time -f 'Total build elapsed: %e' "$dir"/build.sh
/usr/bin/time -f 'Total run elapsed: %e' "$dir"/exec.sh
