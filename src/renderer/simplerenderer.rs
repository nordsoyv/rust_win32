use entities::Color;
use entities::Drawable;
use game::GameState;
use libc;
use renderer::Renderer;
use std;
use std::mem;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::windef::{HDC, HWND, LPRECT, RECT};
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
use winapi::um::wingdi::{
    StretchDIBits, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
};
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE};
use winapi::um::winuser::{GetClientRect, GetDC};

struct OffscreenBuffer {
    info: BITMAPINFO,
    memory: LPVOID,
    width: i32,
    height: i32,
    pitch: i32,
}

pub struct SimpleRenderer {
    back_buffer: OffscreenBuffer,
    window_width: i32,
    window_height: i32,
    hdc: HDC,
}

pub fn create_simple_renderer(
    handle: HWND,
    back_buffer_width: i32,
    back_buffer_height: i32,
) -> SimpleRenderer {
    {
        unsafe {
            let dc = GetDC(handle);
            let lp_rect: LPRECT = libc::malloc(mem::size_of::<RECT>() as libc::size_t) as *mut RECT;
            GetClientRect(handle, lp_rect);
            let client_width = (*lp_rect).right;
            let client_height = (*lp_rect).bottom;

            let bytes_per_pixel = 4;
            let bitmap_memory_size = back_buffer_width * back_buffer_height * bytes_per_pixel;
            let buffer = OffscreenBuffer {
                width: back_buffer_width,
                height: back_buffer_height,
                pitch: back_buffer_width,

                info: BITMAPINFO {
                    bmiHeader: BITMAPINFOHEADER {
                        biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                        biWidth: back_buffer_width,
                        biHeight: back_buffer_height,
                        biPlanes: 1,
                        biBitCount: 32,
                        biCompression: 0,
                        biSizeImage: 0,
                        biXPelsPerMeter: 0,
                        biYPelsPerMeter: 0,
                        biClrUsed: 0,
                        biClrImportant: 0,
                    },
                    bmiColors: [RGBQUAD {
                        rgbBlue: 0,
                        rgbGreen: 0,
                        rgbRed: 0,
                        rgbReserved: 0,
                    }; 1],
                },
                memory: VirtualAlloc(
                    std::ptr::null_mut(),
                    bitmap_memory_size as usize,
                    MEM_COMMIT,
                    PAGE_READWRITE,
                ),
            };

            return SimpleRenderer {
                back_buffer: buffer,
                window_width: client_width,
                window_height: client_height,
                hdc: dc,
            };
        }
    }
}

impl SimpleRenderer {
    fn draw_obj(&self, obj: &Drawable) {
        let rect = obj.get_bounding_box();
        self.draw_rectangle(
            rect.left,
            rect.bottom,
            rect.right,
            rect.top,
            obj.get_color(),
        )
    }

    fn draw_rectangle(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32, color: &Color) {
        let mut start_x = min_x as i32;
        if start_x < 0 {
            start_x = 0;
        }
        let mut start_y = min_y as i32;
        if start_y < 0 {
            start_y = 0;
        }
        let mut end_x = max_x as i32;
        if end_x > self.back_buffer.width {
            end_x = self.back_buffer.width;
        }
        let mut end_y = max_y as i32;
        if end_y > self.back_buffer.height {
            end_y = self.back_buffer.height;
        }

        let mut c: u32 = 0;
        let alpha: u32 = ((color.a * 255.0) as u8) as u32;
        let red: u32 = ((color.r * 255.0) as u8) as u32;
        let green: u32 = ((color.g * 255.0) as u8) as u32;
        let blue: u32 = ((color.b * 255.0) as u8) as u32;

        c = c | alpha << 24;
        c = c | red << 16;
        c = c | green << 8;
        c = c | blue << 0;

        unsafe {
            let start_of_memory = self.back_buffer.memory as *mut u32;
            for y in start_y..end_y {
                let mut offset: isize = self.back_buffer.pitch as isize * y as isize;
                offset += start_x as isize;
                for _x in start_x..end_x {
                    let mut pixel = start_of_memory.offset(offset);
                    *pixel = c;
                    offset += 1;
                }
            }
        }
    }

    fn _render_gradient(&self, x_offset: i32, y_offset: i32) {
        unsafe {
            let start_of_memory = self.back_buffer.memory as *mut u32;
            let mut offset = 0;
            for y in 0..self.back_buffer.height {
                for x in 0..self.back_buffer.width {
                    offset += 1;
                    let blue: u32 = ((x + x_offset) as u8).into();
                    let green: u32 = ((y + y_offset) as u8).into();
                    let mut pixel = start_of_memory.offset(offset);
                    *pixel = green << 8 | blue;
                }
            }
        }
    }

    fn clear_screen(&mut self) {
        unsafe {
            VirtualFree(self.back_buffer.memory, 0, MEM_RELEASE);
            let bitmap_memory_size = self.back_buffer.width * self.back_buffer.height * 4;
            self.back_buffer.memory = VirtualAlloc(
                std::ptr::null_mut(),
                bitmap_memory_size as usize,
                MEM_COMMIT,
                PAGE_READWRITE,
            )
        }
    }
}

impl Renderer for SimpleRenderer {
    fn render_frame(&mut self, game_state: &mut GameState) {
        self.clear_screen();
        self.draw_obj(&game_state.player);
        for e in &game_state.walls {
            self.draw_obj(e);
        }
        for e in &game_state.bullets {
            self.draw_obj(e);
        }
        for e in &game_state.enemies {
            self.draw_obj(e);
        }

        unsafe {
            StretchDIBits(
                self.hdc,
                0,
                0,
                self.window_width,
                self.window_height,
                0,
                0,
                self.back_buffer.width,
                self.back_buffer.height,
                self.back_buffer.memory,
                &self.back_buffer.info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
        }
    }
}
