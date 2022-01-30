// https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types
// https://docs.microsoft.com/en-us/cpp/cpp/data-type-ranges?view=msvc-160
#![allow(non_camel_case_types, non_snake_case)]

extern crate core;
pub use super::*;
pub use core::ptr::{null, null_mut};

macro_rules! unsafe_impl_default_zeroed {
	($t:ty) => {
		impl Default for $t {
			#[inline]
			#[must_use]
			fn default() -> Self {
				unsafe { core::mem::zeroed() }
			}
		}
	};
}

#[link(name = "Kernel32")]
extern "system" {
	/// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
	/// [`GetProcAddress`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)
	pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
	/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
	pub fn GetLastError() -> DWORD;
	/// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
	pub fn SetLastError(dwErrCode: DWORD);
	/// [`FormatMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
	pub fn FormatMessageW(
		dwFlags: DWORD,
		lpSource: LPCVOID,
		dwMessageId: DWORD,
		dwLanguageId: DWORD,
		lpBuffer: LPWSTR,
		nSize: DWORD,
		Arguments: *const va_list,
	) -> DWORD;
	/// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
	/// [`LoadLibraryW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
	pub fn LoadLibraryW(lpLibFileName: LPCWSTR) -> HMODULE;
	/// [`FreeLibrary`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
	pub fn FreeLibrary(hLibModule: HMODULE) -> BOOL;
}
pub type FARPROC = *mut core::ffi::c_void;

#[link(name = "User32")]
extern "system" {
	/// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
	pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;

	/// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	pub fn CreateWindowExW(
		dwExStyle: DWORD,
		lpClassName: LPCWSTR,
		lpWindowName: LPCWSTR,
		dwStyle: DWORD,
		X: c_int,
		Y: c_int,
		nWidth: c_int,
		nHeight: c_int,
		hWndParent: HWND,
		hMenu: HMENU,
		hInstance: HINSTANCE,
		lpParam: LPVOID,
	) -> HWND;

	/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	pub fn BeginPaint(hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;
	/// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	pub fn DefWindowProcW(hwnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	pub fn DestroyWindow(hWnd: HWND) -> BOOL;
	/// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
	pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
	/// [``](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
	/// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
	pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFillerMin: UINT, wMsgFillerMax: UINT) -> BOOL;
	/// [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
	pub fn GetSysColor(nIndex: c_int) -> DWORD;
	/// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
	/// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox)
	pub fn MessageBox(hWnd: HWND, lpText: LPCTSTR, lpCaption: LPCTSTR, uType: UINT) -> c_int;
	/// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
	pub fn PostQuitMessage(nExitCode: c_int);
	/// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
	/// [`SetCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursor)
	pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
	/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
	pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
	/// [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
	pub fn UnregisterClassW(lpClassName: LPCWSTR, hInstance: HINSTANCE) -> BOOL;

	/// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	pub fn GetDC(hWnd: HWND) -> HDC;
	/// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
	pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
}

