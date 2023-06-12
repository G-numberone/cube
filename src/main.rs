use sdl2::pixels::Color;
use sdl2::rect::Point;
use reqwest;
use std::fs::*;
use std::path::Path;
use ndarray::*;
use sdl2::video::WindowPos;

fn init() {
	let mut exists: bool = false;

	if let Ok(true) = Path::try_exists("./whaattt".as_ref()) { exists = true }
	if !exists {
		create_dir("./whaattt").unwrap();

		// TODO: make it download SDL2 here then move itself as well
	}
}

static WINDOW_POSITIONS: [[i32; 2]; 8] = [
	[0, 0],
	[500, 0],
	[1000, 0],
	[1500, 0],
	[0, 500],
	[500, 500],
	[1000, 500],
	[1500, 500]
];
static mut WINDOW_NUMBER: usize = 0;

unsafe fn get_window_count() -> usize {
	let mut exists: bool = false;

	if let Ok(true) = Path::try_exists("open_windows.txt".as_ref()) { exists = true }
	if !exists {
		write("open_windows.txt", "0").unwrap();
		return 0_usize
	}

	let windows = String::from_utf8(read("open_windows.txt").unwrap()).unwrap();


	if windows.len() >= 8 {
		std::process::abort();
	} else {
		loop {
			if windows.contains(&WINDOW_NUMBER.to_string()) {
				WINDOW_NUMBER += 1
			} else {
				let new_windows = format!("{}{}", &windows, WINDOW_NUMBER);
				write("open_windows.txt", new_windows.as_str()).unwrap();
				break
			}
		}
		WINDOW_NUMBER
	}
}

fn check_for_crashed_windows() {
	todo!()
}

fn main() {
	//init();

	const OBJ_SIDE_LENGTH: f32 = 300.0;
	static mut CUBE_POINTS: [[f32; 3]; 8] = [
		[  OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 ], // A ; x: 600
		[ -OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 ], // B ; x: 200
		[ -OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 ], // C
		[  OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 ], // D
		[ -OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 ], // E
		[  OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 ], // F
		[ -OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 ], // G
		[  OBJ_SIDE_LENGTH / 2.0 ,  OBJ_SIDE_LENGTH / 2.0 , -OBJ_SIDE_LENGTH / 2.0 ], // H
	];
	const CUBE_CONNECTIONS: [[usize; 2]; 12] = [
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
		for point in CUBE_POINTS.iter_mut() {
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

	let window_length = 500;
	let mut window = video_subsystem.window("virus.exe", window_length, window_length)
		.allow_highdpi()
		.borderless()
		.build()
		.expect("Failed to create window!");

	unsafe {
		let pos = WINDOW_POSITIONS[get_window_count()];
		let (x, y) = (WindowPos::Positioned(pos[0]), WindowPos::Positioned(pos[1]));
		window.set_position(x, y);
	}

	let mut canvas = window.into_canvas()
		.present_vsync()
		.accelerated()
		.build()
		.expect("Failed to create canvas!");

	canvas.set_logical_size(window_length, window_length).expect("Failed to set logical size!");
	canvas.set_draw_color(Color::WHITE);
	canvas.clear();

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
			for v in CUBE_CONNECTIONS {
				let point_a1 = CUBE_POINTS[v[0]];
				let point_a2 = CUBE_POINTS[v[1]];

				let start = camera_transform(point_a1);
				let end = camera_transform(point_a2);

				canvas.draw_line(
					Point::new(
						round(start[0]) + window_length as i32 / 2,
						round(start[1]) + window_length as i32 / 2
					),
					Point::new(
						round(end[0]) + window_length as i32 / 2,
						round(end[1]) + window_length as i32 / 2
					)
				).expect("Failed to draw line!");
			}

			canvas.present();
		}
	}
}