use std::env::args;
use std::fs::OpenOptions;
use std::io::stdin;
use std::process::exit;
use image;
use image::{DynamicImage, GenericImageView, Pixel};
use color_name::{Color};
use stl_io::{Normal, Triangle, Vertex};

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() == 1 {
        println!("You must provide the absolute/relative path of the QR code image as the first argument");
        exit(1)
    }

    let image_path = args[1].as_str();
    let qr_image = image::open(image_path);
    if qr_image.is_err() {
        println!("Could not find an image at the path \"{}\"", image_path);
        exit(1);
    }

    let args_color = if args.len() > 2 {
        Some(args[2].trim().to_string())
    } else {
        None
    };

    let args_grid_size = if args.len() > 3 {
        let args_grid_size_parsed = args[3].trim().parse::<u32>();
        if args_grid_size_parsed.is_err() {
            panic!("The grid size argument is not a valid whole number!");
        }

        Some(args_grid_size_parsed.unwrap())
    } else {
        None
    };

    let qr_image = qr_image.unwrap().grayscale();

    println!("Processing the QR code image at location {}", image_path);
    println!("This image is {}px by {}px", qr_image.width(), qr_image.height());

    let (primary_color, secondary_color) = get_colors(&qr_image);

    let primary_color_name = Color::similar(primary_color).to_lowercase();
    let secondary_color_name = Color::similar(secondary_color).to_lowercase();

    println!("The two colors used are {} and {}", primary_color_name, secondary_color_name);

    let mut final_primary_color = [0, 0, 0];

    if args_color.is_some() {
        let args_color = args_color.unwrap();
        if args_color != primary_color_name && args_color != secondary_color_name {
            panic!("The color argument \"{}\" is not a valid color option!", args_color);
        }
    } else {
        loop {
            println!("Which color would you like to have the geometry ({} or {})?", primary_color_name, secondary_color_name);

            let mut color_choice = String::new();
            stdin().read_line(&mut color_choice).unwrap();

            let color_choice = color_choice.trim();

            if color_choice == primary_color_name {
                final_primary_color = secondary_color;

                break;
            } else if color_choice == secondary_color_name {
                final_primary_color = primary_color;

                break;
            }

            println!("That is not a valid color option! Try again. . .");
        }
    }

    let mut grid_size = 0;

    if args_grid_size.is_some() {
        grid_size = args_grid_size.unwrap();
    } else {
        loop {
            println!("What is the size of a single grid cell in pixels?");
            let mut grid_size_input = String::new();
            stdin().read_line(&mut grid_size_input).unwrap();

            let parsed_grid_size = grid_size_input.trim().parse();
            if parsed_grid_size.is_ok() {
                grid_size = parsed_grid_size.unwrap();
                break;
            }

            println!("That is not a valid whole number! Try again. . .");
        }
    }

    let grid = build_representation(&qr_image, grid_size, final_primary_color);
    create_stl(grid);

    println!("STL successfully created at \"qrcode.stl\"");
}

fn get_colors(qr_image: &DynamicImage) -> ([u8; 3], [u8; 3]) {
    let mut colors: Vec<[u8; 3]> = vec![];
    let mut occurances: Vec<(usize, usize)> = vec![];

    for x in 0..qr_image.width() {
        for y in 0..qr_image.height() {
            let pixel = qr_image.get_pixel(x, y);
            let value = pixel.to_rgb().0;

            if !colors.contains(&value) {
                colors.push(value);
                occurances.push((0, occurances.len()));
            }

            let color_index = colors.iter().position(|&x| x == value).expect("INTERNAL ERROR");
            occurances[color_index].0 += 1;
        }
    }

    occurances.sort_by(|&a, &b| a.0.partial_cmp(&b.0).unwrap());

    let primary_color_index = occurances[occurances.len() - 1].1;
    let secondary_color_index = occurances[occurances.len() - 2].1;

    let primary_color = colors[primary_color_index];
    let secondary_color = colors[secondary_color_index];

    (primary_color, secondary_color)
}

fn build_representation(qr_image: &DynamicImage, grid_size: u32, primary_color: [u8; 3]) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![];

    let mut x = grid_size / 2;
    let mut y;
    while x < qr_image.width() {
        grid.push(vec![]);

        y = grid_size / 2;
        while y < qr_image.height() {
            let is_primary = qr_image.get_pixel(x, y).to_rgb().0 == primary_color;

            grid.last_mut().unwrap().push(is_primary);
            y += grid_size;
        }

        x += grid_size;
    }

    grid
}