#[link(name = "Gdi32")]
extern "system" {
	/// [`ChoosePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat)
	pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;
	/// [`SetPixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixelformat)
	pub fn SetPixelFormat(hdc: HDC, format: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;
	/// [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
	pub fn DescribePixelFormat(
		hdc: HDC,
		iPixelType: c_int,
		nBytes: c_uint,
		ppfd: LPPIXELFORMATDESCRIPTOR,
	) -> c_int;
	/// [`SwapBuffers`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-swapbuffers)
	pub fn SwapBuffers(unnammedParam1: HDC) -> BOOL;
}
#[link(name = "Opengl32")]
extern "system" {
	/// [`wglCreateContext`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglcreatecontext)
	pub fn wglCreateContext(unnamedParam1: HDC) -> HGLRC;
	/// [`wglDeleteContxt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wgldeletecontext)
	pub fn wglDeleteContext(unnamedParam1: HGLRC) -> BOOL;
	/// [`wglGetProcAddress`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress)
	pub fn wglGetProcAddress(unnamedParam1: LPCSTR) -> PROC;
	/// [`wglMakeCurrent`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglmakecurrent)
	pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> BOOL;

	/// [`glGenBuffers`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml)
	pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
	/// [`glBufferData`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml)
	pub fn glBufferData(
		target: GLenum,
		size: GLsizeiptr,
		data: *const core::ffi::c_void,
		usage: GLenum,
	);
}
//triangle part
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLfixed = i32;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLsizei = u32;
pub type GLenum = u32;
pub type GLintptr = isize;
pub type GLsizeiptr = usize;
pub type GLsync = usize;
pub type GLbitfield = i32;
//pub type GLhalf;
pub type GLfloat = f32;
pub type GLclampf = f32; // need f32 clamp function?
pub type GLdouble = f64;
pub type GLclampd = f64; // need f64 clamp function?

pub const GL_BUFFER_SIZE: u32 = 0x8764;
pub const GL_BUFFER_USAGE: u32 = 0x8765;
pub const GL_QUERY_COUNTER_BITS: u32 = 0x8864;
pub const GL_CURRENT_QUERY: u32 = 0x8865;
pub const GL_QUERY_RESULT: u32 = 0x8866;
pub const GL_QUERY_RESULT_AVAILABLE: u32 = 0x8867;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const GL_ARRAY_BUFFER_BINDING: u32 = 0x8894;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: u32 = 0x8895;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 0x889F;
pub const GL_READ_ONLY: u32 = 0x88B8;
pub const GL_WRITE_ONLY: u32 = 0x88B9;
pub const GL_READ_WRITE: u32 = 0x88BA;
pub const GL_BUFFER_ACCESS: u32 = 0x88BB;
pub const GL_BUFFER_MAPPED: u32 = 0x88BC;
pub const GL_BUFFER_MAP_POINTER: u32 = 0x88BD;
pub const GL_STREAM_DRAW: u32 = 0x88E0;
pub const GL_STREAM_READ: u32 = 0x88E1;
pub const GL_STREAM_COPY: u32 = 0x88E2;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_STATIC_READ: u32 = 0x88E5;
pub const GL_STATIC_COPY: u32 = 0x88E6;
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const GL_DYNAMIC_READ: u32 = 0x88E9;
pub const GL_DYNAMIC_COPY: u32 = 0x88EA;
pub const GL_SAMPLES_PASSED: u32 = 0x8914;
pub const GL_SRC1_ALPHA: u32 = 0x8589;

//end triangle part
pub type HGLRC = HANDLE;
/// Type for [wglGetExtensionsStringARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_extensions_string.txt)
pub type wglGetExtensionsStringARB_t = Option<unsafe extern "system" fn(HDC) -> *const c_char>;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_NUMBER_PIXEL_FORMATS_ARB: c_int = 0x2000;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_DRAW_TO_WINDOW_ARB: c_int = 0x2001;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_DRAW_TO_BITMAP_ARB: c_int = 0x2002;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ACCELERATION_ARB: c_int = 0x2003;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_NEED_PALETTE_ARB: c_int = 0x2004;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_NEED_SYSTEM_PALETTE_ARB: c_int = 0x2005;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SWAP_LAYER_BUFFERS_ARB: c_int = 0x2006;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SWAP_METHOD_ARB: c_int = 0x2007;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_NUMBER_OVERLAYS_ARB: c_int = 0x2008;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_NUMBER_UNDERLAYS_ARB: c_int = 0x2009;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TRANSPARENT_ARB: c_int = 0x200A;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TRANSPARENT_RED_VALUE_ARB: c_int = 0x2037;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TRANSPARENT_GREEN_VALUE_ARB: c_int = 0x2038;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TRANSPARENT_BLUE_VALUE_ARB: c_int = 0x2039;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TRANSPARENT_ALPHA_VALUE_ARB: c_int = 0x203A;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TRANSPARENT_INDEX_VALUE_ARB: c_int = 0x203B;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SHARE_DEPTH_ARB: c_int = 0x200C;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SHARE_STENCIL_ARB: c_int = 0x200D;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SHARE_ACCUM_ARB: c_int = 0x200E;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SUPPORT_GDI_ARB: c_int = 0x200F;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SUPPORT_OPENGL_ARB: c_int = 0x2010;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_DOUBLE_BUFFER_ARB: c_int = 0x2011;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_STEREO_ARB: c_int = 0x2012;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_PIXEL_TYPE_ARB: c_int = 0x2013;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_COLOR_BITS_ARB: c_int = 0x2014;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_RED_BITS_ARB: c_int = 0x2015;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_RED_SHIFT_ARB: c_int = 0x2016;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_GREEN_BITS_ARB: c_int = 0x2017;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_GREEN_SHIFT_ARB: c_int = 0x2018;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_BLUE_BITS_ARB: c_int = 0x2019;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_BLUE_SHIFT_ARB: c_int = 0x201A;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ALPHA_BITS_ARB: c_int = 0x201B;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ALPHA_SHIFT_ARB: c_int = 0x201C;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ACCUM_BITS_ARB: c_int = 0x201D;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ACCUM_RED_BITS_ARB: c_int = 0x201E;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ACCUM_GREEN_BITS_ARB: c_int = 0x201F;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ACCUM_BLUE_BITS_ARB: c_int = 0x2020;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_ACCUM_ALPHA_BITS_ARB: c_int = 0x2021;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_DEPTH_BITS_ARB: c_int = 0x2022;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_STENCIL_BITS_ARB: c_int = 0x2023;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_AUX_BUFFERS_ARB: c_int = 0x2024;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_NO_ACCELERATION_ARB: c_int = 0x2025;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_GENERIC_ACCELERATION_ARB: c_int = 0x2026;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_FULL_ACCELERATION_ARB: c_int = 0x2027;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SWAP_EXCHANGE_ARB: c_int = 0x2028;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SWAP_COPY_ARB: c_int = 0x2029;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_SWAP_UNDEFINED_ARB: c_int = 0x202A;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TYPE_RGBA_ARB: c_int = 0x202B;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
pub const WGL_TYPE_COLORINDEX_ARB: c_int = 0x202C;
/// Defined in [WGL_ARB_pixel_format](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)

/// Defined in [EXT_framebuffer_sRGB](https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_framebuffer_sRGB.txt)
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_EXT: c_int = 0x20A9;

/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const WGL_SAMPLE_BUFFERS_ARB: c_int = 0x2041;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const WGL_SAMPLES_ARB: c_int = 0x2042;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const MULTISAMPLE_ARB: c_int = 0x809D;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const SAMPLE_ALPHA_TO_COVERAGE_ARB: c_int = 0x809E;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const SAMPLE_ALPHA_TO_ONE_ARB: c_int = 0x809F;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const SAMPLE_COVERAGE_ARB: c_int = 0x80A0;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const MULTISAMPLE_BIT_ARB: c_int = 0x20000000;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const SAMPLE_BUFFERS_ARB: c_int = 0x80A8;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const SAMPLES_ARB: c_int = 0x80A9;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const SAMPLE_COVERAGE_VALUE_ARB: c_int = 0x80AA;
/// Defined in [ARB_multisample](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt)
pub const SAMPLE_COVERAGE_INVERT_ARB: c_int = 0x80AB;

/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_LAYER_PLANE_ARB: c_int = 0x2093;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_FLAGS_ARB: c_int = 0x2094;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_DEBUG_BIT_ARB: c_int = 0x0001;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_int = 0x0002;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x00000001;
/// Defined in [WGL_ARB_create_context](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_int = 0x00000002;

/// [`MAKEINTRESOURCEW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR {
	i as ULONG_PTR as LPWSTR
}
/// The predefined cursor styles.
pub enum IDCursor {
	/// Standard arrow and small hourglass
	AppStarting = 32650,
	/// Standard arrow
	Arrow = 32512,
	/// Crosshair
	Cross = 32515,
	/// Hand
	Hand = 32649,
	/// Arrow and question mark
	Help = 32651,
	/// I-Beam
	IBeam = 32513,
	/// Slashed circle
	No = 32648,
	/// Four-pointed arrow pointing north, south, east, and west
	SizeAll = 32646,
	/// Double-pointed arrow pointing northeast and southwest
	SizeNeSw = 32643,
	/// Double-pointed arrow pointing north and south
	SizeNS = 32645,
	/// Double-pointed arrow pointing northwest and southeast
	SizeNwSe = 32642,
	/// Double-pointed arrow pointing west and east
	SizeWE = 32644,
	/// Vertical arrow,
	UpArrow = 32516,
	/// Hourglass
	Wait = 32514,
}
/// See [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor {
	_3dDarkShadow = 21,
	_3dLight = 22,
	ActiveBorder = 10,
	ActiveCaption = 2,
	AppWorkspace = 12,
	/// Button face, also "3D face" color.
	ButtonFace = 15,
	/// Button highlight, also "3D highlight" color.
	ButtonHighlight = 20,
	/// Button shadow, also "3D shadow" color.
	ButtonShadow = 16,
	ButtonText = 18,
	CaptionText = 9,
	/// Desktop background color
	Desktop = 1,
	GradientActiveCaption = 27,
	GradientInactiveCaption = 28,
	GrayText = 17,
	Highlight = 13,
	HighlightText = 14,
	HotLight = 26,
	InactiveBorder = 11,
	InactiveCaption = 3,
	InactiveCaptionText = 19,
	InfoBackground = 24,
	InfoText = 23,
	Menu = 4,
	MenuHighlight = 29,
	MenuBar = 30,
	MenuText = 7,
	ScrollBar = 0,
	Window = 5,
	WindowFrame = 6,
	WindowText = 8,
}

pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;

/// Allocates a unique device context for each window in the class.
pub const CS_OWNDC: u32 = 0x0020;
/// Redraws the entire window if a movement or size adjustment changes the width
/// of the client area.
pub const CS_HREDRAW: u32 = 0x0002;
/// Redraws the entire window if a mouvement or size adjustment changes the
/// height of the client area
pub const CS_VREDRAW: u32 = 0x0001;
pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
pub const COLOR_WINDOW: u32 = 5;
pub const IDOK: c_int = 1;
pub const MB_OKCANCEL: u32 = 0x00000001;

pub const WM_CLOSE: u32 = 0x0010;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_NCDESTROY: u32 = 0x0082;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_QUIT: u32 = 0x0012;

/// Excludes the area occupied by child windows when drawing occurs within the
/// parent window.
///
/// This style is used when creating the parent window.
pub const WS_CLIPCHILDREN: u32 = 0x020_0000;
/// Clips child windows relative to each other.
///
/// That is, when a particular child window receives a WM_PAINT message,
/// the WS_CLIPSIBLINGS style clips all other overlapping child windows out of
/// the region of the child window to be updated. If WS_CLIPSIBLINGS is not
/// specified and child windows overlap, it is possible, when drawing within the
/// client area of a child window, to draw within the client area of a
/// neighboring child window.
pub const WS_CLIPSIBILIGS: u32 = 0x0400_0000;
pub const WS_OVERLAPPED: u32 = 0x0000_0000;
pub const WS_CAPTION: u32 = 0x00C0_0000;
pub const WS_SYSMENU: u32 = 0x0008_0000;
pub const WS_THICKFRAME: u32 = 0x0004_0000;
pub const WS_MINIMIZEBOX: u32 = 0x0002_0000;
pub const WS_MAXIMIZEBOX: u32 = 0x0001_0000;
pub const WS_OVERLAPPEDWINDOW: u32 =
	WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
// reminder bitwise OR |
pub const SW_SHOW: c_int = 5;
pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);
pub const GWLP_USERDATA: c_int = -21;

pub type ATOM = WORD;
pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type c_char = i8;
pub type c_int = i32;
pub type c_long = i32;
pub type c_uchar = u8;
pub type c_uint = u32;
pub type c_uint64 = u64;
pub type c_ulong = u32;
pub type c_ushort = u16;
pub type DWORD = c_ulong;
pub type HANDLE = PVOID;
pub type HBRUSH = HANDLE;
pub type HCURSOR = HICON;
pub type HDC = HANDLE;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HLOCAL = HANDLE;
pub type HMENU = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HWND = HANDLE;
pub type LONG = c_long;
pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LPCTSTR = LPCWSTR;
/// Pointer to an ANSI string.
pub type LPCSTR = *const c_char;
/// Pointer to a procedure of unknown type.
pub type PROC = *mut core::ffi::c_void;
pub type LPCWSTR = *const WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type LPCVOID = *const core::ffi::c_void;
pub type LPVOID = *mut core::ffi::c_void;
pub type LRESULT = LONG_PTR;
pub type PVOID = *mut core::ffi::c_void;
pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type ULONG_PTR = c_uint64;
pub type WCHAR = wchar_t;
pub type wchar_t = u16;
pub type WPARAM = UINT_PTR;
pub type WORD = c_ushort;
pub type va_list = *mut c_char;

pub type WNDPROC = Option<
	unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

#[repr(C)]
pub struct tagWNDCLASSW {
	pub style: UINT,
	pub lpfnWndProc: WNDPROC,
	pub cbClsExtra: c_int,
	pub cbWndExtra: c_int,
	pub hInstance: HINSTANCE,
	pub hIcon: HICON,
	pub hCursor: HCURSOR,
	pub hbrBackground: HBRUSH,
	pub lpszMenuName: LPCWSTR,
	pub lpszClassName: LPCWSTR,
}
pub type WNDCLASSW = tagWNDCLASSW;
unsafe_impl_default_zeroed!(tagWNDCLASSW);

