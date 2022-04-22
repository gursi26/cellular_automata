#![allow(dead_code, unused_variables)]

extern crate raylib;
use raylib::prelude::*;
// use rand::Rng;

const WIDTH: i32 = 1440; 
const HEIGHT: i32 = 900;
const TARGETFPS: u32 = 60; 
const BG_COLOR: Color = Color::BLACK;
const PIXEL_SIZE: Vector2 = Vector2{x: 10.0, y: 10.0};

const GRID_WIDTH: usize = (WIDTH / PIXEL_SIZE.x as i32) as usize; 
const GRID_HEIGHT: usize = (HEIGHT / PIXEL_SIZE.y as i32) as usize; 

#[derive(Clone, Copy, PartialEq, Debug)]
enum ParticleType{
    Air,
    Sand,
    Stone,
    Water,
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
        };
    }

    fn move_sand(
        particle_grid: &mut [[Pixel; GRID_WIDTH]; GRID_HEIGHT],
        moved: &mut Vec<(usize, usize)>,
        fall_dir: &mut bool,
        x: usize, 
        y: usize,
    ) {
        let bottom_boundary_condition = y < GRID_HEIGHT - 1;
        let top_boundary_condition = y > 1;
        let left_boundary_condition = x > 1;
        let right_boundary_condition = x < GRID_WIDTH - 1;

        let current_particle = particle_grid[y][x];

        if bottom_boundary_condition && !moved.contains(&(y,x)) {
            let new_y_pos = y + current_particle.vel.y as usize;

            // straight down check
            if ParticleType::Air == particle_grid[new_y_pos][x].particle_type {
                particle_grid[new_y_pos][x] = current_particle;
                particle_grid[y][x] = Pixel::create_air_particle();
                moved.push((new_y_pos, x));
            } 
            // right down check
            else if right_boundary_condition && *fall_dir {

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
            else if left_boundary_condition {

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
        *fall_dir = !*fall_dir;
    }

    fn move_pixel(
        particle_grid: &mut [[Pixel; GRID_WIDTH]; GRID_HEIGHT],
        moved: &mut Vec<(usize, usize)>,
        fall_dir: &mut bool,
        x: usize, 
        y: usize,
    ) {
        match particle_grid[y][x].particle_type {
            ParticleType::Sand => Pixel::move_sand(&mut *particle_grid, &mut *moved, &mut *fall_dir, x, y),
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

    fn create_stone_particle() -> Pixel {
        Pixel {
            vel: Vector2 {x: 0.0, y: 0.0},
            particle_type: ParticleType::Stone,
        }
    }
}

fn draw_debug_info(d: &mut RaylibDrawHandle, mouse_x: usize, mouse_y: usize) {
    let mut mouse_pos_str = mouse_x.to_string(); 
    mouse_pos_str.push_str(" ");
    mouse_pos_str.push_str(&mouse_y.to_string()[..]);
    d.draw_text(&mouse_pos_str, 10, 10, 20, Color::WHITE);
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
                }
                _ => (),
            }
        } else if let ParticleType::Air = *current_draw_particle {
            particle_grid[mouse_y][mouse_x] = Pixel::create_air_particle();
        }
    }

    let mut change_particle = current_draw_particle.clone();
    if d.is_key_pressed(KeyboardKey::KEY_ZERO) {
        change_particle = ParticleType::Air;
    } else if d.is_key_pressed(KeyboardKey::KEY_Z) {
        *particle_grid = [[Pixel::create_air_particle(); GRID_WIDTH]; GRID_HEIGHT];
    } else if d.is_key_pressed(KeyboardKey::KEY_TWO) {
        change_particle = ParticleType::Sand;
    } else if d.is_key_pressed(KeyboardKey::KEY_ONE) {
        change_particle = ParticleType::Stone;
    }

    if change_particle != *current_draw_particle {
        *current_draw_particle = change_particle;
    }

    return (mouse_x, mouse_y);
}

fn main() {
    let (mut rl, thread) = init().size(WIDTH, HEIGHT).fullscreen().title("Pandemic Simulation").build();
    rl.set_target_fps(TARGETFPS);

    let mut particle_grid = [[Pixel::create_air_particle(); GRID_WIDTH]; GRID_HEIGHT];
    let mut current_draw_particle = ParticleType::Stone;
    let mut draw_flag = true;
    let mut fall_dir = true;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);

        let mut moved: Vec<(usize, usize)> = Vec::new();

        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                Pixel::move_pixel(&mut particle_grid, &mut moved, &mut fall_dir, col, row);
                particle_grid[row][col].draw_pixel(&mut d, row, col);
            }
        }

        let (mouse_x, mouse_y) = input_handler(&mut d, &mut particle_grid, &mut draw_flag, &mut current_draw_particle);
        draw_debug_info(&mut d, mouse_x, mouse_y);
    }
}
