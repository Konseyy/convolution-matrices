use image::{DynamicImage, GenericImageView, Rgba};
use ndarray::arr2;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

struct Point {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}
fn process_image(input_path: &str) -> Option<(u32, u32, DynamicImage)> {
    let img = image::open(&Path::new(input_path));
    if img.is_err() {
        println!("Error: {}", img.err().unwrap());
        return None;
    }

    let mut points: Vec<Point> = Vec::new();
    let width = img.as_ref().unwrap().width();
    let height = img.as_ref().unwrap().height();

    return Some((width, height, img.as_ref().unwrap().clone()));
}

fn get_neighbor_coordinates(x: u32, y: u32, width: u32, height: u32) -> Vec<(u32, u32)> {
    let mut neighbors: Vec<(u32, u32)> = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < width - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < height - 1 {
        neighbors.push((x, y + 1));
    }
    return neighbors;
}

fn main() {
    let sharpen_matrix_3 = arr2(&[[0, -1, 0], [-1, 4, -1], [0, -1, 0]]);
    let sharpen_matrix_5 = arr2(&[
        [0, 0, -1, 0, 0],
        [0, -1, -2, -1, 0],
        [-1, -2, 16, -2, -1],
        [0, -1, -2, -1, 0],
        [0, 0, -1, 0, 0],
    ]);
    let blur_matrix_3 = arr2(&[[1, 2, 1], [2, 4, 2], [1, 2, 1]]);
    let blur_matrix_5 = arr2(&[
        [1, 4, 6, 4, 1],
        [4, 16, 24, 16, 4],
        [6, 24, 36, 24, 6],
        [4, 16, 24, 16, 4],
        [1, 4, 6, 4, 1],
    ]);

    let sharpen: bool; // true for sharpen, false for blur
    let mut src_img_path = String::new();

    print!("Please enter image path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut src_img_path)
        .expect("Did not enter a correct string");
    if let Some('\n') = src_img_path.chars().next_back() {
        src_img_path.pop();
    }
    if let Some('\r') = src_img_path.chars().next_back() {
        src_img_path.pop();
    }
    let mut sharp_or_blur_input = String::new();
    print!("Please enter whether to sharpen or not ('t' or 'f'): ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut sharp_or_blur_input)
        .expect("Did not enter a correct string");
    if let Some('\n') = sharp_or_blur_input.chars().next_back() {
        sharp_or_blur_input.pop();
    }
    if let Some('\r') = sharp_or_blur_input.chars().next_back() {
        sharp_or_blur_input.pop();
    }

    if sharp_or_blur_input == "f" {
        sharpen = false;
    } else if sharp_or_blur_input == "t" {
        sharpen = true;
    } else {
        println!("Please enter either 't' or 'f'");
        return;
    }

    let img_info_result = process_image(&src_img_path);
    if img_info_result.is_none() {
        println!("Could not process image {}", src_img_path);
        return;
    }
    let img_info = img_info_result.unwrap();
    let mut new_img = image::ImageBuffer::new(img_info.0 * 3, img_info.1);

    for mode in [3, 5] {
        for pixel in img_info.2.pixels() {
            let x = pixel.0;
            let y = pixel.1;
            new_img.put_pixel(x, y, image::Rgb([pixel.2[0], pixel.2[1], pixel.2[2]]));
            if mode == 3 {
                if x < 1 || x > img_info.0 - 2 || y < 1 || y > img_info.1 - 2 {
                    continue;
                }
            }
            if mode == 5 {
                if x < 2 || x > img_info.0 - 3 || y < 2 || y > img_info.1 - 3 {
                    continue;
                }
            }
            let neighbors_3 = [
                [x - 1, y - 1],
                [x, y - 1],
                [x + 1, y - 1],
                [x - 1, y],
                [x, y],
                [x + 1, y],
                [x - 1, y + 1],
                [x, y + 1],
                [x + 1, y + 1],
            ];
            let neighbors_5 = [
                [x - 2, y - 2],
                [x - 1, y - 2],
                [x, y - 2],
                [x + 1, y - 2],
                [x + 2, y - 2],
                [x - 2, y - 1],
                [x - 1, y - 1],
                [x, y - 1],
                [x + 1, y - 1],
                [x + 2, y - 1],
                [x - 2, y],
                [x - 1, y],
                [x, y],
                [x + 1, y],
                [x + 2, y],
                [x - 2, y + 1],
                [x - 1, y + 1],
                [x, y + 1],
                [x + 1, y + 1],
                [x + 2, y + 1],
                [x - 2, y + 2],
                [x - 1, y + 2],
                [x, y + 2],
                [x + 1, y + 2],
                [x + 2, y + 2],
            ];

            let current_rgb = img_info.2.get_pixel(x, y);
            if mode == 3 {
                // mode 3
                if sharpen {
                    let dot_product = neighbors_3
                        .iter()
                        .map(|neighbor| {
                            let neighbor_rgb = img_info.2.get_pixel(neighbor[0], neighbor[1]);
                            let neighbor_r = neighbor_rgb[0] as i32;
                            let neighbor_g = neighbor_rgb[1] as i32;
                            let neighbor_b = neighbor_rgb[2] as i32;
                            let sharpen_matrix_value: i32 = sharpen_matrix_3[[
                                (neighbor[0] - x + 1) as usize,
                                (neighbor[1] - y + 1) as usize,
                            ]];
                            let neighbor_r = neighbor_r * sharpen_matrix_value;
                            let neighbor_g = neighbor_g * sharpen_matrix_value;
                            let neighbor_b = neighbor_b * sharpen_matrix_value;
                            return (neighbor_r, neighbor_g, neighbor_b);
                        })
                        .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                        .unwrap();
                    let new_r = current_rgb[0] as i32 + dot_product.0;
                    let new_g = current_rgb[1] as i32 + dot_product.1;
                    let new_b = current_rgb[2] as i32 + dot_product.2;
                    let new_r = if new_r > 255 { 255 } else { new_r };
                    let new_g = if new_g > 255 { 255 } else { new_g };
                    let new_b = if new_b > 255 { 255 } else { new_b };
                    let new_r = if new_r < 0 { 0 } else { new_r };
                    let new_g = if new_g < 0 { 0 } else { new_g };
                    let new_b = if new_b < 0 { 0 } else { new_b };
                    let new_rgb = [new_r as u8, new_g as u8, new_b as u8];
                    new_img.put_pixel(x + img_info.0, y, image::Rgb(new_rgb));
                } else {
                    let dot_product = neighbors_3
                        .iter()
                        .map(|neighbor| {
                            let neighbor_rgb = img_info.2.get_pixel(neighbor[0], neighbor[1]);
                            let neighbor_r = neighbor_rgb[0] as i32;
                            let neighbor_g = neighbor_rgb[1] as i32;
                            let neighbor_b = neighbor_rgb[2] as i32;
                            let blur_matrix_value = blur_matrix_3[[
                                (neighbor[0] - x + 1) as usize,
                                (neighbor[1] - y + 1) as usize,
                            ]];
                            let neighbor_r = neighbor_r * blur_matrix_value;
                            let neighbor_g = neighbor_g * blur_matrix_value;
                            let neighbor_b = neighbor_b * blur_matrix_value;
                            return (neighbor_r, neighbor_g, neighbor_b);
                        })
                        .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                        .unwrap();
                    let new_r = dot_product.0 / 16;
                    let new_g = dot_product.1 / 16;
                    let new_b = dot_product.2 / 16;
                    let new_r = if new_r > 255 { 255 } else { new_r };
                    let new_g = if new_g > 255 { 255 } else { new_g };
                    let new_b = if new_b > 255 { 255 } else { new_b };
                    let new_r = if new_r < 0 { 0 } else { new_r };
                    let new_g = if new_g < 0 { 0 } else { new_g };
                    let new_b = if new_b < 0 { 0 } else { new_b };
                    let new_rgb = [new_r as u8, new_g as u8, new_b as u8];
                    new_img.put_pixel(x + img_info.0, y, image::Rgb(new_rgb));
                }
            } else {
                // mode 5
                if sharpen {
                    let dot_product = neighbors_5
                        .iter()
                        .map(|neighbor| {
                            let neighbor_rgb = img_info.2.get_pixel(neighbor[0], neighbor[1]);
                            let neighbor_r = neighbor_rgb[0] as i32;
                            let neighbor_g = neighbor_rgb[1] as i32;
                            let neighbor_b = neighbor_rgb[2] as i32;
                            let sharpen_matrix_value = sharpen_matrix_5[[
                                (neighbor[0] - x + 2) as usize,
                                (neighbor[1] - y + 2) as usize,
                            ]];
                            let neighbor_r = neighbor_r * sharpen_matrix_value;
                            let neighbor_g = neighbor_g * sharpen_matrix_value;
                            let neighbor_b = neighbor_b * sharpen_matrix_value;
                            return (neighbor_r, neighbor_g, neighbor_b);
                        })
                        .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                        .unwrap();
                    let new_r = current_rgb[0] as i32 + dot_product.0;
                    let new_g = current_rgb[1] as i32 + dot_product.1;
                    let new_b = current_rgb[2] as i32 + dot_product.2;
                    let new_r = if new_r > 255 { 255 } else { new_r };
                    let new_g = if new_g > 255 { 255 } else { new_g };
                    let new_b = if new_b > 255 { 255 } else { new_b };
                    let new_r = if new_r < 0 { 0 } else { new_r };
                    let new_g = if new_g < 0 { 0 } else { new_g };
                    let new_b = if new_b < 0 { 0 } else { new_b };
                    let new_rgb = [new_r as u8, new_g as u8, new_b as u8];
                    new_img.put_pixel(x + img_info.0 * 2, y, image::Rgb(new_rgb));
                } else {
                    let dot_product = neighbors_5
                        .iter()
                        .map(|neighbor| {
                            let neighbor_rgb = img_info.2.get_pixel(neighbor[0], neighbor[1]);
                            let neighbor_r = neighbor_rgb[0] as i32;
                            let neighbor_g = neighbor_rgb[1] as i32;
                            let neighbor_b = neighbor_rgb[2] as i32;
                            let blur_matrix_value = blur_matrix_5[[
                                (neighbor[0] - x + 2) as usize,
                                (neighbor[1] - y + 2) as usize,
                            ]];
                            let neighbor_r = neighbor_r * blur_matrix_value;
                            let neighbor_g = neighbor_g * blur_matrix_value;
                            let neighbor_b = neighbor_b * blur_matrix_value;
                            return (neighbor_r, neighbor_g, neighbor_b);
                        })
                        .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                        .unwrap();
                    let new_r = dot_product.0 / 256;
                    let new_g = dot_product.1 / 256;
                    let new_b = dot_product.2 / 256;
                    let new_r = if new_r > 255 { 255 } else { new_r };
                    let new_g = if new_g > 255 { 255 } else { new_g };
                    let new_b = if new_b > 255 { 255 } else { new_b };
                    let new_r = if new_r < 0 { 0 } else { new_r };
                    let new_g = if new_g < 0 { 0 } else { new_g };
                    let new_b = if new_b < 0 { 0 } else { new_b };
                    let new_rgb = [new_r as u8, new_g as u8, new_b as u8];
                    new_img.put_pixel(x + img_info.0 * 2, y, image::Rgb(new_rgb));
                }
            }
        }
    }

    fs::create_dir_all("images").unwrap();
    new_img.save("images/comparison.png").unwrap();
}
