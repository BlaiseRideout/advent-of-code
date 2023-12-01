#!/bin/bash

echo "EXAMPLE:"
cargo run example.txt

echo "EXAMPLE 2:"
cargo run example2.txt

if [[ -f input ]]; then
  echo

  echo "INPUT:"
  cargo run input
fi
