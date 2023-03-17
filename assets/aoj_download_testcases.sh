#!/bin/bash

set -euxvo pipefail

# Check if the user provided an argument
if [ $# -eq 0 ]; then
  echo "Please provide a problem_id as an argument"
  exit 1
fi

mkdir "$1"
cd "$1"

base_url="https://judgedat.u-aizu.ac.jp/testcases/$1"
num_problems=$(curl "${base_url}/header" | jq '.headers | length')

for ((i=0;i<$num_problems;i++)); do
  url="${base_url}/${i}"
  mkdir "${i}"
  cd "${i}"
  echo "Downloading Problem $i: ${url}"
  wget "${url}/in"
  wget "${url}/out"
  cd ..
done

cd ..

