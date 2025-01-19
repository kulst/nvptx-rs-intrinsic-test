# Small example for CUDA kernels written in Rust

# Prerequisites
### Nightly toolchain
``` rustup install nightly```
### NVPTX target
``` rustup +nightly target add nvptx64-nvidia-cuda```
### LLVM bitcode linker and LLVM tools
``` rustup +nightly component add llvm-tools llvm-bitcode-linker```

# Compiling and problem
``` cargo +nightly rustc --release -- -emit=mir -emit=llvm-ir```

The `.ll` and `.mir` files are now in `target/nvptx64-nvidia-cuda/release/deps`.

The compilation gives an error from the linker that the intrinsic has 
an incorrect return type.

The intrinsic is defined as:
```tableGen
def int_nvvm_tex_1d_v4f32_s32
  : Intrinsic<[llvm_float_ty, llvm_float_ty, llvm_float_ty, llvm_float_ty],
              [llvm_i64_ty, llvm_i64_ty, llvm_i32_ty], [],
              "llvm.nvvm.tex.1d.v4f32.s32">;
```

This means the return parameters are defined as a list of 4 floating point values
and the intrinsic arguments as list of i64, i64, i32.

The intrinsic is defined in Rust as
```Rust
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.nvvm.tex.1d.v4f32.s32"]
    fn _tex1dfetch_f32(a : u64, b: u64, c: u32) -> (f32, f32, f32, f32);
}
```

However, in LLVM-IR the return type gets defined as `[4 x i32]`.
Probably this is the issue.

As in the MIR the f32 are still present in the return value this
happens probably during lowering from MIR to LLVM-IR.
