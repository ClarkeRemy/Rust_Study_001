#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use triangle_from_scratch::win32::*;

fn main() {
  let sample_window_class = "Sample Window Class";
  let sample_window_class_wn = wide_null(sample_window_class);

  let mut wc = WNDCLASSW::default();
  wc.lpfnWndProc = Some(window_procedure);
  wc.hInstance = get_process_handle();
  wc.lpszClassName = sample_window_class_wn.as_ptr();
  wc.hCursor = load_predefined_cursor(IDCursor::Arrow).unwrap();

  let _atom = unsafe { register_class(&wc) }.unwrap();

  let lparam: *mut i32 = Box::leak(Box::new(5_i32));
  let hwnd = unsafe {
    create_app_window(
      sample_window_class,
      "Sample Window Name",
      None,
      [800, 600],
      lparam.cast(),
    )
  }
  .unwrap();
  let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

  loop {
    match get_any_message() {
      Ok(msg) => {
        if msg.message == WM_QUIT {
          std::process::exit(msg.wParam as i32);
        }
        translate_message(&msg);
        unsafe {
          DispatchMessageW(&msg);
        }
      }
      Err(e) => panic!("Error when getting from the message queue: {}", e),
    }
  }
}

pub unsafe extern "system" fn window_procedure(
  hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM,
) -> LRESULT {
  match msg {
    WM_NCCREATE => {
      println!("NC Create");
      let createstruct: *mut CREATESTRUCTW = lparam as *mut _;
      if createstruct.is_null() {
        return 0;
      }
      let ptr = (*createstruct).lpCreateParams as *mut i32;
      return set_window_userdata::<i32>(hwnd, ptr).is_ok() as LRESULT;
    }
    WM_CREATE => println!("Create"),
    WM_CLOSE => {
      let text_null = wide_null("Really quit?");
      let caption_null = wide_null("My Caption");
      let mb_output = MessageBoxW(
        hwnd,
        text_null.as_ptr(),
        caption_null.as_ptr(),
        MB_OKCANCEL,
      );
      if mb_output == IDOK {
        let _success = DestroyWindow(hwnd);
      }
    }
    WM_DESTROY => {
      match get_window_userdata::<i32>(hwnd) {
        Ok(ptr) if !ptr.is_null() => {
          Box::from_raw(ptr);
          println!("Cleaned up the box.");
        }
        Ok(_) => {
          println!("userdata ptr is null, no cleanup")
        }
        Err(e) => {
          println!("Error while getting the userdata ptr to clean it up: {}", e)
        }
      }
      post_quit_message(0);
    }
    WM_PAINT => {
      match get_window_userdata::<i32>(hwnd) {
        Ok(ptr) if !ptr.is_null() => {
          println!("Current ptr: {}", *ptr);
          *ptr += 1;
        }
        Ok(_) => {
          println!("userdata ptr is null")
        }
        Err(e) => {
          println!("Error while getting the userdata ptr: {}", e)
        }
      }
      do_some_painting(hwnd, |hdc, _erase_bg, target_rect| {
        let _ = fill_rect_with_sys_color(hdc, &target_rect, SysColor::Window);
        Ok(())
      })
      .unwrap_or_else(|e| println!("error during painting: {}", e));
    }
    _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
  }
  0
}
