use sdl2::pixels::Color;
use sdl2::rect::Point;
//use reqwest;
use ndarray::*;

fn main() {
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

	let theta: f32 = 0.005_f32.to_radians();
	let sine_theta: f32 = theta.sin();
	let cosine_theta: f32 = theta.cos();

	let u_x: f32 = 2.0;
	let u_y: f32 = 2.0;
	let u_z: f32 = 2.0;

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

	loop {
		rotate();

		canvas.set_draw_color(Color::WHITE);
		canvas.clear();

		unsafe {
			canvas.set_draw_color(Color::BLACK);
			for v in CONNECTIONS {
				let start = [POINTS[v[0]][0], POINTS[v[0]][1]];
				let end = [POINTS[v[1]][0], POINTS[v[1]][1]];

				canvas.draw_line(
					Point::new(round(start[0]) + 200, round(start[1]) + 200),
					Point::new(round(end[0]) + 200, round(end[1]) + 200)
				).expect("Failed to draw line!");
			}

			canvas.present();
		}
	}
}