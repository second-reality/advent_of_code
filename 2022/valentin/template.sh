#! /bin/bash


for i in {2..9}
do
  echo "Cp to day $i"
  cp src/day01.rs "src/day0$i.rs"
done

for i in {10..25}
do
  echo "Cp to day $i"
  cp src/day01.rs "src/day$i.rs"
done