#[repr(C)]
pub struct tagMSG {
	pub hwnd: HWND,
	pub message: UINT,
	pub wParam: WPARAM,
	pub lParam: LPARAM,
	pub time: DWORD,
	pub pt: POINT,
	pub lPrivate: DWORD,
}
pub type MSG = tagMSG;
pub type LPMSG = *const tagMSG;
unsafe_impl_default_zeroed!(tagMSG);

#[repr(C)]
pub struct tagPOINT {
	pub x: LONG,
	pub y: LONG,
}
pub type POINT = tagPOINT;
unsafe_impl_default_zeroed!(tagPOINT);

/// [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
#[repr(C)]
pub struct tagPAINTSTRUCT {
	pub hdc: HDC,
	pub fErase: BOOL,
	pub rcPaint: RECT,
	pub fRestore: BOOL,
	pub fIncUpdate: BOOL,
	pub rgbReserved: [BYTE; 32],
}
pub type LPPAINTSTRUCT = *const tagPAINTSTRUCT;
pub type PAINTSTRUCT = tagPAINTSTRUCT;
unsafe_impl_default_zeroed!(tagPAINTSTRUCT);

/// [`RECT`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct tagRECT {
	pub left: LONG,
	pub top: LONG,
	pub right: LONG,
	pub bottom: LONG,
}
pub type RECT = tagRECT;
unsafe_impl_default_zeroed!(tagRECT);

/// [`CREATESTRUCTW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
#[repr(C)]
pub struct tagCREATESTRUCTW {
	pub lpCreateParams: LPVOID,
	pub hInstance: HINSTANCE,
	pub hMenu: HMENU,
	pub hWndParent: HWND,
	pub cy: c_int,
	pub cx: c_int,
	pub y: c_int,
	pub x: c_int,
	pub style: LONG,
	pub lpszName: LPCWSTR,
	pub lpszClass: LPCWSTR,
	pub dwExStyle: DWORD,
}
pub type CREATESTRUCTW = tagCREATESTRUCTW;
pub type LPCREATESTRUCTW = *const tagCREATESTRUCTW;
unsafe_impl_default_zeroed!(tagCREATESTRUCTW);

/// [`PIXPIXELFORMATDESCRIPTOR`](https://docs.microsoft.com/en-us/windows/win32/api/Wingdi/ns-wingdi-pixelformatdescriptor)
#[derive(Debug)]
#[repr(C)]
pub struct tagPIXELFORMATDESCRIPTOR {
	pub nSize: WORD,
	pub nVersion: WORD,
	pub dwFlags: DWORD,
	pub iPixelType: BYTE,
	pub cColorBits: BYTE,
	pub cRedBits: BYTE,
	pub cRedShift: BYTE,
	pub cGreenBits: BYTE,
	pub cGreenShift: BYTE,
	pub cBlueBits: BYTE,
	pub cBlueShift: BYTE,
	pub cAlphaBits: BYTE,
	pub cAlphaShift: BYTE,
	pub cAccumBits: BYTE,
	pub cAccumRedBits: BYTE,
	pub cAccumGreenBits: BYTE,
	pub cAccumBlueBits: BYTE,
	pub cAccumAlphaBits: BYTE,
	pub cDepthBits: BYTE,
	pub cStencilBits: BYTE,
	pub cAuxBuffers: BYTE,
	pub iLayerType: BYTE,
	pub bReserved: BYTE,
	pub dwLayerMask: DWORD,
	pub dwVisibleMask: DWORD,
	pub dwDamageMask: DWORD,
}
pub type PIXELFORMATDESCRIPTOR = tagPIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *const tagPIXELFORMATDESCRIPTOR;

impl Default for PIXELFORMATDESCRIPTOR {
	#[inline]
	#[must_use]
	fn default() -> Self {
		let mut out: Self = unsafe { core::mem::zeroed() };
		out.nSize = core::mem::size_of::<Self>() as WORD;
		out.nVersion = 1;
		out
	}
}
/// [`PIXELFORMATDESCRIPTOR`] pixel type
pub const PFD_TYPE_RGBA: u8 = 0;
/// [`PIXELFORMATDESCRIPTOR`] pixel type
pub const PFD_TYPE_COLORINDEX: u8 = 1;

/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_MAIN_PLANE: u8 = 0;
/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_OVERLAY_PLANE: u8 = 1;
/// [`PIXELFORMATDESCRIPTOR`] layer type
pub const PFD_UNDERLAY_PLANE: u8 = u8::MAX /* was (-1) */;

pub const PFD_DOUBLEBUFFER: u32 = 0x00000001;
pub const PFD_STEREO: u32 = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: u32 = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: u32 = 0x00000008;
pub const PFD_SUPPORT_GDI: u32 = 0x00000010;
pub const PFD_SUPPORT_OPENGL: u32 = 0x00000020;
pub const PFD_GENERIC_FORMAT: u32 = 0x00000040;
pub const PFD_NEED_PALETTE: u32 = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: u32 = 0x00000100;
pub const PFD_SWAP_EXCHANGE: u32 = 0x00000200;
pub const PFD_SWAP_COPY: u32 = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: u32 = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: u32 = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: u32 = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: u32 = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: u32 = 0x00008000;

/// use with [`ChoosePixelFormat`] only
pub const PFD_DEPTH_DONTCARE: u32 = 0x20000000;
/// use with [`ChoosePixelFormat`] only
pub const PFD_DOUBLEBUFFER_DONTCARE: u32 = 0x40000000;
/// use with [`ChoosePixelFormat`] only
pub const PFD_STEREO_DONTCARE: u32 = 0x80000000;