fn create_stl(grid: Vec<Vec<bool>>) {
    let mut mesh: Vec<Triangle> = vec![];

    let mut i = 1f32;
    for x in grid {
        let mut j = 1f32;
        for y in x {
            if !y {
                mesh.append(&mut create_block(i, j, 2.0).iter().as_slice().to_vec());
            }
            j += 2.0;
        }

        i += 2.0;
    }

    let mut file = OpenOptions::new().write(true).create_new(true).open("qrcode.stl").unwrap();
    stl_io::write_stl(&mut file, mesh.iter()).unwrap();
}

fn create_block(x: f32, y: f32, height: f32) -> [Triangle; 12] {
    let block = [
        // Top triangles
        Triangle {
            normal: Normal::new([0.0, 0.0, 1.0]),
            vertices: [
                Vertex::new([-1.0 + x, 1.0 + y, 0.0]),
                Vertex::new([-1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([1.0 + x, 1.0 + y, 0.0]),
            ],
        },
        Triangle {
            normal: Normal::new([0.0, 0.0, 1.0]),
            vertices: [
                Vertex::new([-1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([1.0 + x, 1.0 + y, 0.0]),
            ],
        },
        // Bottom Triangles
        Triangle {
            normal: Normal::new([0.0, 0.0, -1.0]),
            vertices: [
                Vertex::new([-1.0 + x, 1.0 + y, -height]),
                Vertex::new([1.0 + x, 1.0 + y, -height]),
                Vertex::new([-1.0 + x, -1.0 + y, -height]),
            ],
        },
        Triangle {
            normal: Normal::new([0.0, 0.0, -1.0]),
            vertices: [
                Vertex::new([-1.0 + x, -1.0 + y, -height]),
                Vertex::new([1.0 + x, 1.0 + y, -height]),
                Vertex::new([1.0 + x, -1.0 + y, -height]),
            ],
        },
        // Left-Facing Triangles
        Triangle {
            normal: Normal::new([-1.0, 0.0, 0.0]),
            vertices: [
                Vertex::new([-1.0 + x, 1.0 + y, 0.0]),
                Vertex::new([-1.0 + x, 1.0 + y, -height]),
                Vertex::new([-1.0 + x, -1.0 + y, 0.0]),
            ],
        },
        Triangle {
            normal: Normal::new([-1.0, 0.0, 0.0]),
            vertices: [
                Vertex::new([-1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([-1.0 + x, 1.0 + y, -height]),
                Vertex::new([-1.0 + x, -1.0 + y, -height]),
            ],
        },
        // Right-Facing Triangles
        Triangle {
            normal: Normal::new([1.0, 0.0, 0.0]),
            vertices: [
                Vertex::new([1.0 + x, 1.0 + y, 0.0]),
                Vertex::new([1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([1.0 + x, 1.0 + y, -height]),
            ],
        },
        Triangle {
            normal: Normal::new([1.0, 0.0, 0.0]),
            vertices: [
                Vertex::new([1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([1.0 + x, -1.0 + y, -height]),
                Vertex::new([1.0 + x, 1.0 + y, -height]),
            ],
        },
        // Upwards-Facing Triangles
        Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [
                Vertex::new([1.0 + x, 1.0 + y, 0.0]),
                Vertex::new([1.0 + x, 1.0 + y, -height]),
                Vertex::new([-1.0 + x, 1.0 + y, 0.0]),
            ],
        },
        Triangle {
            normal: Normal::new([0.0, 1.0, 0.0]),
            vertices: [
                Vertex::new([-1.0 + x, 1.0 + y, 0.0]),
                Vertex::new([1.0 + x, 1.0 + y, -height]),
                Vertex::new([-1.0 + x, 1.0 + y, -height]),
            ],
        },
        // Downwards-Facing Triangles
        Triangle {
            normal: Normal::new([0.0, -1.0, 0.0]),
            vertices: [
                Vertex::new([1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([-1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([1.0 + x, -1.0 + y, -height]),
            ],
        },
        Triangle {
            normal: Normal::new([0.0, -1.0, 0.0]),
            vertices: [
                Vertex::new([-1.0 + x, -1.0 + y, 0.0]),
                Vertex::new([-1.0 + x, -1.0 + y, -height]),
                Vertex::new([1.0 + x, -1.0 + y, -height]),
            ],
        },
    ];

    block
}