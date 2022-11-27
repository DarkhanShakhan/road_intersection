use sdl2::{render::WindowCanvas};
use rand::{Rand, Rng};
use sdl2::pixels::Color;
use sdl2::rect:: Rect;
#[derive(Clone)]
pub struct Vehicle {
    pub position:(i32, i32),
    pub turn: Turning,
    pub direction: Direction
}
#[derive(Clone,Copy)]
pub enum Direction {
    North, 
    South,
    West,
    East
}
#[derive(Clone,Copy)]
pub enum Turning {
    Left,
    Right,
    Straight
}

impl Rand for Turning {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 2) {
            0 => Turning::Left,
            1 => Turning::Right,
            _ => Turning::Straight,
        }
    }
}

impl Vehicle {
    pub fn new(position:(i32, i32), turn:Turning, direction:Direction) ->Self {
        Vehicle { position: position, turn:turn, direction:direction }
    }
}
#[derive(Clone, Copy)]
pub enum Light {
    Red,
    Green
}

pub struct Traffic {
    pub vehicles: Vec<Vec<Vehicle>>,
    pub lights: Vec<Light>,
    
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


impl Traffic {
    pub fn new() -> Self{
        Traffic{vehicles: vec![vec![];4], lights:vec![Light::Red;4]}
    }
    
    pub fn update_vehicles(&mut self, canvas: &mut WindowCanvas){
        for route in &mut self.vehicles {
            for ix in 0..route.len(){
                if stop_vehicle(&self.lights,canvas,&route[ix]) {
                    update_vehicle(canvas, &mut route[ix], 0);
                } else if ix != 0 {
                    if is_safe_distance(&route[ix], &route[ix-1]){
                        update_vehicle(canvas, &mut route[ix], 10);
                    } else {
                        update_vehicle(canvas, &mut route[ix], 0);
                    }
                } else {
                    update_vehicle(canvas, &mut route[ix], 10);
                }
            }
        }
    }
   
    pub fn add_vehicle(&mut self, vehicle: Vehicle){
        match vehicle.direction {
            Direction::North => {
                self.vehicles[0].push(vehicle);
            },
            Direction::South => {
                self.vehicles[1].push(vehicle);
            },
            Direction::West => {
                self.vehicles[2].push(vehicle);
            },
            Direction::East => {
                self.vehicles[3].push(vehicle);
            },
        }
    }
    pub fn update_ligths(&mut self, canvas: &mut WindowCanvas){
        let (w, h) = canvas.output_size().unwrap();
        //north - south lights
        let north_light = Rect::new(w as i32/2-40, h as i32/2-40, 20,20);
        draw_lights(canvas, north_light, self.lights[0]);
        let south_light = Rect::new(w as i32/2+20, h as i32/2+20,20,20);
        draw_lights(canvas, south_light, self.lights[1]);
        //west - east lights
        let west_light = Rect::new(w as i32/2+20, h as i32/2-40, 20,20);
        draw_lights(canvas, west_light, self.lights[0]);    
        let east_light = Rect::new(w as i32/2-40, h as i32/2+20,20,20);
        draw_lights(canvas, east_light, self.lights[1]);

    }

}

fn stop_vehicle(lights:&Vec<Light>,canvas:&WindowCanvas, vehicle:&Vehicle)-> bool{
    let (w, h) = canvas.output_size().unwrap();
    match vehicle.direction {
        Direction::North => {
            match lights[0] {
                Light::Green =>{
                    return false
                },
                Light::Red => {
                    if vehicle.position.1 == h as i32/2 + 20 {
                        return true
                    } else {
                        return false
                    }
                }
            }
        },
        Direction::South => {
            match lights[1] {
                Light::Green =>{
                    return false
                },
                Light::Red => {
                    if vehicle.position.1 == h as i32/2 - 40 {
                        return true
                    } else {
                        return false
                    }
                }
            }
        },
        Direction::West => {
            match lights[2] {
                Light::Green =>{
                    return false
                },
                Light::Red => {
                    if vehicle.position.0 == w as i32/2 + 20 {
                        return true
                    } else {
                        return false
                    }
                }
            }
        },
        Direction::East => {
            match lights[3] {
                Light::Green =>{
                    return false
                },
                Light::Red => {
                    if vehicle.position.0 == w as i32/2 - 40{
                        return true
                    } else {
                        return false
                    }
                }
            }
        },



        _ => false,
    }
}

fn draw_lights(canvas: &mut WindowCanvas,rect:Rect, light:Light){
    match light {
        Light::Green => {
            canvas.set_draw_color(Color::GREEN);
            canvas.fill_rect(rect).unwrap();
        },
        Light::Red => {
            canvas.set_draw_color(Color::RED);
            canvas.fill_rect(rect).unwrap();
        }    
    }
}

fn update_vehicle(canvas: &mut WindowCanvas,vehicle: &mut Vehicle, speed:i32){
    let rect = Rect::new(vehicle.position.0, vehicle.position.1, 20,20);
    let (width,height) = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::CYAN);
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
