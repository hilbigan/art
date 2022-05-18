use rand::prelude::*;
use image::{RgbImage, Rgb};
use std::time::Instant;
use std::collections::BTreeSet;
use image::io::Reader as ImageReader;

fn main() {
    let width = 200;
    let height = 200;
    let mut rng = rand::thread_rng();
    let mut input = vec![vec![Color { r: 0, g: 0, b: 0 }; width]; height];
    let input_image = ImageReader::open("./monet.jpg").unwrap().decode().unwrap();
    let input_image_rgb = input_image.as_rgb8().unwrap();
    for x in 0 .. width {
        for y in 0 .. height {
            let rgb = input_image_rgb.get_pixel(x as u32, y as u32).0;
            //input[y][x] = Color { r: rng.gen_range(0 .. 255), g: rng.gen_range(0 .. 255), b: rng.gen_range(0 .. 255) };
            input[y][x] = Color { r: rgb[0] as usize, g: rgb[1] as usize, b: rgb[2] as usize };
        }
    }
    let start = Instant::now();
    let generated = generate_image(width, height, &input);
    println!("{}ms", start.elapsed().as_millis());
    let mut img = RgbImage::new(width as u32, height as u32);
    for x in 0 .. width {
        for y in 0 .. height {
            let color = generated[y][x];
            img.put_pixel(x as u32, y as u32, Rgb([color.r as u8, color.g as u8, color.b as u8]));
        }
    }
    img.save("./out.png");
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug, Copy, Clone)]
struct Color {
    r: usize,
    g: usize,
    b: usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CellState {
    Empty,
    Fringe,
    Filled,
}

impl Color {
    fn dist(&self, other: &Color) -> usize {
        (isize::pow((self.r as isize - other.r as isize), 2) + isize::pow((self.g as isize - other.g as isize), 2) + isize::pow((self.b as isize - other.b as isize), 2)) as usize
    }
}

fn generate_image(width: usize, height: usize, input: &Vec<Vec<Color>>) -> Vec<Vec<Color>> {
    let mut pixels = vec![vec![Color { r: 0, g: 0, b: 0 }; width]; height];
    let mut cell_state = vec![vec![CellState::Empty; width]; height];
    let mut avg_color =  vec![vec![Color { r: 0, g: 0, b: 0 }; width]; height];
    let mut spots = vec![Point { x: width/2, y: height/2 }];
    let mut x = 0;
    let mut y = 0;

    macro_rules! neighbors {
        ( $spot:expr, $fn:block ) => {
            let x_upper = usize::min($spot.x + 1, width-1);
            let x_lower = isize::max($spot.x as isize - 1, 0) as usize;
            let y_upper = usize::min($spot.y + 1, height-1);
            let y_lower = isize::max($spot.y as isize - 1, 0) as usize;
            for xx in x_lower ..= x_upper {
                for yy in y_lower ..= y_upper {
                    if xx == $spot.x && yy == $spot.y {
                        continue;
                    }

                    x = xx;
                    y = yy;

                    $fn
                }
            }
        }
    }

    let mut rng = rand::thread_rng();
    let mut placed = 0;
    while !spots.is_empty() {
        let rand_x = rng.gen_range(0 .. width);
        let rand_y = rng.gen_range(0 .. height);
        let color = input[rand_y][rand_x];
        let best_spot_index = spots.iter().enumerate().min_by_key(|(i, spot)| {
            let mut total_distance = 0;
            neighbors!(spot, {
                if cell_state[y][x] == CellState::Filled {
                    total_distance += pixels[y][x].dist(&color);
                }
            });
            total_distance
        }).unwrap().0;
        let best_spot = spots.remove(best_spot_index);
        pixels[best_spot.y][best_spot.x] = color;
        cell_state[best_spot.y][best_spot.x] = CellState::Filled;
        neighbors!(best_spot, {
            if cell_state[y][x] == CellState::Empty {
                cell_state[y][x] = CellState::Fringe;
                spots.push(Point { x, y });
            }
        });
        
        placed += 1;
        print!("\r{:06}/{:06}", placed, width * height);
    }
    println!();

    return pixels;
}
