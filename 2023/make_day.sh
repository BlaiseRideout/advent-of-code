#!/bin/bash

if [[ $1 != "" ]]; then
  mkdir -p "$1"
  shopt -s dotglob nullglob
  cp -R template/* "$1/"
  REPLACE_PATTERN="s/template/$1/g"
  sed -i "$REPLACE_PATTERN" "$1/"*.*
  rename "$REPLACE_PATTERN" "$1/"*.*
  rm "$1/"*template* 2> /dev/null
fi
