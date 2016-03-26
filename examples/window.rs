extern crate minifb;
extern crate viewdock;

use minifb::{Key, WindowOptions};
use viewdock::{Workspace, Split, Rect, ViewHandle, Direction};

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

fn fill_rect(dest: &mut [u32], rect: Rect, color: u32) {
    let x0 = rect.x as usize;
    let y0 = rect.y as usize;
    let x1 = x0 + rect.width as usize;
    let y1 = y0 + rect.height as usize;

    for y in y0..y1 {
        for x in x0..x1 {
            dest[(y * WIDTH) + x] = color;
        }
    }
}

fn draw_recrusive(dest: &mut [u32], split: &Split) {
    for view in &split.left_views.views {
        fill_rect(dest, view.rect, view.handle.0 as u32);
    }

    for view in &split.right_views.views {
        fill_rect(dest, view.rect, view.handle.0 as u32);
    }

    if let Some(ref split) = split.left {
        draw_recrusive(dest, split); 
    }

    if let Some(ref split) = split.right {
        draw_recrusive(dest, split); 
    }
}

fn draw_ws(dest: &mut [u32], ws: &Workspace) {
    if let Some(ref split) = ws.split {
        draw_recrusive(dest, split);
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut ws = Workspace::new(Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32)).unwrap();

    ws.split_top(ViewHandle(0xff), Direction::Vertical);
    ws.split_top(ViewHandle(0xff00ff), Direction::Vertical);
    ws.split_by_view_handle(Direction::Vertical, ViewHandle(0xff00ff), ViewHandle(0x00ff00));
    ws.split_by_view_handle(Direction::Horizontal, ViewHandle(0x00ff00), ViewHandle(0x5522));
    ws.update();

    let mut window = minifb::Window::new("ViewDockTest",
                                WIDTH,
                                HEIGHT,
                                WindowOptions::default())
                        .expect("Unable to Open Window");

    while window.is_open() && !window.is_key_down(Key::Escape) {

        fill_rect(&mut buffer, Rect::new(0.0, 0.0, WIDTH as f32, HEIGHT as f32), 0);
        draw_ws(&mut buffer, &ws);

        window.update_with_buffer(&buffer);
    }
}
