// Let's put this so that it won't open console
//#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate winapi;
extern crate libc;
// https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/um/libloaderapi/index.html?search=winuser

mod game;
mod renderer;

use game::GameState;
use game::game_loop;


use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use std::io::Error;

use self::winapi::shared::windef::{
  HWND,
  RECT,
  LPRECT,
  HDC,
};
use self::winapi::shared::minwindef::{
  LPARAM,
  WPARAM,
  UINT,
  LRESULT,
};

use self::winapi::um::libloaderapi::GetModuleHandleW;
use self::winapi::um::winuser::{
  DefWindowProcW,
  RegisterClassW,
  CreateWindowExW,
  TranslateMessage,
  DispatchMessageW,
  PeekMessageW,
  PostQuitMessage,
  GetClientRect,
  ShowWindow,
  GetAsyncKeyState,
  GetDC,
};
use self::winapi::um::winuser::{
  MSG,
  WNDCLASSW,
  CS_OWNDC,
  CS_HREDRAW,
  CS_VREDRAW,
  CW_USEDEFAULT,
  WS_OVERLAPPEDWINDOW,
  WS_VISIBLE,
//    WM_PAINT,
  WM_DESTROY,
  WM_CREATE,
  PM_REMOVE,
  SW_HIDE,
  VK_ESCAPE,
};


use self::winapi::um::wincon::GetConsoleWindow;

use std::time::Duration;
use std::time::Instant;
use renderer::Renderer;

// ----------------------------------------------------

// We have to encode text to wide format for Windows
#[cfg(windows)]
fn win32_string(value: &str) -> *const u16 {
  let v: Vec<u16> = OsStr::new(value).encode_wide().chain(once(0)).collect();
  return v.as_ptr();
}

// Window struct
#[cfg(windows)]
struct Window {
  handle: HWND,
  dc: HDC,
}


fn hide_console_window() {
  unsafe {
    let window = GetConsoleWindow();
    if window != std::ptr::null_mut() {
      ShowWindow(window, SW_HIDE);
    }
  }
}


// window Proc
pub unsafe extern "system" fn window_proc(hwnd: HWND,
                                          msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  match msg {
    WM_CREATE => {
      println!("Created window")
    }
//        WM_PAINT => {
//            hdc = BeginPaint(hwnd, lp_paint_struct);
//            GetClientRect(hwnd, lp_rect);
//            DrawTextW(hdc, win32_string("Done with pride and prejudice by Culeva Alex"), -1, lp_rect, DT_SINGLELINE | DT_CENTER | DT_VCENTER);
//            EndPaint(hwnd, lp_paint_struct);
//        }
    WM_DESTROY => {
      println!("QUIT!");
      PostQuitMessage(0);
    }
    _ => {
      return DefWindowProcW(hwnd, msg, w_param, l_param);
    }
  }

  return 0;
}

// Create window function
#[cfg(windows)]
fn create_window(name: &str, title: &str) -> Result<Window, Error> {
  let name = win32_string(name);
  let title = win32_string(title);

  unsafe {

    // Create handle instance that will call GetModuleHandleW, which grabs the instance handle of WNDCLASSW (check third parameter)
    let h_instance = GetModuleHandleW(null_mut());

    // Create "class" for window, using WNDCLASSW struct (different from Window our struct)
    let wnd_class = WNDCLASSW {
      style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,        // Style
      lpfnWndProc: Some(window_proc),            // The callbackfunction for any window event that can occur in our window!!! Here you could react to events like WM_SIZE or WM_QUIT.
      hInstance: h_instance,                            // The instance handle for our application which we can retrieve by calling GetModuleHandleW.
      lpszClassName: name,                    // Our class name which needs to be a UTF-16 string (defined earlier before unsafe). as_ptr() (Rust's own function) returns a raw pointer to the slice's buffer
      cbClsExtra: 0,
      cbWndExtra: 0,
      hIcon: null_mut(),
      hCursor: null_mut(),
      hbrBackground: null_mut(),
      lpszMenuName: null_mut(),
    };

    // We have to register this class for Windows to use
    RegisterClassW(&wnd_class);

    // More info: https://msdn.microsoft.com/en-us/library/windows/desktop/ms632680(v=vs.85).aspx
    // Create a window based on registered class
    let handle = CreateWindowExW(
      0,                                    // dwExStyle
      name,                        // lpClassName, name of the class that we want to use for this window, which will be the same that we have registered before.
      title,                        // lpWindowName
      WS_OVERLAPPEDWINDOW | WS_VISIBLE,    // dwStyle
      CW_USEDEFAULT,                        // Int x
      CW_USEDEFAULT,                        // Int y
      960,                        // Int nWidth
      540,                        // Int nHeight
      null_mut(),                            // hWndParent
      null_mut(),                            // hMenu
      h_instance,                            // hInstance
      null_mut());                        // lpParam

    if handle.is_null() {
      Err(Error::last_os_error())
    } else {
      let dc = GetDC(handle);
      let lp_rect: LPRECT = libc::malloc(mem::size_of::<RECT>() as libc::size_t) as *mut RECT;
      GetClientRect(handle, lp_rect);
      Ok(Window { handle, dc })
    }
  }
}

