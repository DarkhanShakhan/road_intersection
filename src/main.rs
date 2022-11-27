mod traffic;
use traffic::*;
use sdl2::{render::WindowCanvas};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 700, 700)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    let (width,height) = canvas.output_size().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let from_north = (width as i32/2 - 20, -20);
    let from_east = (width as i32 , height as i32/2-20);
    let from_west = (-20, height as i32/2);
    let from_south = (width as i32/2, height as i32 );
    let mut traffic = Traffic::new(); 
    let mut rng = rand::thread_rng();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown{keycode:Some(Keycode::Down), ..} => {
                    let vehicle = Vehicle::new(from_south, rng.gen(), Direction::North);
                    traffic.add_vehicle(vehicle);
                }
                Event::KeyDown{keycode:Some(Keycode::Up), ..} => {
                    let vehicle = Vehicle::new(from_north, rng.gen(), Direction::South);
                    traffic.add_vehicle(vehicle);
                }
                Event::KeyDown{keycode:Some(Keycode::Left), ..} => {
                    let vehicle = Vehicle::new(from_west, rng.gen(), Direction::East);  
                    traffic.add_vehicle(vehicle);
                }
                Event::KeyDown{keycode:Some(Keycode::Right), ..} => {
                    let vehicle = Vehicle::new(from_east, rng.gen(), Direction::West);
                    traffic.add_vehicle(vehicle);
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::GREY);
        canvas.clear();
        update_layout(&mut canvas);
        traffic.update_vehicles(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 ));
    }
}

fn update_layout(canvas: &mut WindowCanvas){
    let (width, height) = canvas.output_size().unwrap();
    let (v_lane1, v_lane2) = (width as i32/2-20, width as i32/2 +20);
    let (h_lane1, h_lane2) = (height as i32/2-20 , height as i32/2+20);
    canvas.set_draw_color(Color::WHITE);
    canvas.draw_line(Point::new(v_lane1,height as i32), Point::new(v_lane1,h_lane2)).unwrap();
    canvas.draw_line(Point::new(v_lane1,0 as i32), Point::new(v_lane1,h_lane1   )).unwrap();
    canvas.draw_line(Point::new(0, h_lane2), Point::new(v_lane1, h_lane2)).unwrap();
    canvas.draw_line(Point::new(0, h_lane1), Point::new(v_lane1, h_lane1)).unwrap();
    canvas.draw_line(Point::new(v_lane2,h_lane1), Point::new(v_lane2,0)).unwrap();
    canvas.draw_line(Point::new(v_lane2,height as i32), Point::new(v_lane2,h_lane2   )).unwrap();
    canvas.draw_line(Point::new(width as i32, h_lane2), Point::new(v_lane2, h_lane2)).unwrap();
    canvas.draw_line(Point::new(width as i32, h_lane1), Point::new(v_lane2, h_lane1)).unwrap();
    canvas.present();
}