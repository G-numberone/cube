use sdl2::pixels::Color;
use sdl2::rect::Point;
use reqwest;
use std::fs::*;
use std::path::Path;
use ndarray::*;
use std::mem::swap;

fn init() {
	let mut exists: bool = false;

	if let Ok(true) = Path::try_exists("./whaattt".as_ref()) { exists = true }
	if !exists {
		create_dir("./whaattt").unwrap();

		// TODO: make it download SDL2 here then move itself as well
	}
}

fn main() {
	//init();

	const CUBE_SIDE_LENGTH: f32 = 400.0;

	static mut POINTS: [[f32; 3]; 8] = [
		[  CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 ], // A ; x: 600
		[ -CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 ], // B ; x: 200
		[ -CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 ], // C
		[  CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 ], // D
		[ -CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 ], // E
		[  CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 ], // F
		[ -CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 ], // G
		[  CUBE_SIDE_LENGTH / 2.0 ,  CUBE_SIDE_LENGTH / 2.0 , -CUBE_SIDE_LENGTH / 2.0 ], // H
	];
	const CONNECTIONS: [[usize; 2]; 12] = [
		[0, 1],
		[0, 3],
		[1, 2],
		[2, 3],
		[1, 4],
		[0, 5],
		[2, 6],
		[3, 7],
		[5, 4],
		[5, 7],
		[4, 6],
		[6, 7],
	];

	let theta: f32 = 0.01_f32.to_radians();
	let sine_theta: f32 = theta.sin();
	let cosine_theta: f32 = theta.cos();

	let (u_x, u_y, u_z) = (
		20.0_f32,
		40.0_f32,
		20.0_f32
	);

	let rotation_matrix: Array2<f32> = arr2(&[
		[
			cosine_theta + u_x.powf(2.0) * (1.0 - cosine_theta),
			u_x * u_y * (1.0 - cosine_theta) - u_z * sine_theta,
			u_x * u_z * (1.0 - cosine_theta) + u_y * sine_theta
		],
		[
			u_y * u_x * (1.0 - cosine_theta) + u_z * sine_theta,
			cosine_theta + u_y.powf(2.0) * (1.0 - cosine_theta),
			u_y * u_z * (1.0 - cosine_theta) - u_x * sine_theta
		],
		[
			u_z * u_x * (1.0 - cosine_theta) - u_y * sine_theta,
			u_z * u_y * (1.0 - cosine_theta) + u_x * sine_theta,
			cosine_theta + u_z.powf(2.0) * (1.0 - cosine_theta)
		],
	]);

	let rotate = || unsafe {
		for point in POINTS.iter_mut() {
			let p_o = arr1(&[
				point[0],
				point[1],
				point[2],
			]);

			let product = p_o.dot(&rotation_matrix);
			*point = [
				product[0],
				product[1],
				product[2]
			];
		}
	};

	fn round(x: f32) -> i32 {
		(x + 0.5) as i32
	}

	let sdl_context = sdl2::init().expect("Failed to initialise sdl context!");
	let video_subsystem = sdl_context.video().unwrap();

	let window_length = 800;
	let window = video_subsystem.window("wa'er", window_length, window_length)
		.allow_highdpi()
		.borderless()
		.build()
		.expect("Failed to create window!");

	let mut canvas = window.into_canvas()
		.present_vsync()
		.accelerated()
		.build()
		.expect("Failed to create canvas!");

	canvas.set_logical_size(window_length, window_length).expect("Failed to set logical size!");
	canvas.set_draw_color(Color::WHITE);
	canvas.clear();

	let mut plot_pixel = |mut x: f32, mut y: f32, c: f32| {
		let (x, y) = (round(x), round(y));

		canvas.set_draw_color(Color::RGBA(255, 255, 255, round(c * 255.0) as u8));
		canvas.draw_point(Point::new(x, y)).unwrap()
	};
	fn ipart(x: f32) -> f32 {
		x.floor()
	}
	fn fpart(x: f32) -> f32 {
		x - ipart(x)
	}
	fn rfpart(x: f32) -> f32 {
		1.0 - fpart(x)
	}

	let mut draw_antialiased_line = |mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32| {
		let steep = (y1 - y0).abs() > (x1 - x0).abs();

		if steep {
			swap(&mut x0, &mut y0);
			swap(&mut x1, &mut y1)
		}
		if x0 > x1 {
			swap(&mut x0, &mut x1);
			swap(&mut y0, &mut y1)
		}

		let dx = x1 - x0;
		let dy = y1 - y0;

		let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

		let mut x_end = round(x0) as f32;
		let mut y_end = y0 + gradient * (x_end - x0);
		let mut x_gap = rfpart(x0 + 0.5);
		let mut x_pxl_1 = x_end;
		let mut y_pxl_1 = ipart(y_end);

		if steep {
			plot_pixel(y_pxl_1, x_pxl_1, rfpart(y_end) * x_gap);
			plot_pixel(y_pxl_1 + 1.0, x_pxl_1, fpart(y_end) * x_gap)
		} else {
			plot_pixel(x_pxl_1, y_pxl_1, rfpart(y_end) * x_gap);
			plot_pixel(x_pxl_1, y_pxl_1 + 1.0, fpart(y_end) * x_gap)
		}

		let mut inter_y = y_end + gradient;
		x_end = round(x1) as f32;
		y_end = y1 + gradient * (x_end - x1);
		x_gap = fpart(x1 + 0.5);
		let x_pxl_2 = x_end;
		let y_pxl_2 = ipart(y_end);

		if steep {
			plot_pixel(y_pxl_2, x_pxl_2, rfpart(y_end) * x_gap);
			plot_pixel(y_pxl_2 + 1.0, x_pxl_2, fpart(y_end) * x_gap)
		} else {
			plot_pixel(x_pxl_2, y_pxl_2, rfpart(y_end) * x_gap);
			plot_pixel(x_pxl_2, y_pxl_2 + 1.0, fpart(y_end) * x_gap)
		}

		if steep {
			for x in round(x_pxl_1 + 1.0)..round(x_pxl_2) {
				plot_pixel(ipart(inter_y), x as f32, rfpart(inter_y));
				plot_pixel(ipart(inter_y) + 1.0, x as f32, fpart(inter_y));

				inter_y = inter_y + gradient;
			}
		} else {
			for x in round(x_pxl_1 + 1.0)..round(x_pxl_2) {
				plot_pixel(x as f32, ipart(inter_y), rfpart(inter_y));
				plot_pixel(x as f32, ipart(inter_y) + 1.0, fpart(inter_y));

				inter_y = inter_y + gradient;
			}
		}
	};

	let (camera_x, camera_y, camera_z) = (
		0.0_f32,
		0.0_f32,
		420.0_f32
	);
	let (angle_x, angle_y, angle_z) = (
		0.0_f32,
		0.0_f32,
		0.0_f32
	);
	let (cam_offset_x, cam_offset_y, cam_offset_z) = (
		0.0_f32,
		0.0_f32,
		-272.0_f32
	);

	let camera_transform = |point_a: [f32; 3]| -> [f32; 2] {
		let (x, y, z) = (
			point_a[0] - camera_x,
			point_a[1] - camera_y,
			point_a[2] - camera_z
		);
		let (d_x, d_y, d_z) = (
			angle_y.cos() * (angle_z.sin() * y + angle_z.cos() * x) - angle_y.sin() * z,
			angle_x.sin() * (angle_y.cos() * z + angle_y.sin() * (angle_z.sin() * y + angle_z.cos() * x)) + angle_x.cos() * (angle_z.cos() * y - angle_z.sin() * x),
			angle_x.cos() * (angle_z.cos() * z + angle_y.sin() * (angle_z.sin() * y + angle_z.cos() * x)) - angle_x.sin() * (angle_z.cos() * y - angle_z.sin() * x)
		);
		let (b_x, b_y) = (
			(cam_offset_z / d_z) * d_x + cam_offset_x,
			(cam_offset_z / d_z) * d_y + cam_offset_y
		);

		[b_x, b_y]
	};

	loop {
		rotate();

		canvas.set_draw_color(Color::WHITE);
		canvas.clear();

		unsafe {
			canvas.set_draw_color(Color::BLACK);
			for v in CONNECTIONS {
				let point_a1 = POINTS[v[0]];
				let point_a2 = POINTS[v[1]];

				let start = camera_transform(point_a1);
				let end = camera_transform(point_a2);

				draw_antialiased_line(start[0], start[1], end[0], end[1]);
				/*
				canvas.draw_line(
					Point::new(
						round(start[0]) + CUBE_SIDE_LENGTH as i32,
						round(start[1]) + CUBE_SIDE_LENGTH as i32
					),
					Point::new(
						round(end[0]) + CUBE_SIDE_LENGTH as i32,
						round(end[1]) + CUBE_SIDE_LENGTH as i32
					)
				).expect("Failed to draw line!");
				*/
			}

			canvas.present();
		}
	}
}