#[repr(transparent)]
pub struct Win32Error(pub DWORD);
impl Win32Error {
	pub const APPLICATION_ERROR_BIT: DWORD = 1 << 29;
}
impl core::fmt::Debug for Win32Error {
	/// Displays the error using `FormatMessageW`
	///
	/// ```
	/// use triangle_from_scratch::win32::*;
	/// let s = format!("{:?}", Win32Error(0));
	/// assert_eq!("The operation completed successfully.  ", s);
	/// let app_error = format!("{:?}", Win32Error(1 << 29));
	/// assert_eq!("Win32ApplicationError(536870912)", app_error);
	/// ```
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		if self.0 & Self::APPLICATION_ERROR_BIT > 0 {
			return write!(f, "Win32ApplicationError({})", self.0);
		}
		let dwFlags =
			FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS;
		let mut buffer: *mut u16 = null_mut();
		let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;
		let tchar_count_excluding_null =
			unsafe { FormatMessageW(dwFlags, null_mut(), self.0, 0, lpBuffer, 0, null_mut()) };
		match tchar_count_excluding_null == 0 || buffer.is_null() {
			// some sort of problem happened, we can't usefully get_last_error since
			// Display formatting doesn't let you give an error value.
			true => return Err(core::fmt::Error),
			_ => {
				struct OnDropLocalFree(HLOCAL);
				impl Drop for OnDropLocalFree {
					fn drop(&mut self) {
						unsafe { LocalFree(self.0) };
					}
				}
				let _on_drop = OnDropLocalFree(buffer as HLOCAL);
				let buffer_slice: &[u16] =
					unsafe { core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize) };
				for decode_result in core::char::decode_utf16(buffer_slice.iter().copied()) {
					match decode_result {
						Ok('\r') | Ok('\n') => write!(f, " ")?,
						Ok(ch) => write!(f, "{}", ch)?,
						Err(_) => write!(f, "�")?,
					}
				}
			}
		}
		match f.alternate() {
			true => return write!(f, "WinError({})", self.0),
			_ => (),
		}
		Ok(())
	}
}
impl core::fmt::Display for Win32Error {
	/// Same as `Debug` impl
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:?}", self)
	}
}
impl std::error::Error for Win32Error {}

// Rust UTF-8 to Windows C UTF-16 NULL terminated
pub fn wide_null(s: &str) -> Vec<u16> {
	s.encode_utf16().chain(Some(0)).collect()
}

pub fn load_predefined_cursor(cursor: IDCursor) -> Result<HCURSOR, Win32Error> {
	// Safety: The enum only allows values from approved list. See MSDN.
	let hcursor = unsafe { LoadCursorW(null_mut(), MAKEINTRESOURCEW(cursor as WORD)) };
	match hcursor.is_null() {
		true => Err(get_last_error()),
		false => Ok(hcursor),
	}
}

pub unsafe fn register_class(window_class: &WNDCLASSW) -> Result<ATOM, Win32Error> {
	let atom = RegisterClassW(window_class);
	match atom == 0 {
		true => Err(get_last_error()),
		false => Ok(atom),
	}
}
pub fn get_process_handle() -> HMODULE {
	// Safety: as per the MSDN docs.
	unsafe { GetModuleHandleW(null()) }
}

/// Gets the thread-local last-error code value.
///
/// See [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> Win32Error {
	Win32Error(unsafe { GetLastError() })
}

/// Creates a window.
///
/// See [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
pub unsafe fn create_app_window(
	class_name: &str,
	window_name: &str,
	position: Option<[i32; 2]>,
	[width, height]: [i32; 2],
	create_param: LPVOID,
) -> Result<HWND, Win32Error> {
	let (x, y) = match position {
		None => (CW_USEDEFAULT, CW_USEDEFAULT),
		Some([x, y]) => {
			let chk = |p| match p {
				0 => CW_USEDEFAULT,
				_ => p,
			};
			(chk(x), chk(y))
		}
	};
	let hwnd = CreateWindowExW(
		0,
		wide_null(class_name).as_ptr(),
		wide_null(window_name).as_ptr(),
		WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBILIGS,
		x,
		y,
		width,
		height,
		null_mut(),
		null_mut(),
		get_process_handle(),
		create_param,
	);
	match hwnd.is_null() {
		true => Err(get_last_error()),
		_ => Ok(hwnd),
	}
}

/// Get a message from the threads's message queue.
/// or it can  be a non-window message as well.
///
/// See [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
#[inline(always)]
pub fn get_any_message() -> Result<MSG, Win32Error> {
	let mut msg = MSG::default();
	let output = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
	match output {
		-1 => Err(get_last_error()),
		_ => Ok(msg),
	}
}

/// Translates virtual-key messages into character messages.
///
/// The character messages go into your thread's message queue,
/// and you'll see them if you continue to consume messages.
///
/// **Returns:**
/// * `true` if the message was `WM_KEYDOWN`, `WM_KEYUP`, `WM_SYSKEYDOWN`, or
///   `WM_SYSKEYUP`.
/// * `true` for any other message type that generated a character message.
/// * otherwise `false`
///
/// See [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
pub fn translate_message(msg: &MSG) -> bool {
	0 != unsafe { TranslateMessage(msg) }
}

/// Sets the thread-local last-error code value.
///
/// See [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
pub fn set_last_error(e: Win32Error) {
	unsafe { SetLastError(e.0) }
}

/// Sets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The previous userdata pointer.
///
/// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
pub unsafe fn set_window_userdata<T>(hwnd: HWND, ptr: *mut T) -> Result<*mut T, Win32Error> {
	set_last_error(Win32Error(0));
	let out = SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as LONG_PTR);
	match out {
		0 => {
			let last_error = get_last_error();
			// if output is 0, it's only a real" error is last_error is non-zero
			match last_error.0 {
				0 => Ok(out as *mut T),
				_ => Err(last_error),
			}
		}
		_ => Ok(out as *mut T),
	}
}

