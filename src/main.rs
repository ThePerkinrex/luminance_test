use luminance::context::GraphicsContext;
use luminance::framebuffer::{DepthSlot, Framebuffer};
use luminance::pipeline::PipelineState;
use luminance::pixel::{Depth32F, Pixel};
use luminance::texture::{Dim2, Texture};

use luminance_glfw::{
	Action, CursorMode, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt,
};

use cgmath::{EuclideanSpace, InnerSpace, Point3, Quaternion, Rad, Rotation3, Vector3};

use std::path::Path;
use std::process::exit;
use std::time::Instant;

mod engine;
pub mod terrain;

const X_DEFAULT_SIZE: u32 = 1000;
const Y_DEFAULT_SIZE: u32 = 1000;

fn main() {
	//let t = terrain::generate(100,100);
	// println!("{:?} {:?}", Vector3 {
	// 	x: 0.,
	// 	y: 0.,
	// 	z: 0.
	// }, Vector3 {
	// 	x: 0.,
	// 	y: 0.,
	// 	z: 0.
	// } + Vector3::<f32>::unit_x());
	// return;

	let surface = GlfwSurface::new(
		WindowDim::Windowed(X_DEFAULT_SIZE, Y_DEFAULT_SIZE),
		"Hello, world!",
		WindowOpt::default().set_cursor_mode(CursorMode::Disabled),
	);

	let res = match surface {
		Ok(surface) => {
			eprintln!("graphics surface created");
			main_loop(surface);
			0
		}

		Err(e) => {
			eprintln!("cannot create graphics surface:\n{}", e);
			1
		}
	};
	if res == 1 {
		exit(1);
	}
}

