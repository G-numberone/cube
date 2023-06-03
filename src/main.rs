use minifb::*;

fn main() {
	static mut POINTS: [[f32; 3]; 8] = [
		[600.0, 200.0, 100.0],  // A
		[200.0, 200.0, 100.0],  // B
		[200.0, 200.0, -100.0], // C
		[600.0, 200.0, -100.0], // D
		[200.0, 600.0, 100.0],  // E
		[600.0, 600.0, 100.0],  // F
		[200.0, 600.0, -100.0], // G
		[600.0, 600.0, -100.0], // H
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

	const TOTAL_LINE_LENGTH: usize = 4000;
	static mut LINE_POINTS: [[i32; 2]; TOTAL_LINE_LENGTH] = [[0, 0]; TOTAL_LINE_LENGTH];

	let theta: f32 = 0.1;
	let sine_theta: f32 = theta.sin();
	let cosine_theta: f32 = theta.cos();

	let rotate_x = || unsafe {
		POINTS.into_iter().for_each(|mut point| {
			point = [
				point[0],
				point[1] * cosine_theta + point[2] * -sine_theta,
				point[1] * sine_theta + point[2] * cosine_theta,
			]
		});
	};
	let rotate_y = || unsafe {
		POINTS.into_iter().for_each(|mut point| {
			point = [
				point[0] * cosine_theta + point[2] * sine_theta,
				point[1],
				point[0] * -sine_theta + point[2] * cosine_theta,
			]
		});
	};
	let rotate_z = || unsafe {
		POINTS.into_iter().for_each(|mut point| {
			point = [
				point[0] * cosine_theta + point[1] * -sine_theta,
				point[0] * sine_theta + point[1] * cosine_theta,
				point[2],
			];
		});
	};

	const SIDE_LENGTH: u32 = 800;
	let mut window = Window::new(
		"wa'er",
		SIDE_LENGTH as usize,
		SIDE_LENGTH as usize,
		WindowOptions::default(),
	)
		.expect("something happened :(");
	window.limit_update_rate(None);

	const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
		// stolen from minifb docs
		let (r, g, b) = (r as u32, g as u32, b as u32);
		(r << 16) | (g << 8) | b
	}

	const WHITE: u32 = from_u8_rgb(255, 255, 255);
	static BLACK: u32 = from_u8_rgb(0, 0, 0);

	let mut buffer: Vec<u32> = vec![WHITE; (SIDE_LENGTH * SIDE_LENGTH) as usize];
	let blank_buffer: Vec<u32> = vec![WHITE; (SIDE_LENGTH * SIDE_LENGTH) as usize];

	fn round(n: f32) -> u32 {
		(n + 0.5).floor() as u32
	}

	static mut INDEX: usize = 0;
	let index_pixel = |x: u32, y: u32| unsafe {
		if INDEX == TOTAL_LINE_LENGTH {
			println!("Error: Points array lacks sufficient space");
		} else {
			LINE_POINTS[INDEX] = [x as i32, y as i32];
			INDEX += 1;
		}
	};

	// bresenham's line drawing algorithm
	let drawline = |mut x0: f32, mut y0: f32, x1: f32, y1: f32| {
		let dx = (x1 - x0).abs();
		let sx = if x0 < x1 { 1.0 } else { -1.0 };
		let dy = -((y1 - y0).abs());
		let sy = if y0 < y1 { 1.0 } else { -1.0 };
		let mut error = dx + dy;

		loop {
			index_pixel(round(x0), round(y0));
			if x0 == x1 && y0 == y1 { break }

			let e2 = 2.0 * error;

			if e2 >= dy {
				if x0 == x1 { break }
				error += dy;
				x0 += sx;
			}

			if e2 <= dx {
				if y0 == y1 { break }
				error += dx;
				y0 += sy;
			}
		}
	};

	let rotate_points = || unsafe {
		for v in CONNECTIONS {
			let start = [POINTS[v[0]][0], POINTS[v[0]][1]];
			let end = [POINTS[v[1]][0], POINTS[v[1]][1]];

			drawline(start[0], start[1], end[0], end[1]);
		}

		rotate_x();
		rotate_y();
		rotate_z();
	};

	loop {
		rotate_points();

		unsafe {
			for i in LINE_POINTS {
				let x = i[0];
				let y = i[1];

				if !(y > SIDE_LENGTH as i32 || y == 0) {
					let buffer_index = ((y - 1) * SIDE_LENGTH as i32 + x - 1) as usize;
					buffer[buffer_index] = BLACK;
					println!("Indexed to buffer: {buffer_index:?}");
				};
			}
			LINE_POINTS = [[0, 0]; TOTAL_LINE_LENGTH];
			INDEX = 0;
		};

		window
			.update_with_buffer(&blank_buffer, SIDE_LENGTH as usize, SIDE_LENGTH as usize)
			.unwrap();

		window
			.update_with_buffer(&buffer, SIDE_LENGTH as usize, SIDE_LENGTH as usize)
			.unwrap();

		if !window.is_open() {
			std::process::exit(69);
		};
	}
}