/// Gets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The userdata pointer.
///
/// [`GetWindowLongPtrW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
pub unsafe fn get_window_userdata<T>(hwnd: HWND) -> Result<*mut T, Win32Error> {
	set_last_error(Win32Error(0));
	let out = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
	match out {
		0 => {
			let last_error = get_last_error();
			// if output is 0, it's only a real" error is last_error is non-zero
			match last_error.0 {
				0 => Ok(out as *mut T),
				_ => Err(last_error),
			}
		}
		_ => Ok(out as *mut T),
	}
}

/// Indicates to the system that a thread has made a request to terminate
/// (quit).
/// the exit code becomes the `wparam` of the [`WM_Quit`] message your message
/// loop eventually gets.
///
/// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
pub fn post_quit_message(exit_code: c_int) {
	unsafe { PostQuitMessage(exit_code) }
}

/// Prepares the specified window for painting.
///
/// On success: you get back both the [`HDC`] and [`PAINTSTRUCT`]
/// that you'll need for future painting calls (including [`EndPaint`]).
///
/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
pub unsafe fn begin_paint(hwnd: HWND) -> Result<(HDC, PAINTSTRUCT), Win32Error> {
	let mut ps = PAINTSTRUCT::default();
	let hdc = BeginPaint(hwnd, &mut ps);
	match hdc {
		h if h.is_null() => Err(get_last_error()),
		_ => Ok((hdc, ps)),
	}
}

/// Fills a rectangle with the given system color.
///
/// When filling the specified rectangle, this does **not** include the
/// rectangle's right and bottom sides, GDI fills a rectangle up to, but not
/// including, the right column and bottom row, regardless of the current mapping mode.
///
/// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
pub unsafe fn fill_rect_with_sys_color(hdc: HDC, rect: &RECT, color: SysColor) -> Result<(), ()> {
	match FillRect(hdc, rect, (color as u32 + 1) as HBRUSH) {
		0 => Err(()),
		_ => Ok(()),
	}
}

/// See [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
pub unsafe fn end_paint(hwnd: HWND, ps: &PAINTSTRUCT) {
	EndPaint(hwnd, ps);
}

/// Performs [`begin_paint`] / [`end_paint`] around your closure.
pub unsafe fn do_some_painting<F, T>(hwnd: HWND, f: F) -> Result<T, Win32Error>
where
	F: FnOnce(HDC, bool, RECT) -> Result<T, Win32Error>,
{
	let (hdc, ps) = begin_paint(hwnd)?;
	let output = f(hdc, ps.fErase != 0, ps.rcPaint);
	end_paint(hwnd, &ps);
	output
}

