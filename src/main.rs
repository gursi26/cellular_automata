#![allow(dead_code, unused_variables)]

extern crate raylib;
use raylib::prelude::*;
use rand::Rng;

const WIDTH: i32 = 1440; 
const HEIGHT: i32 = 900;
const TARGETFPS: u32 = 60; 
const ACCELERATION_GRAVITY: f32 = 0.2;
const DISPERSION_RATE: i32 = 10;
const BG_COLOR: Color = Color::BLACK;
const PIXEL_SIZE: Vector2 = Vector2{x: 10.0, y: 10.0};
const SMALLER_PIXEL: Vector2 = Vector2{x: 6.0, y: 6.0};

const GRID_WIDTH: usize = (WIDTH / PIXEL_SIZE.x as i32) as usize; 
const GRID_HEIGHT: usize = (HEIGHT / PIXEL_SIZE.y as i32) as usize; 

#[derive(Clone, Copy, PartialEq, Debug)]
enum ParticleType{
    Air,
    Sand,
    Stone,
    Water,
    Boundary,
}

#[derive(Clone, Copy)]
struct Pixel {
    vel: Vector2,
    particle_type: ParticleType,
}

impl Pixel {
    fn draw_pixel(&self, d: &mut RaylibDrawHandle, row: usize, col: usize) {
        let pos = Vector2{x: col as f32 * PIXEL_SIZE.x, y: row as f32 * PIXEL_SIZE.y};
        match self.particle_type {
            ParticleType::Air => (),
            ParticleType::Sand => d.draw_rectangle_v(pos, PIXEL_SIZE, Color::BEIGE),
            ParticleType::Stone => d.draw_rectangle_v(pos, PIXEL_SIZE, Color::BROWN),
            ParticleType::Water => d.draw_rectangle_v(pos, PIXEL_SIZE, Color::BLUE),
            ParticleType::Boundary => {
                d.draw_rectangle_v(pos, PIXEL_SIZE, Color::WHITE);
                d.draw_rectangle(pos.x as i32 + 2, pos.y as i32 + 2, SMALLER_PIXEL.x as i32, SMALLER_PIXEL.y as i32, Color::BLACK);
            },
        };
    }

    fn move_water(
        particle_grid: &mut [[Pixel; GRID_WIDTH]; GRID_HEIGHT],
        moved: &mut Vec<(usize, usize)>,
        mut x: usize, 
        mut y: usize,
    ) {
        let mut current_particle = particle_grid[y][x];

        if !moved.contains(&(y,x)) {
            let new_y_pos = y + current_particle.vel.y as usize;

            // straight down check
            if ParticleType::Air == particle_grid[new_y_pos][x].particle_type {
                particle_grid[new_y_pos][x] = current_particle;
                particle_grid[y][x] = Pixel::create_air_particle();
                moved.push((new_y_pos, x));
            } else {
            // right down check
                if rand::thread_rng().gen_range(0..100) < 50 {

                    let (old_x, old_y) = (x.clone(), y.clone());

                    for i in 0..current_particle.vel.x as i32 {
                        let right_level_check = ParticleType::Air == particle_grid[y][x + 1].particle_type;
                        let right_down_check = ParticleType::Air == particle_grid[y + 1][x + 1].particle_type;

                        if right_level_check {
                            x += 1;
                            if right_down_check {
                                y += 1;
                            }
                        } else {
                            break;
                        }
                    }

                    particle_grid[old_y][old_x] = Pixel::create_air_particle();
                    if current_particle.vel.x > 3.0 {
                        current_particle.vel.x -= 1.0;
                    }
                    particle_grid[y][x] = current_particle;
                    moved.push((y, x));
                }
                // left down check
                else {
                    let (old_x, old_y) = (x.clone(), y.clone());

                    for i in 0..current_particle.vel.x as i32 {
                        let right_level_check = ParticleType::Air == particle_grid[y][x - 1].particle_type;
                        let right_down_check = ParticleType::Air == particle_grid[y + 1][x - 1].particle_type;

                        if right_level_check {
                            x -= 1;
                            if right_down_check {
                                y += 1;
                            }
                        } else {
                            break;
                        }
                    }

                    particle_grid[old_y][old_x] = Pixel::create_air_particle();
                    if current_particle.vel.x > 3.0 {
                        current_particle.vel.x -= 1.0;
                    }
                    particle_grid[y][x] = current_particle;
                    moved.push((y, x));
                }
            }
        }
    }

