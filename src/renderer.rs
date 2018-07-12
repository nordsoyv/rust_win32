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


pub struct Renderer {
  back_buffer: OffscreenBuffer,
  window_width: i32,
  window_height: i32,
  hdc: HDC,
}


impl Renderer {
  pub fn new(hdc: HDC, window_width: i32, window_height: i32, back_buffer_width: i32, back_buffer_height: i32) -> Renderer {

    let bytes_per_pixel = 4;
    let bitmap_memory_size = back_buffer_width * back_buffer_height * bytes_per_pixel;
    unsafe {
      let buffer = OffscreenBuffer {
        width: back_buffer_width,
        height: back_buffer_height,
        pitch: back_buffer_width * bytes_per_pixel,

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

      return Renderer {
        back_buffer: buffer,
        window_width,
        window_height,
        hdc,
      };
    }
  }

  pub fn render_frame(&self, game_state: &mut GameState ) {
    self.render_gradient(game_state.player.pos_x as i32, game_state.player.pos_y as i32);
    unsafe {
      StretchDIBits(self.hdc,
                    0, 0, self.window_width, self.window_height,
                    0, 0, self.back_buffer.width, self.back_buffer.height,
                    self.back_buffer.memory, &self.back_buffer.info, DIB_RGB_COLORS, SRCCOPY);
    }
  }

  fn render_gradient(&self, x_offset: i32, y_offset: i32) {
//    let row = &back_buffer.memory as u8;
    unsafe {
      let start_of_memory = self.back_buffer.memory as *mut u32;
      let mut offset = 0;
      for y in 0..self.back_buffer.height {
//    let mut pixel: u32 = row as u32;
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

