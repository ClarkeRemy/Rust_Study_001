#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate core;
//use core::ptr::{null, null_mut};
use lib::win32::*;
use lib::*;

struct WindowData {
	hdc: HDC,
	hglrc: HGLRC,
	opengl32: HMODULE,
	gl_clear: glClear_t,
	gl_clear_color: glClearColor_t,
}
impl WindowData {
	pub fn gl_get_proc_address(&self, name: &[u8]) -> *mut core::ffi::c_void {
		assert!(*name.last().unwrap() == 0);
		let p = unsafe { wglGetProcAddress(name.as_ptr().cast()) };
		match p as usize {
			0 | 1 | 2 | usize::MAX => unsafe { GetProcAddress(self.opengl32, name.as_ptr().cast()) },
			_ => p,
		}
	}
	#[rustfmt::skip]
	pub unsafe fn load_gl_functions(&mut self){
		self.gl_clear = core::mem::transmute(self.gl_get_proc_address(c_str!("glClear")));
		self.gl_clear_color = core::mem::transmute(self.gl_get_proc_address(c_str!("glClearColor")));
	}
}
impl Default for WindowData {
	fn default() -> Self {
		unsafe { core::mem::zeroed() }
	}
}

fn main() {
	let instance = get_process_handle();

	let (wgl_extentions, wglChoosePixelFormatARB, wglCreateContextAttribsARB, wglSwapIntervalEXT) =
		get_wgl_basics().unwrap();
	println!("> WGL Extentions: {:?}", wgl_extentions);

	let win_class_name: &'static str = "Sample Window Class";
	let wc_name_wide = wide_null(win_class_name.clone());

	let mut wc = WNDCLASSW::default();
	wc.style = CS_OWNDC;
	wc.lpfnWndProc = Some(window_procedure);
	wc.hInstance = get_process_handle();
	wc.lpszClassName = wc_name_wide.as_ptr();
	wc.hCursor = load_predefined_cursor(IDCursor::Arrow).unwrap();

	let atom = unsafe { register_class(&wc) }.unwrap_or_else(|_: Win32Error| {
		let last_error = get_last_error();
		panic!(
			"Could not register the window class, error code: {}",
			last_error
		);
	});

	let lparam: *mut WindowData = Box::leak(Box::new(WindowData::default()));
	let hwnd: HWND = unsafe {
		create_app_window(
			win_class_name,
			win_class_name,
			None,
			[800, 600],
			lparam.cast(),
		)
	}
	.unwrap();
	let hdc = unsafe { get_dc(hwnd) }.unwrap();
	unsafe { (*lparam).hdc = hdc };

	// base criteria
	let mut int_attribs = vec![
		[WGL_DRAW_TO_WINDOW_ARB, true as _],
		[WGL_SUPPORT_OPENGL_ARB, true as _],
		[WGL_DOUBLE_BUFFER_ARB, true as _],
		[WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB],
		[WGL_COLOR_BITS_ARB, 32],
		[WGL_DEPTH_BITS_ARB, 24],
		[WGL_STENCIL_BITS_ARB, 8],
	];
	// if sRGB is supported, ask for that
	match wgl_extentions.iter().any(|s| s == "WGL_ARB_multisample") {
		true => int_attribs.push([WGL_SAMPLE_BUFFERS_ARB, 1]),
		_ => (),
	};
	// finalize our list
	int_attribs.push([0, 0]);
	// choose a format, get the PIXELFORMATDESCRIPTOR, and set it.
	let pix_format =
		unsafe { do_wglChoosePixelFormatARB(wglChoosePixelFormatARB, hdc, &int_attribs, &[]) }.unwrap();
	let pfd = unsafe { describe_pixel_format(hdc, pix_format) }.unwrap();
	println!("> Selected Pixel Format: {:?}", pfd);
	unsafe { set_pixel_format(hdc, pix_format, &pfd) }.unwrap();

	// now we create a context.
	const FLAGS: c_int = WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB
		| match cfg!(debug_assertions) {
			true => WGL_CONTEXT_DEBUG_BIT_ARB,
			_ => 0,
		};
	let hglrc = unsafe {
		do_wglCreateContextAttribsARB(
			wglCreateContextAttribsARB,
			hdc,
			null_mut(),
			&[
				[WGL_CONTEXT_MAJOR_VERSION_ARB, 3],
				[WGL_CONTEXT_MINOR_VERSION_ARB, 3],
				[
					WGL_CONTEXT_PROFILE_MASK_ARB,
					WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
				],
				[WGL_CONTEXT_FLAGS_ARB, FLAGS],
				[0, 0],
			],
		)
	}
	.unwrap();
	unsafe { wgl_make_current(hdc, hglrc) }.unwrap();
	unsafe { (*lparam).hglrc = hglrc };

	let opengl32 = load_library("opengl32.dll").unwrap();
	unsafe { (*lparam).opengl32 = opengl32 };
	unsafe { (*lparam).load_gl_functions() };

	// Enable "adaptive" vsync if possible, otherwise normal vsync
	match wgl_extentions
		.iter()
		.any(|s| s == "WGL_EXT_swap_control_tear")
	{
		true => unsafe {
			(wglSwapIntervalEXT.unwrap())(-1);
		},
		_ => unsafe {
			(wglSwapIntervalEXT.unwrap())(1);
		},
	}
	// FINALLY A TRINAGLE

	let verticies: [f32; 9] = [
		-0.5, -0.5, 0.0, //
		0.5, -0.5, 0.0, //
		0.0, 0.5, 0.0, //
	];
	let mut VBO: c_uint = 0;
	
	// unsafe { glGenBuffers(1, &VBO as *const _ as *mut _) };
	// unsafe {
	// 	glBufferData(
	// 		GL_ARRAY_BUFFER,
	// 		std::mem::size_of::<f32>(),
	// 		verticies.as_ptr() as _,
	// 		GL_STATIC_DRAW,
	// 	)
	// };
	// // can't get these to work
	const vertexShaderSource: &'static [u8] = b"\
	        #version 330 core\n\
	        layout (location = 0) in vec3 aPos;\n\
					void main()\n\
					{\n  \
					gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0)\n\
					}\0";

	// TRIANGLE READY
	match hwnd.is_null() {
		true => panic!("failed to create a window."),
		_ => (),
	}
	let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };
	let mut msg = MSG::default();
	loop {
		match get_any_message() {
			Ok(msg) => match msg.message {
				WM_QUIT => std::process::exit(msg.wParam as i32),
				_ => {
					translate_message(&msg);
					unsafe {
						DispatchMessageW(&msg);
					}
				}
			},
			Err(e) => panic!("Error when getting from the message queue {}", e),
		}
	}
}

