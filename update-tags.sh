#!/bin/bash
git pull --tags
hash=$(git rev-parse HEAD)
for i in .git/refs/tags/*; do
  echo $hash > $i
done
git push --tags --force
