mod simplerenderer;

use game_core::GameState;
use renderer::simplerenderer::SimpleRenderer;

use winapi::shared::windef::HWND;

pub trait Renderer {
    fn render_frame(&mut self, game_state: &mut GameState);
}

pub fn create_simple_renderer(
    handle: HWND,
    back_buffer_width: i32,
    back_buffer_height: i32,
) -> SimpleRenderer {
    simplerenderer::create_simple_renderer(handle, back_buffer_width, back_buffer_height)
}
