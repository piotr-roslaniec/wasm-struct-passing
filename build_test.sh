#!/bin/bash

make clean && \
make && \
wasm-pack test --node && \
(
    cd js && \
    rm -rf node_modules/wasm-struct-passing && \
    yarn install --check-files && \
    yarn test
)