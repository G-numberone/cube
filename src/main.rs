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
		[600.0, 600.0, -100.0]  // H
	];
	const CONNECTIONS: [[usize; 2]; 12] = [
		[0, 1], [0, 3],
		[1, 2], [2, 3],
		[1, 4], [0, 5],
		[2, 6], [3, 7],
		[5, 4], [5, 7],
		[4, 6], [6, 7]
	];

	let theta: f32 = 0.01;
	let sine_theta: f32 = theta.sin();
	let cosine_theta: f32 = theta.cos();

	let rotate_x = || unsafe {
		for i in 0 .. POINTS.len() {
			POINTS[i] = [
				POINTS[i][0],
				POINTS[i][1] * cosine_theta + POINTS[i][2] * -sine_theta,
				POINTS[i][1] * sine_theta + POINTS[i][2] * cosine_theta
			];
		}
	};
	let rotate_y = || unsafe {
		for i in 0 .. POINTS.len() {
			POINTS[i] = [
				POINTS[i][0] * cosine_theta + POINTS[i][2] * sine_theta,
				POINTS[i][1],
				POINTS[i][0] * -sine_theta + POINTS[i][2] * cosine_theta
			];
		}
	};
	let rotate_z = || unsafe {
		for i in 0 .. POINTS.len() {
			POINTS[i] = [
				POINTS[i][0] * cosine_theta + POINTS[i][1] * -sine_theta,
				POINTS[i][0] * sine_theta + POINTS[i][1] * cosine_theta,
				POINTS[i][2]
			];
		}
	};

	const SIDE_LENGTH: u32 = 800;
	let mut window = Window::new(
		"cube spinning 😱",
		SIDE_LENGTH as usize,
		SIDE_LENGTH as usize,
		WindowOptions::default()
	).expect("something happened :(");
	window.limit_update_rate(Some(std::time::Duration::from_millis(10)));

	const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
		let (r, g, b) = (r as u32, g as u32, b as u32);
		(r << 16) | (g << 8) | b
	}

	const WHITE: u32 = from_u8_rgb(255, 255, 255);
	static BLACK: u32 = from_u8_rgb(0, 0, 0);

	let mut buffer: Vec<u32> = vec![WHITE; ( SIDE_LENGTH * SIDE_LENGTH) as usize];

	fn round(n: f32) -> u32 {
		return (n + 0.5).floor() as u32
	}

	let mut set_pixel_in_buffer = |x: u32, y: u32| {
		let index = || {
			return if y > 800 {
				0
			} else if y != 0 {
				println!("Success!\n{0} : {1}", x, y);
				((y - 1) * SIDE_LENGTH + x - 1) as usize
			} else {
				x as usize
			};
		};

		if index() >= 640000 && index() == 0 {
			println!("out of bounds ???\n{0}, {1}, {2}", index(), x, y)
		} else {
			buffer[index()] = BLACK;
		};
	};

	let mut drawline = |x0: f32, y0: f32, x1: f32, y1: f32| {
		let dx = x1 - x0;
		let dy = y1 - y0;
		let mut x = x0;
		let mut y = y0;
		let mut p = 2.0 * dy - dx;

		while x < x1 {
			if p >= 0.0 {
				set_pixel_in_buffer(
					round(x),
					round(y)
				);
				y += 1.0;
				p += 2.0 * dy - 2.0 * dx;
			} else {
				set_pixel_in_buffer(
					round(x),
					round(y)
				);
				p += 2.0 * dy;
			}
			x += 1.0;
		};
	};

	let mut rotate_points = || unsafe {
		for v in CONNECTIONS {
			let start = [
				POINTS[v[0]][0],
				POINTS[v[0]][1]
			];
			let end = [
				POINTS[v[1]][0],
				POINTS[v[1]][1]
			];

			drawline(
				start[0],
				start[1],
				end[0],
				end[1]
			);
		}

		rotate_x();
		rotate_y();
		rotate_z();
	};

	loop {
		rotate_points();

		window.update_with_buffer(
			&buffer,	// cannot borrow `buffer` as immutable because it is also borrowed as mutable, immutable borrow occurs here
			SIDE_LENGTH as usize,
			SIDE_LENGTH as usize
		).unwrap();

		if !window.is_open() {
			std::process::exit(69);
		}
	}
}
