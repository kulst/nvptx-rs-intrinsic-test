#![no_std]
#![feature(link_llvm_intrinsics)]
#![feature(abi_ptx, stdarch_nvptx)]

use core::arch::nvptx::*;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.nvvm.tex.1d.v4f32.s32"]
    fn _tex1dfetch_f32(a : u64, b: u64, c: u32) -> (f32, f32, f32, f32);
}

fn tex1dfetch_f32(a: u64, b: u64, c: u32) -> f32 {
    unsafe { _tex1dfetch_f32(a, b, c).0 }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn texture_memcpy(dst: *mut f32, src: u64, n: usize) {
    let thread_count = _block_dim_x() * _grid_dim_x();
    let mut id = _block_dim_x()
        .wrapping_mul(_block_idx_x())
        .wrapping_add(_thread_idx_x());
    while (id as usize) < n {
        *dst.offset(id as isize) = tex1dfetch_f32(src, 0, id as u32);
        id = id.wrapping_add(thread_count);
    }
}
