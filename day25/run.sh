#!/bin/bash

echo "EXAMPLE:"
cat example.txt | cargo run

if [[ -f input ]]; then
  echo

  echo "INPUT:"
  cat input | cargo run
fi
