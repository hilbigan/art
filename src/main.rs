use rand::prelude::*;
use image::{RgbImage, Rgb};
use std::time::Instant;
use image::io::Reader as ImageReader;
use rayon::prelude::*;
use std::env;
use itertools::Itertools;

const PUNISH_TOO_MANY_NEIGHBORS: usize = 0;

fn main() {
    let args = env::args().collect_vec();
    let width: usize = args[1].parse().expect("arg 1: usize width");
    let height: usize = args[2].parse().expect("arg 2: usize height");
    let input_path = args[3].clone();
    let output_path = if args.len() > 4 { args[4].clone() } else { "out.png".into() };
    println!("{}x{} palette: {}", width, height, input_path);
    
    let mut rng = rand::thread_rng();
    let mut input = vec![vec![Color { r: 0, g: 0, b: 0 }; width]; height];
    let input_image = ImageReader::open(input_path).unwrap().decode().unwrap();
    let image_offset_x = rng.gen_range(0 .. (input_image.width() as usize - width));
    let image_offset_y = rng.gen_range(0 .. (input_image.height() as usize - height));
    let input_image_rgb = input_image.as_rgb8().unwrap();
    for x in image_offset_x .. image_offset_x + width {
        for y in image_offset_y .. image_offset_y + height {
            let rgb = input_image_rgb.get_pixel(x as u32 % input_image.width(), y as u32 % input_image.height()).0;
            input[y - image_offset_y][x - image_offset_x] = Color { r: rgb[0] as usize, g: rgb[1] as usize, b: rgb[2] as usize };
        }
    }
    
    let mut starting_configurations = vec![
        vec![Point { x: width / 2, y: height / 2 }],
        vec![Point { x: 0, y: 0 }],
        vec![Point { x: width-1, y: height-1 }],
        vec![Point { x: width / 4, y: height / 4 }, Point { x: 3 * width / 4, y: height / 4 }, Point { x: width / 4, y: 3 * height / 4 }, Point { x: 3 * width / 4, y: 3 * height / 4 }],
        vec![Point { x: width / 4, y: height / 4 }, Point { x: width / 2, y: height / 2 }, Point { x: 3 * width / 4, y: 3 * height / 4 }],
    ];
    let chosen_starting_configuration = starting_configurations.remove(rng.gen_range(0 .. starting_configurations.len()));
    println!("Seed: {:?}", chosen_starting_configuration);
    
    let start = Instant::now();
    let generated = generate_image(width, height, &input, chosen_starting_configuration);
    println!("{}ms", start.elapsed().as_millis());
    let mut img = RgbImage::new(width as u32, height as u32);
    for x in 0 .. width {
        for y in 0 .. height {
            let color = generated[y][x];
            img.put_pixel(x as u32, y as u32, Rgb([color.r as u8, color.g as u8, color.b as u8]));
        }
    }
    img.save(output_path).expect("write image");
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
        (isize::pow(self.r as isize - other.r as isize, 2) + isize::pow(self.g as isize - other.g as isize, 2) + isize::pow(self.b as isize - other.b as isize, 2)) as usize
    }
}

fn generate_image(width: usize, height: usize, input: &Vec<Vec<Color>>, starting_spots: Vec<Point>) -> Vec<Vec<Color>> {
    let mut pixels = vec![vec![Color { r: 0, g: 0, b: 0 }; width]; height];
    let mut cell_state = vec![vec![CellState::Empty; width]; height];
    let mut spots = starting_spots;
    let mut x;
    let mut y;

    macro_rules! neighbors {
        ( $spot:expr, $x:expr, $y:expr, $fn:block ) => {
            let x_upper = usize::min($spot.x + 1, width-1);
            let x_lower = isize::max($spot.x as isize - 1, 0) as usize;
            let y_upper = usize::min($spot.y + 1, height-1);
            let y_lower = isize::max($spot.y as isize - 1, 0) as usize;
            for xx in x_lower ..= x_upper {
                for yy in y_lower ..= y_upper {
                    if xx == $spot.x && yy == $spot.y {
                        continue;
                    }

                    $x = xx;
                    $y = yy;

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
        let best_spot_index = spots.par_iter().enumerate().fold(|| (0, 1 << 24), |(best_index, best_value), (index, spot)| {
            let mut x;
            let mut y;
            let mut value = 0;
            let mut created_neighbors = 0;
        
            neighbors!(spot, x, y, {
                if cell_state[y][x] == CellState::Filled {
                    value += pixels[y][x].dist(&color);
        
                    if value > best_value {
                        // No need to go further
                        break
                    }
                } else {
                    created_neighbors += 1;
                }
            });
            
            value += created_neighbors * PUNISH_TOO_MANY_NEIGHBORS;
        
            if value < best_value {
                (index, value)
            } else {
                (best_index, best_value)
            }
        }).min_by_key(|(_, value)| *value).unwrap().0;

        let best_spot = spots.remove(best_spot_index);
        pixels[best_spot.y][best_spot.x] = color;
        cell_state[best_spot.y][best_spot.x] = CellState::Filled;
        neighbors!(best_spot, x, y, {
            if cell_state[y][x] == CellState::Empty {
                cell_state[y][x] = CellState::Fringe;
                spots.push(Point { x, y });
            }
        });
        
        placed += 1;
        print!("\r{:7}/{:7} ({:3.2}%)", placed, width * height, (placed as f32) / ((width * height) as f32) * 100.);
    }
    println!();

    return pixels;
}
