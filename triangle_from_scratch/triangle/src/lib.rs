pub mod macros;
pub mod util;
#[cfg(windows)]
pub mod win32;
pub use crate::{win32::*,gl::*,util::*};
pub mod gl;

pub unsafe fn gather_null_terminated_bytes(mut p: *const u8) -> Vec<u8> {
	let mut v = vec![];
	while *p != 0 {
		v.push(*p);
		p = p.add(1);
	}
	v
}
pub fn min_alloc_lossy_into_string(bytes: Vec<u8>) -> String {
	match String::from_utf8(bytes) {
		Ok(s) => s,
		Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
	}
}

/// Type for [wglChoosePixelFormatARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub type wglChoosePixelFormatARB_t = Option<
	unsafe extern "system" fn(
		hdc: HDC,
		piAttribIList: *const c_int,
		pfAttribFList: *const f32,
		nMaxFormats: c_uint,
		piFormats: *mut c_int,
		nNumFormats: *mut UINT,
	) -> BOOL,
>;
pub type FLOAT = c_float;
pub type c_float = f32;
/// Type for [wglCreateContextAttribsARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub type wglCreateContextAttribsARB_t = Option<
	unsafe extern "system" fn(hDC: HDC, hShareContext: HGLRC, attribList: *const c_int) -> HGLRC,
>;
/// Type for [wglSwapIntervalEXT](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt)
pub type wglSwapIntervalEXT_t =Option<unsafe extern "system" fn(interval: c_int)->BOOL>;
