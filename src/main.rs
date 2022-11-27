use sdl2::{render::WindowCanvas};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

#[derive(Clone)]
struct Vehicle {
    position:(i32, i32),
    turn: Turning,
    direction: Direction
}
#[derive(Clone,Copy)]
enum Direction {
    North, 
    South,
    West,
    East
}
#[derive(Clone,Copy)]
enum Turning {
    Left,
    Right,
    Straight
}
impl Vehicle {
    fn new(position:(i32, i32), turn:Turning, direction:Direction) ->Self {
        Vehicle { position: position, turn:turn, direction:direction }
    }
}
#[derive(Clone, Copy)]
enum Light {
    Red,
    Green
}

struct Traffic {
    vehicles: Vec<Vec<Vehicle>>,
    lights: Vec<Light>,
    
    // canvas
}

fn is_safe_distance(curr:&Vehicle, previous:&Vehicle) ->bool {
    match curr.direction {
        Direction::North => {
            if curr.position.1 - (previous.position.1 + 30) > 10 || curr.position.0 != previous.position.0{
                true
            } else {
                false
            }
        }
        _ => true
    }
}


impl Traffic {
    fn new() -> Self{
        Traffic{vehicles: vec![vec![];4], lights:vec![Light::Green;4]}
    }
    
    fn update_vehicles(&mut self, canvas: &mut WindowCanvas){
        for route in &mut self.vehicles {
            for ix in 0..route.len(){
                if ix != 0 {
                    if is_safe_distance(&route[ix], &route[ix-1]){
                        update_vehicle(canvas, &mut route[ix], 10);
                    } else {
                        update_vehicle(canvas, &mut route[ix], 0);
                    }
                } else {
                    update_vehicle(canvas, &mut route[ix], 10);
                }
                // update_vehicle(canvas, vehicle);
            }
        }
    }

    fn add_vehicle(&mut self, vehicle: Vehicle){
        match vehicle.direction {
            Direction::North => {
                self.vehicles[0].push(vehicle);
            },
            _ => ()
        }
    }

}


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
    let from_north = (width as i32/2 - 20, 0);
    let from_east = (width as i32 -20, height as i32/2-20);
    let from_west = (0, height as i32/2);
    let from_south = (width as i32/2, height as i32 - 20);
    let mut vehicle = Vehicle::new(from_south, Turning::Right, Direction::North);
    let mut vehicle2 = Vehicle::new(from_south, Turning::Right, Direction::North);
    let mut vehicle3 = Vehicle::new(from_south, Turning::Right, Direction::North);
    let mut vehicle4 = Vehicle::new(from_south, Turning::Right, Direction::North);
    let mut traffic = Traffic::new();
    traffic.add_vehicle(vehicle);
    traffic.add_vehicle(vehicle2);
    traffic.add_vehicle(vehicle3);
    traffic.add_vehicle(vehicle4);
    // let mut i = 0;
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::GREY);
        canvas.clear();
        update_layout(&mut canvas);
        traffic.update_vehicles(&mut canvas);
        canvas.present();
        // canvas.fill_rect(Rect::new(350, 0, 20, 20)).unwrap();
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 ));
    }
}

fn update_vehicle(canvas: &mut WindowCanvas,vehicle: &mut Vehicle, speed:i32){
    let rect = Rect::new(vehicle.position.0, vehicle.position.1, 20,20);
    let (width,height) = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::GREEN);
    canvas.fill_rect(rect).unwrap();
    // canvas.present();
    match vehicle.direction {
        Direction::South => {
            vehicle.position.1 += speed;
            match vehicle.turn {
                Turning::Right => 
                    {
                        if vehicle.position.1 == height as i32/2 - 20{
                            vehicle.direction = Direction::West;
                            vehicle.turn = Turning::Straight;
                        }

                    },
                Turning::Left => {
                    if vehicle.position.1 == height as i32/2 {
                        vehicle.direction = Direction::East;
                        vehicle.turn = Turning::Straight;
                    }
                },
                Turning::Straight => ()
            }
        },      
        Direction::West => {
            vehicle.position.0 -= speed;
            match vehicle.turn {
                Turning::Right => 
                    {
                        if vehicle.position.0 == width as i32/2{
                            vehicle.direction = Direction::North;
                            vehicle.turn = Turning::Straight;
                        }

                    },
                Turning::Left => {
                    if vehicle.position.0 == width as i32/2-20 {
                        vehicle.direction = Direction::South;
                        vehicle.turn = Turning::Straight;
                    }
                },
                Turning::Straight => ()
            }
        }
        Direction::North => {
            vehicle.position.1 -=speed;
            match vehicle.turn {
                Turning::Right =>
                {
                    if vehicle.position.1 == height as i32/2 {
                        vehicle.direction = Direction::East;
                        vehicle.turn = Turning::Straight;
                    }
                },
                Turning::Left => {
                    if vehicle.position.1 == height as i32/2-20 {
                        vehicle.direction = Direction::West;
                        vehicle.turn = Turning::Straight;
                    }
                },
                Turning::Straight => ()
            }
        },
        Direction::East => {
            vehicle.position.0 +=speed;
            match vehicle.turn {
                Turning::Right =>
                {
                    if vehicle.position.0 == width as i32/2-20 {
                        vehicle.direction = Direction::South;
                        vehicle.turn = Turning::Straight;
                    }
                },
                Turning::Left => {
                    if vehicle.position.1 == width as i32/2 {
                        vehicle.direction = Direction::West;
                        vehicle.turn = Turning::Straight;
                    }
                },
                Turning::Straight => ()
            }
        }
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