fn main_loop(mut surface: GlfwSurface) {
	let mut file_loader = engine::FileLoader::new();
	let mut size = [X_DEFAULT_SIZE, Y_DEFAULT_SIZE];
	let start_t = Instant::now();

	let entity = engine::hud::Entity::load(
		&mut file_loader,
		&mut surface,
		&[
			engine::hud::VertexPosition::new([0, 0]),
			engine::hud::VertexPosition::new([100, 0]),
			engine::hud::VertexPosition::new([100, 200]),
			engine::hud::VertexPosition::new([0, 200]),
		],
		&[0, 1, 2, 0, 2, 3],
		Path::new("texture.ron"),
	)
	.expect("Error creeating entity");

	let font = engine::text::Font::new(
		"Roboto",
		engine::text::FontWeight::Black,
		engine::text::FontStyle::Regular,
		20.,
	);
	// let mut rt_font = engine::text_rusttype::Font::new("Roboto", engine::text_rusttype::FontWeight::Black, engine::text_rusttype::FontStyle::Regular, 20.);
	// font.set_color(engine::RgbaColor::new(255,0,0,170));
	let mut entity2 = engine::hud::Entity::new_entity_from_string(
		&mut file_loader,
		&mut surface,
		"Lies & deception".into(),
		&font,
	)
	.unwrap();

	entity2.set_pos([100, 100]);
	entity2.set_depth(-1.0);

	let mut back_buffer = surface.back_buffer().unwrap();
	let depth_map_size = [1024, 1024];
	let depth_fb: Framebuffer<Dim2, (), Depth32F> =
		Framebuffer::new(&mut surface, depth_map_size, 0, Default::default()).unwrap();
	let depth_e = engine::hud::DepthEntity::new(
		&mut surface,
		&[
			engine::hud::Vertex::new(
				engine::hud::VertexPosition::new([0, 0]),
				engine::hud::VertexUV::new([0, 0]),
			),
			engine::hud::Vertex::new(
				engine::hud::VertexPosition::new([500, 0]),
				engine::hud::VertexUV::new([depth_map_size[0], 0]),
			),
			engine::hud::Vertex::new(
				engine::hud::VertexPosition::new([0, 500]),
				engine::hud::VertexUV::new([0, depth_map_size[1]]),
			),
			engine::hud::Vertex::new(
				engine::hud::VertexPosition::new([500, 500]),
				engine::hud::VertexUV::new(depth_map_size),
			),
		],
		&[0, 1, 2, 1, 2, 3],
		size,
	);

	let pos = [0, 0];

	let mut hud_registry = engine::EntityRegistry::new();
	//hud_registry.register(&"Playeer", entity);
	hud_registry.register(&"Text", entity2);
	// hud_registry.register(&"Shadow", depth_e);

	let renderer = engine::hud::Renderer::new();

	let mut key_registry = engine::KeyRegistry::new();

	let mut spatial_renderer =
		engine::spatial::Renderer::new(&mut file_loader, &mut surface, size, depth_map_size);
	// let mut depth_renderer = engine::spatial::depth::Renderer::new(&mut file_loader, &mut surface, size);

	let mut last_pos = [0.0; 2];
	let mut f = true;

	'app: loop {
		let mut resized = false;
		// handle events
		for event in surface.poll_events() {
			key_registry.event(&event);
			match &event {
				WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
					break 'app
				}
				WindowEvent::Key(Key::K, _, Action::Press, _) => {
					// hud_registry
					// 	.get_mut(&"Playeer")
					// 	.unwrap()
					// 	.set_state("2")
					// 	.expect("Error setting state");
				}
				WindowEvent::CursorPos(x, y) => {
					if !f {
						let move_x = -(*x - last_pos[0]);
						let move_y = *y - last_pos[1];

						// let e = Euler {
						// 	x: Rad(0.005 * move_y).normalize_signed(),
						// 	y: Rad(0.005) * move_x,
						// 	z: Rad(0.),
						// };
						let qy = Quaternion::from_angle_y(Rad(0.005 * move_x))
							.cast()
							.unwrap();
						let qx = Quaternion::from_angle_x(Rad(0.005 * move_y))
							.cast()
							.unwrap();
						spatial_renderer.camera.rot = spatial_renderer.camera.rot * qy * qx;
						spatial_renderer.camera.update_dir();
					// let e: Euler<Rad<f32>> = spatial_renderer.camera.rot.into();
					// dbg!(e, Rad(0.005 * move_x));
					} else {
						f = false;
					}
					last_pos = [*x, *y];
				}
				WindowEvent::FramebufferSize(x, y) => {
					size = [*x as u32, *y as u32];

					resized = true;
				}
				_ => (),
			}
		}

		key_registry.for_pressed_keys(|key| {
			let mut fd_scale = 0.0;
			let mut rt_scale = 0.0;
			let speed = 0.5;
			match key {
				Key::W => {
					//pos[1] += 10;
					fd_scale = 1.;
				}
				Key::S => {
					//pos[1] -= 10;
					fd_scale = -1.;
				}
				Key::A => {
					//pos[0] -= 10;
					rt_scale = -1.;
				}
				Key::D => {
					//pos[0] += 10;
					rt_scale = 1.;
				}
				_ => (),
			};
			let mut fd = spatial_renderer.camera.dir.clone().normalize();
			//fd.y = 0.;
			let mut rt = Vector3::new(-fd.z, 0.0, fd.x);
			fd = fd.normalize_to(fd_scale * speed);
			rt = rt.normalize_to(rt_scale * speed);
			spatial_renderer.camera.pos += fd + rt;
		});
		spatial_renderer.camera.update();

		let light_pos = spatial_renderer.camera.pos + Vector3::new(0.0, 10., 0.);
		spatial_renderer.depth_camera.pos = light_pos;
		spatial_renderer
			.depth_camera
			.look_at(spatial_renderer.mesh.pos);

		if resized {
			back_buffer = surface.back_buffer().unwrap();
		}

		// println!("{:?}", d.size());
		// entity.set_pos(pos.clone());
		// hud_registry
		// 	.get_mut(&"Playeer")
		// 	.unwrap()
		// 	.set_pos(pos.clone());

		// rendering code goes here
		let t = start_t.elapsed().as_millis() as f32 * 1e-3;

		// hud_registry.get_mut(&"Text").unwrap().update_text(&mut surface, &format!("{:.2}", t), &font).expect("Error updating text"); // Dynamic text rendering
		let color = [t.cos(), t.sin(), 0.5, 1.];

		// draw the shadows
		surface.pipeline_builder().pipeline(
			&depth_fb,
			&PipelineState::default(),
			|pipeline, mut shd_gate| {
				// let light_pos = (-1.1,2.,3.).into();
				spatial_renderer.render_depth(&mut shd_gate, &pipeline, &size);
			},
		);

		surface.pipeline_builder().pipeline(
			&back_buffer,
			&PipelineState::default().set_clear_color(color),
			|pipeline, mut shd_gate| {
				spatial_renderer.render(&mut shd_gate, &pipeline, &size);
				// Render the HUD last
				renderer.render(
					&hud_registry,
					&mut shd_gate,
					&pipeline,
					&size,
					&depth_e,
					depth_fb.depth_slot(),
				);
			},
		);

		// let tex = engine::depth_texture_to_color(&mut surface, );
		// hud_registry
		// 	.get_mut(&"Shadow")
		// 	.unwrap()
		// 	.update_tex(*(depth_fb.depth_slot().clone()));

		// swap buffer chains
		surface.swap_buffers();
	}
}
