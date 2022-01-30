// https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types
// https://docs.microsoft.com/en-us/cpp/cpp/data-type-ranges?view=msvc-160
#![allow(non_camel_case_types, non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate core;
use core::ptr::{null, null_mut};

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
	/// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
	pub fn GetLastError() -> DWORD;
}

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

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

	/// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	pub fn DefWindowProcW(hwnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;

	/// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
	pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFillerMin: UINT, wMsgFillerMax: UINT) -> BOOL;

	/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
	pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

	/// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
	pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	pub fn DestroyWindow(hWnd: HWND) -> BOOL;

	/// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
	pub fn PostQuitMessage(nExitCode: c_int);

	/// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

	/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	pub fn BeginPaint(hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;

	/// [``](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
	pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;

	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox)
	pub fn MessageBox(hWnd: HWND, lpText: LPCTSTR, lpCaption: LPCTSTR, uType: UINT) -> c_int;

	/// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;

	/// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	fn GetWindowLongPtrW(hWnd:HWND, nIndex: c_int)->LONG_PTR;

	/// [`SetCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursor)
	fn SetCursor(hCursor: HCURSOR)->HCURSOR;
}

/// [`MAKEINTRESOURCEW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
pub const fn MAKEINTRESOURCEW(i: WORD) -> LPWSTR {
	i as ULONG_PTR as LPWSTR
}

pub unsafe extern "system" fn window_procedure(
	hWnd: HWND,
	Msg: UINT,
	wParam: WPARAM,
	lParam: LPARAM,
) -> LRESULT {
	match Msg {
		WM_NCCREATE => {
			println!("NC Create");
			let createstruct: *mut CREATESTRUCTW = lParam as *mut _;
			return match createstruct.is_null() {
				true => 0,
				false => {
					let boxed_i32_ptr: *mut i32 = (*createstruct).lpCreateParams.cast();
					SetWindowLongPtrW(hWnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);
					1
				}
			};
		},
		WM_CREATE => println!("Create"),
		WM_CLOSE => drop(DestroyWindow(hWnd)),
		WM_DESTROY => PostQuitMessage(0),
		WM_PAINT => {
			let ptr = GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *mut i32;
			println!("Current ptr: {}", *ptr);
			*ptr +=1;
			let mut ps = PAINTSTRUCT::default();
			let hdc = BeginPaint(hWnd, &ps);
			let _success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
			EndPaint(hWnd, &ps);
		}
		_ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
	}
	0
}

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

const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
const WS_OVERLAPPED: u32 = 0x00000000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const WS_OVERLAPPEDWINDOW: u32 =
	WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
// reminder bitwise OR |
const SW_SHOW: c_int = 5;
const IDC_ARROW: LPCWSTR = MAKEINTRESOURCEW(32512);
const GWLP_USERDATA: c_int = -21;

type ATOM = WORD;
type BOOL = c_int;
type BYTE = c_char;
type c_char = u8;
type c_int = i32;
type c_long = i32;
type c_uint = u32;
type c_uint64 = u64;
type c_ulong = u32;
type c_ushort = u16;
type DWORD = c_ulong;
type HANDLE = PVOID;
type HBRUSH = HANDLE;
type HCURSOR = HICON;
type HDC = HANDLE;
type HICON = HANDLE;
type HINSTANCE = HANDLE;
type HMENU = HANDLE;
type HMODULE = HINSTANCE;
type HWND = HANDLE;
type LONG = c_long;
type LONG_PTR = isize;
type LPARAM = LONG_PTR;
type LPCTSTR = LPCWSTR;
type LPCWSTR = *const WCHAR;
type LPWSTR = *mut WCHAR;
type LPVOID = *mut core::ffi::c_void;
type LRESULT = LONG_PTR;
type PVOID = *mut core::ffi::c_void;
type UINT = c_uint;
type UINT_PTR = usize;
type ULONG_PTR = c_uint64;
type WCHAR = wchar_t;
type wchar_t = u16;
type WPARAM = UINT_PTR;
type WORD = c_ushort;

type WNDPROC = Option<
	unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