/// See [`ChoosePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat)
pub unsafe fn choose_pixel_format(
	hdc: HDC,
	ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<c_int, Win32Error> {
	let index = ChoosePixelFormat(hdc, ppfd);
	match index {
		0 => Err(get_last_error()),
		_ => Ok(index),
	}
}

/// See [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
pub unsafe fn get_dc(hwnd: HWND) -> Option<HDC> {
	let hdc = GetDC(hwnd);
	match hdc.is_null() {
		true => None,
		_ => Some(hdc),
	}
}

/// See [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
#[must_use]
pub unsafe fn release_dc(hwnd: HWND, hdc: HDC) -> bool {
	let was_released = ReleaseDC(hwnd, hdc);
	was_released != 0
}

/// See [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
pub unsafe fn destroy_window(hwnd: HWND) -> Result<(), Win32Error> {
	let destroyed = DestroyWindow(hwnd);
	match destroyed {
		0 => Err(get_last_error()),
		_ => Ok(()),
	}
}

/// Sets the pixel format of an HDC.
///
/// * If it's a window's HDC then it sets the pixel format of the window.
/// * You can't set a window's pixel format more than once.
/// * Call this *before* creating an OpenGL context.
/// * OpenGL windows should use [`WS_CLIPCHILDREN`] and [`WS_CLIPSIBLINGS`]
/// * OpenGL windows should *not* use `CS_PARENTDC`
///
/// See [`SetPixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixelformat)
pub unsafe fn set_pixel_format(
	hdc: HDC,
	format: c_int,
	ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<(), Win32Error> {
	let success = SetPixelFormat(hdc, format, ppfd);
	match success {
		0 => Err(get_last_error()),
		_ => Ok(()),
	}
}

/// Gets the maximum pixel format index for the HDC.
///
/// Pixel format indexes are 1-based.
///
/// To print out info on all the pixel formats you'd do something like this:
/// ```no_run
/// # use triangle_from_scratch::win32::*;
/// let hdc = todo!("create a window to get an HDC");
/// let max = unsafe { get_max_pixel_format_index(hdc).unwrap() };
/// for index in 1..=max {
///   let pfd = unsafe { describe_pixel_format(hdc, index).unwrap() };
///   todo!("print the pfd info you want to know");
/// }
/// ```
///
/// See [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
pub unsafe fn get_max_pixel_format_index(hdc: HDC) -> Result<c_int, Win32Error> {
	let max_index = DescribePixelFormat(
		hdc,
		1,
		std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
		null_mut(),
	);
	match max_index {
		0 => Err(get_last_error()),
		_ => Ok(max_index),
	}
}

/// Gets the pixel format info for a given pixel format index.
///
/// See [`DescribePixelFormat`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat)
pub unsafe fn describe_pixel_format(
	hdc: HDC,
	format: c_int,
) -> Result<PIXELFORMATDESCRIPTOR, Win32Error> {
	let mut pfd = PIXELFORMATDESCRIPTOR::default();
	let max_index = DescribePixelFormat(
		hdc,
		format,
		std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
		&mut pfd,
	);
	match max_index {
		0 => Err(get_last_error()),
		_ => Ok(pfd),
	}
}

/// Un-registers the window class from the `HINSTANCE` given.
///
/// * The name must be the name of a registered window class.
/// * This requires re-encoding the name to null-terminated utf-16, which
///   allocates. Using [`unregister_class_by_atom`] instead does not allocate,
///   if you have the atom available.
/// * Before calling this function, an application must destroy all windows
///   created with the specified class.
///
/// See
/// [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
pub unsafe fn unregister_class_by_name(name: &str, instance: HINSTANCE) -> Result<(), Win32Error> {
	let name_null = wide_null(name);
	let out = UnregisterClassW(name_null.as_ptr(), instance);
	match out {
		0 => Err(get_last_error()),
		_ => Ok(()),
	}
}
/// Un-registers the window class from the `HINSTANCE` given.
///
/// * The atom must be the atom of a registered window class.
/// * Before calling this function, an application must destroy all windows
///   created with the specified class.
///
/// See [`UnregisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
pub unsafe fn unregister_class_by_atom(a: ATOM, instance: HINSTANCE) -> Result<(), Win32Error> {
	let out = UnregisterClassW(a as LPCWSTR, instance);
	match out {
		0 => Err(get_last_error()),
		_ => Ok(()),
	}
}

/// See [`wglCreateContext`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglcreatecontext)
pub unsafe fn wgl_create_context(hdc: HDC) -> Result<HGLRC, Win32Error> {
	let hglrc = wglCreateContext(hdc);
	match hglrc {
		_h if _h.is_null() => Err(get_last_error()),
		_ => Ok(hglrc),
	}
}

/// Deletes a GL Context.
///
/// * You **cannot** use this to delete a context current in another thread.
/// * You **can** use this to delete the current thread's context. The context
///   will be made not-current automatically before it is deleted.
///
/// See
/// [`wglDeleteContext`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wgldeletecontext)
pub unsafe fn wgl_delete_context(hglrc: HGLRC) -> Result<(), Win32Error> {
	let success = wglDeleteContext(hglrc);
	match success {
		0 => Err(get_last_error()),
		_ => Ok(()),
	}
}

/// Makes a given HGLRC current in the thread and targets it at the HDC given.
///
/// * You can safely pass `null_mut` for both parameters if you wish to make no
///   context current in the thread.
///
/// See
/// [`wglMakeCurrent`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglmakecurrent)
pub unsafe fn wgl_make_current(hdc: HDC, hglrc: HGLRC) -> Result<(), Win32Error> {
	let success = wglMakeCurrent(hdc, hglrc);
	match success {
		0 => Err(get_last_error()),
		_ => Ok(()),
	}
}

/// Gets a GL function address.
///
/// The input should be a null-terminated function name string. Use the
/// [`c_str!`] macro for assistance.
///
/// * You must have an active GL context for this to work. Otherwise you will
///   always get an error.
/// * The function name is case sensitive, and spelling must be exact.
/// * All outputs are context specific. Functions supported in one rendering
///   context are not necessarily supported in another.
/// * The extension function addresses are unique for each pixel format. All
///   rendering contexts of a given pixel format share the same extension
///   function addresses.
///
/// This *will not* return function pointers exported by `OpenGL32.dll`, meaning
/// that it won't return OpenGL 1.1 functions. For those old function, use
/// [`GetProcAddress`].
pub fn wgl_get_proc_address(func_name: &[u8]) -> Result<PROC, Win32Error> {
	// check that we end the slice with a \0 as expected.
	match func_name.last() {
		Some(b'\0') => (),
		_ => return Err(get_last_error()),
	}
	// Safety: We've checked that the end of the slice is null-terminated.
	let proc = unsafe { wglGetProcAddress(func_name.as_ptr().cast()) };
	match proc as usize {
		// Some non-zero values can also be errors,
		// https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions#Windows
		0 | 1 | 2 | 3 | usize::MAX => return Err(Win32Error(Win32Error::APPLICATION_ERROR_BIT)),
		_ => Ok(proc),
	}
}

/// Gets the WGL extension string for the HDC passed.
///
/// * This relies on [`wgl_get_proc_address`], and so you must have a context
///   current for it to work.
/// * If `wgl_get_proc_address` fails then an Application Error is generated.
/// * If `wgl_get_proc_address` succeeds but the extension string can't be
///   obtained for some other reason you'll get a System Error.
///
/// The output is a space-separated list of extensions that are supported.
///
/// See
/// [`wglGetExtensionsStringARB`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_extensions_string.txt)
pub unsafe fn wgl_get_extension_string_arb(hdc: HDC) -> Result<String, Win32Error> {
	let f: wglGetExtensionsStringARB_t =
		core::mem::transmute(wgl_get_proc_address(c_str!("wglGetExtensionsStringARB"))?);
	let p: *const u8 = (f.ok_or(Win32Error(Win32Error::APPLICATION_ERROR_BIT))?)(hdc).cast();
	match p {
		_n if p.is_null() => Err(get_last_error()),
		_ => {
			let bytes = gather_null_terminated_bytes(p);
			Ok(min_alloc_lossy_into_string(bytes))
		}
	}
}

/// Grabs out the stuff you'll need to have fun with WGL
pub fn get_wgl_basics() -> Result<
	(
		Vec<String>,
		wglChoosePixelFormatARB_t,
		wglCreateContextAttribsARB_t,
		wglSwapIntervalEXT_t,
	),
	Win32Error,
> {
	let instance = get_process_handle();
	let fake_window_class = "Fake Window Class";
	let fake_window_class_wn = wide_null(fake_window_class);
	let mut fake_wc = WNDCLASSW::default();
	fake_wc.style = CS_OWNDC;
	fake_wc.lpfnWndProc = Some(DefWindowProcW);
	fake_wc.hInstance = get_process_handle();
	fake_wc.lpszClassName = fake_window_class_wn.as_ptr();

	let fake_atom = unsafe { register_class(&fake_wc) }.unwrap();

	let pfd = PIXELFORMATDESCRIPTOR {
		dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
		iPixelType: PFD_TYPE_RGBA,
		cColorBits: 32,
		cDepthBits: 24,
		cStencilBits: 8,
		iLayerType: PFD_MAIN_PLANE,
		..Default::default()
	};

	let fake_hwnd =
		unsafe { create_app_window(fake_window_class, "Fake Window", None, [1, 1], null_mut()) }
			.unwrap();
	let fake_hdc = unsafe { get_dc(fake_hwnd) }.unwrap();
	let pf_index = unsafe { choose_pixel_format(fake_hdc, &pfd) }.unwrap();
	unsafe { set_pixel_format(fake_hdc, pf_index, &pfd) }.unwrap();
	let fake_hglrc = unsafe { wgl_create_context(fake_hdc) }.unwrap();
	unsafe { wgl_make_current(fake_hdc, fake_hglrc) }.unwrap();

	match unsafe { describe_pixel_format(fake_hdc, pf_index) } {
		Ok(pfd) => println!("{:#?}", pfd),
		_ => println!("Error: Couldn't get pixel format description."),
	}

	let extensions: Vec<String> = unsafe { wgl_get_extension_string_arb(fake_hdc) }
		.map(|s| {
			s.split(' ')
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string())
				.collect()
		})
		.unwrap_or(Vec::new());

	let wglChoosePixelFormatARB: wglChoosePixelFormatARB_t = unsafe {
		core::mem::transmute(wgl_get_proc_address(c_str!("wglChoosePixelFormatARB")).unwrap())
	};
	let wglCreateContextAttribsARB: wglCreateContextAttribsARB_t = unsafe {
		core::mem::transmute(wgl_get_proc_address(c_str!("wglCreateContextAttribsARB")).unwrap())
	};
	let wglSwapIntervalEXT: wglSwapIntervalEXT_t =
		unsafe { core::mem::transmute(wgl_get_proc_address(c_str!("wglSwapIntervalEXT")).unwrap()) };

	unsafe { wgl_make_current(null_mut(), null_mut()) }.unwrap();
	unsafe { wgl_delete_context(fake_hglrc) }.unwrap();
	assert!(unsafe { release_dc(fake_hwnd, fake_hdc) });
	unsafe { destroy_window(fake_hwnd) }.unwrap();
	unsafe { unregister_class_by_atom(fake_atom, instance) }.unwrap();

	Ok((
		extensions,
		wglChoosePixelFormatARB,
		wglCreateContextAttribsARB,
		wglSwapIntervalEXT,
	))
}

/// Arranges the data for calling a [`wglChoosePixelFormatARB_t`] procedure.
///
/// * Inputs are slices of [key, value] pairs.
/// * Input slices **can** be empty.
/// * Non-empty slices must have a zero value in the key position of the final
///   pair.
pub unsafe fn do_wglChoosePixelFormatARB(
	f: wglChoosePixelFormatARB_t,
	hdc: HDC,
	int_attrs: &[[c_int; 2]],
	float_attrs: &[[FLOAT; 2]],
) -> Result<c_int, Win32Error> {
	let app_err = Win32Error(Win32Error::APPLICATION_ERROR_BIT);
	let i_ptr = match int_attrs.last() {
		Some([k, _v]) => match *k {
			0 => int_attrs.as_ptr(),
			_ => return Err(app_err),
		},
		None => null(),
	};
	let f_ptr = match float_attrs.last() {
		Some([k, _v]) => match *k {
			0.0 => float_attrs.as_ptr(),
			_ => return Err(app_err),
		},
		None => null(),
	};
	let mut out_format = 0;
	let mut out_format_count = 0;
	let b = (f.ok_or(app_err)?)(
		hdc,
		i_ptr.cast(),
		f_ptr.cast(),
		1,
		&mut out_format,
		&mut out_format_count,
	);
	match b {
		b if b != 0 && out_format_count == 1 => Ok(out_format),
		_ => Err(get_last_error()),
	}
}

/// Arranges the data for calling a [`wglCreateContextAttribsARB_t`] procedure.
///
/// * The input slice consists of [key, value] pairs.
/// * The input slice **can** be empty.
/// * Any non-empty input must have zero as the key value of the last position.
pub unsafe fn do_wglCreateContextAttribsARB(
	f: wglCreateContextAttribsARB_t,
	hdc: HDC,
	hShareContext: HGLRC,
	attribList: &[[i32; 2]],
) -> Result<HGLRC, Win32Error> {
	let app_err = Win32Error(Win32Error::APPLICATION_ERROR_BIT);
	let i_ptr = match attribList.last() {
		Some([k, _v]) => match *k {
			0 => attribList.as_ptr(),
			_ => return Err(app_err),
		},
		None => null(),
	};
	let hglrc = (f.ok_or(app_err)?)(hdc, hShareContext, i_ptr.cast());
	match hglrc.is_null() {
		true => Err(get_last_error()),
		_ => Ok(hglrc),
	}
}

/// Loads a dynamic library.
///
/// The precise details of how the library is searched for depend on the input
/// string.
///
/// See [`LoadLibraryW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
pub fn load_library(name: &str) -> Result<HMODULE, Win32Error> {
	let name_null = wide_null(name);
	//Safety: the input pointer is to a null-terminated string
	let hmodule = unsafe { LoadLibraryW(name_null.as_ptr()) };
	match hmodule.is_null() {
		true => Err(get_last_error()),
		_ => Ok(hmodule),
	}
}
