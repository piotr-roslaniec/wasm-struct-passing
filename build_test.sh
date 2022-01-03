#!/bin/bash

make clean && \
make && \
wasm-pack test --node && \
(
    cd js && \
    yarn install && \
    yarn test
)