#[repr(C)]
pub struct tagWNDCLASSW {
	style: UINT,
	lpfnWndProc: WNDPROC,
	cbClsExtra: c_int,
	cbWndExtra: c_int,
	hInstance: HINSTANCE,
	hIcon: HICON,
	hCursor: HCURSOR,
	hbrBackground: HBRUSH,
	lpszMenuName: LPCWSTR,
	lpszClassName: LPCWSTR,
}
type WNDCLASSW = tagWNDCLASSW;
unsafe_impl_default_zeroed!(tagWNDCLASSW);

#[repr(C)]
pub struct tagMSG {
	hwnd: HWND,
	message: UINT,
	wParam: WPARAM,
	lParam: LPARAM,
	time: DWORD,
	pt: POINT,
	lPrivate: DWORD,
}
type MSG = tagMSG;
type LPMSG = *const tagMSG;
unsafe_impl_default_zeroed!(tagMSG);

#[repr(C)]
pub struct tagPOINT {
	x: LONG,
	y: LONG,
}
type POINT = tagPOINT;
unsafe_impl_default_zeroed!(tagPOINT);

/// [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
#[repr(C)]
pub struct tagPAINTSTRUCT {
	hdc: HDC,
	fErase: BOOL,
	rcPaint: RECT,
	fRestore: BOOL,
	fIncUpdate: BOOL,
	rgbReserved: [BYTE; 32],
}
type LPPAINTSTRUCT = *const tagPAINTSTRUCT;
type PAINTSTRUCT = tagPAINTSTRUCT;
unsafe_impl_default_zeroed!(tagPAINTSTRUCT);

/// [`RECT`](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
pub struct tagRECT {
	left: LONG,
	top: LONG,
	right: LONG,
	bottom: LONG,
}
type RECT = tagRECT;
unsafe_impl_default_zeroed!(tagRECT);

/// [`CREATESTRUCTW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw)
#[repr(C)]
struct tagCREATESTRUCTW {
	lpCreateParams: LPVOID,
	hInstance: HINSTANCE,
	hMenu: HMENU,
	hWndParent: HWND,
	cy: c_int,
	cx: c_int,
	y: c_int,
	x: c_int,
	style: LONG,
	lpszName: LPCWSTR,
	lpszClass: LPCWSTR,
	dwExStyle: DWORD,
}
type CREATESTRUCTW = tagCREATESTRUCTW;
type LPCREATESTRUCTW = *const tagCREATESTRUCTW;
unsafe_impl_default_zeroed!(tagCREATESTRUCTW);

// Rust UTF-8 to Windows C UTF-16 NULL terminated
pub fn wide_null(s: &str) -> Vec<u16> {
	s.encode_utf16().chain(Some(0)).collect()
}

fn main() {
	let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
	let sample_window_class_wn = wide_null("Sample Window Class");

	let mut wc = WNDCLASSW::default();
	// wc.lpfnWndProc = Some(DefWindowProcW);
	wc.lpfnWndProc = Some(window_procedure); // ?
	wc.hInstance = hInstance;
	wc.lpszClassName = sample_window_class_wn.as_ptr();
	wc.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };

	let atom = unsafe { RegisterClassW(&wc) };
	if atom == 0 {
		let last_error = unsafe { GetLastError() };
		panic!(
			"Could not register the window class, error code: {}",
			last_error
		);
	}

	let lparam: *mut i32 = Box::leak(Box::new(5_i32));
	let hwnd: HWND = unsafe {
		CreateWindowExW(
			0,
			sample_window_class_wn.as_ptr(),
			sample_window_class_wn.as_ptr(),
			WS_OVERLAPPEDWINDOW,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			null_mut(),
			null_mut(),
			hInstance,
			lparam.cast(),
		)
	};
	if hwnd.is_null() {
		panic!("failed to create a window.");
	}
	let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };
	let mut msg = MSG::default();
	loop {
		let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
		match message_return {
			0 => break,
			-1 => panic!("Error with `GetMessageW`, error code: {}", unsafe {
				GetLastError()
			}),
			_ => unsafe {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			},
		}
	}
}
