set -euo pipefail

cd $(readlink -f $(dirname $0))

export TIMEFORMAT="%R seconds" # elapsed

main()
{
  if [ $# -eq 0 ]; then
    all_days=$(find -maxdepth 1 -mindepth 1 -type d | sort -r)
    build_and_run $all_days
  else
    build_and_run $@
  fi
}

build_and_run()
{
  for day in $@; do
    day=$(echo $day | sed -e 's#./##')
    echo "******** DAY $day *******"
    pushd $day > /dev/null
    tmp_bin=$(mktemp)
    g++ -pedantic -Wall -Wextra -std=c++23 -o $tmp_bin main.cpp 
    time $tmp_bin
    rm $tmp_bin
    popd > /dev/null
  done
}

main $@
