use std::mem;
use std;

//mod game;

use game::GameState;

use winapi::shared::minwindef::LPVOID;

use winapi::shared::windef::HDC;


use winapi::um::wingdi::{
  BITMAPINFO,
  BITMAPINFOHEADER,
  RGBQUAD,
  StretchDIBits,
  DIB_RGB_COLORS,
  SRCCOPY,
};

use winapi::um::winnt::{
  MEM_COMMIT,
  PAGE_READWRITE,
};

use winapi::um::memoryapi::VirtualAlloc;


struct OffscreenBuffer {
  info: BITMAPINFO,
  memory: LPVOID,
  width: i32,
  height: i32,
  pitch: i32,

}


pub trait Renderer {
  fn render_frame(&self, game_state: &mut GameState);
}

pub struct SimpleRenderer {
  back_buffer: OffscreenBuffer,
  window_width: i32,
  window_height: i32,
  hdc: HDC,
}

pub fn create_simple_renderer(hdc: HDC, window_width: i32, window_height: i32, back_buffer_width: i32, back_buffer_height: i32) -> SimpleRenderer {
  {
    let bytes_per_pixel = 4;
    let bitmap_memory_size = back_buffer_width * back_buffer_height * bytes_per_pixel;
    unsafe {
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
        memory: VirtualAlloc(std::ptr::null_mut(), bitmap_memory_size as usize, MEM_COMMIT, PAGE_READWRITE),
      };

      return SimpleRenderer {
        back_buffer: buffer,
        window_width,
        window_height,
        hdc,
      };
    }
  }
}


impl SimpleRenderer {
  fn draw_rectangle(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
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
    unsafe {
      let start_of_memory = self.back_buffer.memory as *mut u32;
      for y in start_y..end_y {
        let mut offset: isize = self.back_buffer.pitch as isize * y as isize;
        offset += start_x as isize;
        for _x in start_x..end_x {
          let mut pixel = start_of_memory.offset(offset);
          *pixel = 0xffffffff;


          offset += 1;
        }
      }
    }
  }

  fn render_gradient(&self, x_offset: i32, y_offset: i32) {
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
}

impl Renderer for SimpleRenderer {
  fn render_frame(&self, game_state: &mut GameState) {
    self.render_gradient(game_state.player.pos_x as i32, game_state.player.pos_y as i32);
    self.draw_rectangle(game_state.player.pos_x,
                        game_state.player.pos_y,
                        game_state.player.pos_x + 40.0,
                        game_state.player.pos_y + 40.0);
    unsafe {
      StretchDIBits(self.hdc,
                    0, 0, self.window_width, self.window_height,
                    0, 0, self.back_buffer.width, self.back_buffer.height,
                    self.back_buffer.memory, &self.back_buffer.info, DIB_RGB_COLORS, SRCCOPY);
    }
  }
}