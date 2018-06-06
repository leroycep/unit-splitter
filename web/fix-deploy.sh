#!/usr/bin/env bash

cargo web deploy --target=wasm32-unknown-unknown
mkdir -p target/unit-splitter
mkdir -p target/unit-splitter/js
cp target/deploy/* target/unit-splitter/
mv target/unit-splitter/unit_splitter_web.js target/unit-splitter/js/app.js