#[allow(non_camel_case_types, non_snake_case)]
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
					let ptr = (*createstruct).lpCreateParams as *mut i32;
					set_window_userdata::<i32>(hWnd, ptr).is_ok() as LRESULT
				}
			};
		}
		WM_CREATE => println!("Create"),
		WM_CLOSE => drop(DestroyWindow(hWnd)),
		WM_DESTROY => {
			match get_window_userdata::<WindowData>(hWnd) {
				Ok(ptr) if !ptr.is_null() => {
					let window_data = Box::from_raw(ptr);
					FreeLibrary(window_data.opengl32);
					wgl_delete_context(window_data.hglrc)
						.unwrap_or_else(|e| eprintln!("GL Context deletion error: {}", e));
				}
				Ok(_) => println!("userdata ptr is null, no cleanup"),
				Err(e) => println!("Error while getting the userdata ptr to clean it up: {}", e),
			}
			post_quit_message(0)
		}
		WM_PAINT => {
			match get_window_userdata::<WindowData>(hWnd) {
				Ok(ptr) if !ptr.is_null() => {
					let window_data = ptr.as_mut().unwrap();
					(window_data.gl_clear_color.unwrap())(0.6, 0.7, 0.8, 1.0);
					(window_data.gl_clear.unwrap())(GL_COLOR_BUFFER_BIT);
					SwapBuffers(window_data.hdc);
				}
				Ok(_) => println!("userdata is null"),
				Err(e) => println!("Error while getting the userdata ptr: {}", e),
			}
			match do_some_painting(hWnd, |hdc, _erase_bg, target_rect| {
				let _ = fill_rect_with_sys_color(hdc, &target_rect, SysColor::Window);
				Ok(())
			}) {
				Err(e) => println!("error during painting: {}", e),
				_ => (),
			};
		}
		_ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
	}
	0
}