    fn move_sand(
        particle_grid: &mut [[Pixel; GRID_WIDTH]; GRID_HEIGHT],
        moved: &mut Vec<(usize, usize)>,
        x: usize, 
        y: usize,
    ) {

        let current_particle = particle_grid[y][x];

        if !moved.contains(&(y,x)) {
            let new_y_pos = y + current_particle.vel.y as usize;

            // straight down check
            if ParticleType::Air == particle_grid[new_y_pos][x].particle_type {
                particle_grid[new_y_pos][x] = current_particle;
                particle_grid[y][x] = Pixel::create_air_particle();
                moved.push((new_y_pos, x));
            } 
            // right down check
            else if rand::thread_rng().gen_range(0..100) < 50 {

                let right_down_check = ParticleType::Air == particle_grid[new_y_pos][x + 1].particle_type;
                let right_level_check = ParticleType::Air == particle_grid[y][x + 1].particle_type;

                if right_down_check && right_level_check {
                    let new_x_pos = x + 1;
                    particle_grid[new_y_pos][new_x_pos] = current_particle;
                    particle_grid[y][x] = Pixel::create_air_particle();
                    moved.push((new_y_pos, new_x_pos));
                }
            } 
            // left down check
            else {

                let left_down_check = ParticleType::Air == particle_grid[new_y_pos][x - 1].particle_type;
                let left_level_check = ParticleType::Air == particle_grid[y][x - 1].particle_type;

                if left_down_check && left_level_check {
                    let new_x_pos = x - 1;
                    particle_grid[new_y_pos][new_x_pos] = current_particle;
                    particle_grid[y][x] = Pixel::create_air_particle();
                    moved.push((new_y_pos, new_x_pos));
                }
            }
        }
    }

    fn move_pixel(
        particle_grid: &mut [[Pixel; GRID_WIDTH]; GRID_HEIGHT],
        moved: &mut Vec<(usize, usize)>,
        x: usize, 
        y: usize,
    ) {
        match particle_grid[y][x].particle_type {
            ParticleType::Sand => Pixel::move_sand(&mut *particle_grid, &mut *moved, x, y),
            ParticleType::Water => Pixel::move_water(&mut *particle_grid, &mut *moved, x, y),
            _ => (),
        }
    }

    fn create_air_particle() -> Pixel {
        Pixel {
            vel: Vector2 {x: 0.0, y: 0.0},
            particle_type: ParticleType::Air,
        }
    }

    fn create_sand_particle() -> Pixel {
        Pixel {
            vel: Vector2 {x: 0.0, y: 1.0},
            particle_type: ParticleType::Sand,
        }
    }

    fn create_water_particle() -> Pixel {
        Pixel {
            vel: Vector2 {x: DISPERSION_RATE as f32, y: 1.0},
            particle_type: ParticleType::Water,
        }
    }

    fn create_stone_particle() -> Pixel {
        Pixel {
            vel: Vector2 {x: 0.0, y: 0.0},
            particle_type: ParticleType::Stone,
        }
    }

    fn create_boundary_particle() -> Pixel {
        Pixel {
            vel: Vector2 {x: 0.0, y: 0.0},
            particle_type: ParticleType::Boundary,
        }
    }
}

fn draw_debug_info(d: &mut RaylibDrawHandle, mouse_x: usize, mouse_y: usize) {
    let mut mouse_pos_str = mouse_x.to_string(); 
    mouse_pos_str.push_str(" ");
    mouse_pos_str.push_str(&mouse_y.to_string()[..]);
    d.draw_text(&mouse_pos_str, 15, 15, 20, Color::WHITE);
}

