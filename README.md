
# Tilekit Hacks

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/tilekit/hacks?quickstart=1)


This repository serves as a tinkering playground and is intended for learning and exploration purposes. Goal is to try lots of things, see what works out, and then gradually improve bit by bit from there.

## Ways

- Each folder should have a specific tinkering task with a corresponding descriptive name, without underscores, dashes please señor.
- It is an individual hacker's responsibility to include easy to follow setup instructions in the README for an out-of-the-box tinkering experience.

## Active

- wasm-layer
- wgpu-layer
- wt-layer
- cli-base
- rust-gpu-setup
- webgpu-computes-starter


## Proposed

- wt-libp2p: Waiting for QUIC P2P draft to get implemented.

## Concluded

- unified-layer: `wasm_component_layer` crate not feasible as of now. Decided to use `extism` instead.
- extism-layer: Not possible to write host agnostic code easily. Component model not necessary as of now so switching to just the `wasm_runtime_layer` crate.
