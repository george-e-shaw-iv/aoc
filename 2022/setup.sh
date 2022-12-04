#!/usr/bin/env bash

if ! command -v brew >/dev/null; then
  echo "Please install asdf and then rerun script - https://brew.sh/" >2
  exit 1
fi

brew install clojure/tools/clojure