fn input_handler(
    d: &mut RaylibDrawHandle, 
    particle_grid: &mut [[Pixel; GRID_WIDTH]; GRID_HEIGHT], 
    draw_flag: &mut bool, 
    current_draw_particle: &mut ParticleType
) -> (usize, usize) 
{
    let (mut mouse_x, mut mouse_y) = (d.get_mouse_x() as usize, d.get_mouse_y() as usize);
    mouse_x = ((mouse_x as f32 - PIXEL_SIZE.x/2.0)/PIXEL_SIZE.x).round() as usize;
    mouse_y = ((mouse_y as f32 - PIXEL_SIZE.y/2.0)/PIXEL_SIZE.y).round() as usize;

    if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
        if let ParticleType::Air = particle_grid[mouse_y][mouse_x].particle_type {
            match *current_draw_particle {
                ParticleType::Stone => particle_grid[mouse_y][mouse_x] = Pixel::create_stone_particle(),
                ParticleType::Sand => {
                    if *draw_flag {
                        particle_grid[mouse_y][mouse_x] = Pixel::create_sand_particle();
                    }
                    *draw_flag = !*draw_flag;
                },
                ParticleType::Water => {
                    if *draw_flag {
                        particle_grid[mouse_y][mouse_x] = Pixel::create_water_particle();
                    }
                    *draw_flag = !*draw_flag;
                },
                _ => (),
            }
        } else if let ParticleType::Air = *current_draw_particle  {
            if particle_grid[mouse_y][mouse_x].particle_type != ParticleType::Boundary {
                particle_grid[mouse_y][mouse_x] = Pixel::create_air_particle();
            }
        }
    }

    let mut change_particle = current_draw_particle.clone();
    if d.is_key_pressed(KeyboardKey::KEY_ZERO) {
        change_particle = ParticleType::Air;
    } else if d.is_key_pressed(KeyboardKey::KEY_Z) {
        *particle_grid = [[Pixel::create_air_particle(); GRID_WIDTH]; GRID_HEIGHT];
        draw_boundary(particle_grid);
    } else if d.is_key_pressed(KeyboardKey::KEY_TWO) {
        change_particle = ParticleType::Sand;
    } else if d.is_key_pressed(KeyboardKey::KEY_ONE) {
        change_particle = ParticleType::Stone;
    } else if d.is_key_pressed(KeyboardKey::KEY_THREE) {
        change_particle = ParticleType::Water;
    }

    if change_particle != *current_draw_particle {
        *current_draw_particle = change_particle;
    }

    return (mouse_x, mouse_y);
}

fn draw_boundary(arr : &mut [[Pixel; GRID_WIDTH]; GRID_HEIGHT]) {
    arr[0] = [Pixel::create_boundary_particle(); GRID_WIDTH];
    arr[GRID_HEIGHT - 1] = [Pixel::create_boundary_particle(); GRID_WIDTH];
    for i in 1..GRID_HEIGHT-1 {
        arr[i][0] = Pixel::create_boundary_particle();
        arr[i][GRID_WIDTH - 1] = Pixel::create_boundary_particle();
    }
}

fn main() {
    let (mut rl, thread) = init().size(WIDTH, HEIGHT).fullscreen().title("Pandemic Simulation").build();
    rl.set_target_fps(TARGETFPS);

    let mut particle_grid = [[Pixel::create_air_particle(); GRID_WIDTH]; GRID_HEIGHT];
    draw_boundary(&mut particle_grid);
    let mut current_draw_particle = ParticleType::Stone;
    let mut draw_flag = true;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);

        let mut moved: Vec<(usize, usize)> = Vec::new();

        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                Pixel::move_pixel(&mut particle_grid, &mut moved, col, row);
                particle_grid[row][col].draw_pixel(&mut d, row, col);
            }
        }

        let (mouse_x, mouse_y) = input_handler(&mut d, &mut particle_grid, &mut draw_flag, &mut current_draw_particle);
        draw_debug_info(&mut d, mouse_x, mouse_y);
    }
}
