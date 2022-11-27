use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use rand::{Rng, Rand};
use sdl2::rect::Rect;
#[derive(Clone)]
pub struct Vehicle {
    pub position:(i32, i32),
    pub turn: Turning,
    pub direction: Direction,
    pub color :Color
}
impl Vehicle {
    pub fn new(position:(i32, i32), turn:Turning, direction:Direction) ->Self {
        let mut color =Color::YELLOW;
        match turn {
            Turning::Left => color = Color::CYAN,
            Turning::Right => color =Color::BLUE,
            Turning::Straight => ()
        }
        Vehicle { position: position, turn:turn, direction:direction, color:color }
    }
}
#[derive(Clone,Copy)]
pub enum Turning {
    Left,
    Right,
    Straight
}

impl Rand for Turning {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => Turning::Left,
            1 => Turning::Right,
            _ => Turning::Straight,
        }
    }
}

#[derive(Clone,Copy)]
pub enum Direction {
    North, 
    South,
    West,
    East
}

impl Rand for Direction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 5) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East
        }
    }
}


pub fn is_safe_distance(curr:&Vehicle, previous:&Vehicle) ->bool {
    match curr.direction {
        Direction::North => {
            if curr.position.1 - (previous.position.1 + 30) > 10 || curr.position.0 != previous.position.0{
                true
            } else {
                false
            }
        }
        Direction::South => {
            if previous.position.1 - (curr.position.1 + 30) > 10 || curr.position.0 != previous.position.0{
                true
            } else {
                false
            }
        }
        Direction::West => {
            if curr.position.0 - (previous.position.0 + 30) > 10 || curr.position.1 != previous.position.1{
                true
            } else {
                false
            }
        }
        Direction::East => {
            if previous.position.0  - (curr.position.0+30) > 10 || curr.position.1 != previous.position.1{
                true
            } else {
                false
            }
        }
    }
}

pub fn update_vehicle(canvas: &mut WindowCanvas,vehicle: &mut Vehicle, speed:i32){
    let rect = Rect::new(vehicle.position.0, vehicle.position.1, 20,20);
    let (width,height) = canvas.output_size().unwrap();
    canvas.set_draw_color(vehicle.color);
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
                    if vehicle.position.0 == width as i32/2 {
                        vehicle.direction = Direction::North;
                        vehicle.turn = Turning::Straight;
                    }
                },
                Turning::Straight => ()
            }
        }
    }
}


pub fn in_intersection(vehicle:&Vehicle, canvas : &WindowCanvas) -> bool {
    let (w, h) = canvas.output_size().unwrap();
    match vehicle.direction {
        Direction::North => {
            if vehicle.position.1 < h as i32 / 2 + 20 {
                true
            } else {
                false
            }
        },
        Direction::South => {
            if vehicle.position.1 > h as i32 / 2 - 40 {
                true
            } else {
                false
            } 
        },
        Direction::West => {
            if vehicle.position.0 < w as i32 / 2 + 20 {
                true
            } else {
                false
            } 
        },
        Direction::East => {
            if vehicle.position.0 > w as i32 / 2 - 40 {
                true
            } else {
                false
            } 
        }
    }
}

pub fn passed_intersection(vehicle:&Vehicle, canvas: &WindowCanvas) -> bool {
    let (w, h) = canvas.output_size().unwrap();
    match vehicle.direction {
        Direction::North => {
            if vehicle.position.1 <= h as i32 / 2 - 40 {
                true
            } else {
                false
            }
        },
        Direction::South => {
            if vehicle.position.1 >= h as i32 / 2 + 20 {
                true
            } else {
                false
            } 
        },
        Direction::West => {
            if vehicle.position.0 <= w as i32 / 2 - 40 {
                true
            } else {
                false
            } 
        },
        Direction::East => {
            if vehicle.position.0 >= w as i32 / 2 + 20 {
                true
            } else {
                false
            } 
        }
    }
}

pub fn passed_scope(vehicle:&Vehicle, canvas: &WindowCanvas) -> bool{
    let (w, h) = canvas.output_size().unwrap();
    vehicle.position.0 <= -20 || vehicle.position.0 >= w  as i32|| vehicle.position.1 <= -20 || vehicle.position.1 >= h as i32
}