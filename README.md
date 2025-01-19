# Small example for CUDA kernels written in Rust

# Prerequisites
## Nightly toolchain
``` rustup install nightly```
## NVPTX target
``` rustup +nightly target add nvptx64-nvidia-cuda```
## LLVM bitcode linker and LLVM tools
``` rustup +nightly component add llvm-tools llvm-bitcode-linker```