fn is_quit_message(msg: MSG) -> bool {
  if msg.message == 161 {
    return true;
  }
  false
}

#[cfg(windows)]
// Create message handling function with which to link to hook window to Windows messaging system
// More info: https://msdn.microsoft.com/en-us/library/windows/desktop/ms644927(v=vs.85).aspx
// returns true if quit is called
fn handle_messages(window: &mut Window) -> bool {
  unsafe {
    let mut message: MSG = mem::uninitialized();

    while PeekMessageW(&mut message as *mut MSG, window.handle, 0, 0, PM_REMOVE) > 0 {
      if is_quit_message(message) { // QUIT
        return true;
      }
      TranslateMessage(&message as *const MSG); // Translate message into something meaningful with TranslateMessage
      DispatchMessageW(&message as *const MSG); // Dispatch message with DispatchMessageW
    }
    return false;
  }
}

fn get_input(game_state: &mut GameState) {
  unsafe {
    game_state.input.quit_key = GetAsyncKeyState(VK_ESCAPE) != 0;
    game_state.input.left_key = GetAsyncKeyState(0x41) != 0;
    game_state.input.right_key = GetAsyncKeyState(0x44) != 0;
    game_state.input.down_key = GetAsyncKeyState(0x53) != 0;
    game_state.input.up_key = GetAsyncKeyState(0x57) != 0;
  }
}


fn main_loop(window: &mut Window, game_state: &mut GameState, renderer: &Renderer) -> bool {
  if handle_messages(window) {
    return true;
  }

  game_state.frame += 1;
  game_state.time.last_frame_time = game_state.time.frame_start_time.elapsed();
  game_state.time.frame_start_time = Instant::now();

  get_input(game_state);


  let continue_running = game_loop(game_state);
  if !continue_running {
    return true;
  }

  renderer.render_frame(game_state);
  let frame_time = game_state.time.frame_start_time.elapsed();
  if frame_time < Duration::from_millis(15) {
    let sleep_time = Duration::from_millis((15 - frame_time.subsec_millis()).into());
    //println!("Sleeping for {:?}", sleep_time);
    std::thread::sleep(sleep_time);
  }

  return false;
}


#[cfg(windows)]
fn main() {
  hide_console_window();

  let mut window = create_window("my_window", "Portfolio manager pro").unwrap();
  let mut game_state = GameState::new();
  let  client_width :i32 ;
  let  client_height : i32 ;

  unsafe {
    let lp_rect: LPRECT = libc::malloc(mem::size_of::<RECT>() as libc::size_t) as *mut RECT;
    GetClientRect(window.handle, lp_rect);
    client_width = (*lp_rect).right;
    client_height = (*lp_rect).bottom;
  }
  let renderer = renderer::create_simple_renderer(window.dc, client_width, client_height, 960, 540);
  loop {
    if main_loop(&mut window, &mut game_state, &renderer) {
      break;
    }
  }
}