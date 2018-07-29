// Let's put this so that it won't open console
//#![windows_subsystem = "windows"]

extern crate game_core;
extern crate libc;
#[cfg(windows)]
extern crate winapi;

// https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/um/libloaderapi/index.html?search=winuser

use game_core::game_loop;
use game_core::GameState;
use renderer::Renderer;
use self::winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use self::winapi::shared::windef::HWND;
use self::winapi::um::libloaderapi::GetModuleHandleW;
use self::winapi::um::wincon::GetConsoleWindow;
use self::winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetAsyncKeyState, PeekMessageW,
    PostQuitMessage, RegisterClassW, ShowWindow, TranslateMessage,
};
use self::winapi::um::winuser::{
    CS_HREDRAW,
    CS_OWNDC,
    CS_VREDRAW,
    CW_USEDEFAULT,
    MSG,
    PM_REMOVE,
    SW_HIDE,
    VK_ESCAPE,
    VK_SPACE,
    //    WM_PAINT,
    WM_CREATE,
    WM_DESTROY,
    WNDCLASSW,
    WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
};
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::time::Duration;
use std::time::Instant;

mod renderer;

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
pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => println!("Created window"),
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
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW, // Style
            lpfnWndProc: Some(window_proc), // The callbackfunction for any window event that can occur in our window!!! Here you could react to events like WM_SIZE or WM_QUIT.
            hInstance: h_instance, // The instance handle for our application which we can retrieve by calling GetModuleHandleW.
            lpszClassName: name, // Our class name which needs to be a UTF-16 string (defined earlier before unsafe). as_ptr() (Rust's own function) returns a raw pointer to the slice's buffer
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
            0,                                // dwExStyle
            name, // lpClassName, name of the class that we want to use for this window, which will be the same that we have registered before.
            title, // lpWindowName
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, // dwStyle
            CW_USEDEFAULT, // Int x
            CW_USEDEFAULT, // Int y
            960,  // Int nWidth
            540,  // Int nHeight
            null_mut(), // hWndParent
            null_mut(), // hMenu
            h_instance, // hInstance
            null_mut(),
        ); // lpParam

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(Window { handle })
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
            if is_quit_message(message) {
                // QUIT
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
        game_state.input.space = GetAsyncKeyState(VK_SPACE) != 0;
        game_state.input.shoot_down = GetAsyncKeyState(0x4B) != 0;
        game_state.input.shoot_up = GetAsyncKeyState(0x49) != 0;
        game_state.input.shoot_left = GetAsyncKeyState(0x4A) != 0;
        game_state.input.shoot_right = GetAsyncKeyState(0x4C) != 0;
    }
}

static mut START_TIME: Option<Instant> = None;
static mut LAST_FRAME_START: Option<Instant> = None;


#[cfg(windows)]
fn main() {
    hide_console_window();

    let mut window = create_window("my_window", "Portfolio manager pro").unwrap();
    let mut game_state = GameState::new(960.0, 540.0);
    unsafe {
        START_TIME = Some(Instant::now());
        LAST_FRAME_START = Some(Instant::now());
    }
    let mut renderer = renderer::create_simple_renderer(window.handle, 960, 540);
    loop {
        if main_loop(&mut window, &mut game_state, &mut renderer) {
            break;
        }
    }
}


fn main_loop(window: &mut Window, game_state: &mut GameState, renderer: &mut Renderer) -> bool {
    if handle_messages(window) {
        return true;
    }
    unsafe {
        let last_frame_time = LAST_FRAME_START.unwrap().elapsed();
        LAST_FRAME_START = Some(Instant::now());

        let delta = last_frame_time.subsec_micros() as f32;
        game_state.time.delta = delta / (1000.0 * 1000.0);

        let total_time = START_TIME.unwrap().elapsed();
        game_state.time.time_elapsed = total_time.as_secs() as f32;
        game_state.time.time_elapsed += total_time.subsec_micros() as f32 / (1000.0 * 1000.0);

        get_input(game_state);

        let game_output = game_loop(game_state);


        if game_output.len() == 0 {
            return true;
        }

        renderer.render_frame(game_output);
        let frame_time = LAST_FRAME_START.unwrap().elapsed();
//        println!("Frame time {:?}", frame_time.subsec_millis());

        if frame_time < Duration::from_millis(15) {
            let sleep_time = Duration::from_millis((15 - frame_time.subsec_millis()).into());
            std::thread::sleep(sleep_time);
        } else {
            println!(
                "Missed frame timing. Last frame took {:?} milliseconds",
                frame_time.subsec_millis()
            )
        }

        return false;
    }
}
