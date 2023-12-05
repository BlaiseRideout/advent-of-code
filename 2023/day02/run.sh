#!/bin/bash

echo "EXAMPLE:"
cargo run example.txt

if [[ -f input ]]; then
  echo

  echo "INPUT:"
  cargo run input
fi
