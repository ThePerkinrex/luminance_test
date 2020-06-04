use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;

use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};

use std::path::Path;
use std::process::exit;
use std::time::Instant;

mod engine;

const X_DEFAULT_SIZE: u32 = 960;
const Y_DEFAULT_SIZE: u32 = 540;

fn main() {
	let surface = GlfwSurface::new(
		WindowDim::Windowed(X_DEFAULT_SIZE, Y_DEFAULT_SIZE),
		"Hello, world!",
		WindowOpt::default(),
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
	let mut size = [X_DEFAULT_SIZE, Y_DEFAULT_SIZE];
	let start_t = Instant::now();

	let mut entity = engine::hud::Entity::load(&mut surface, &[
		engine::hud::VertexPosition::new([0, 0]),
		engine::hud::VertexPosition::new([100, 0]),
		engine::hud::VertexPosition::new([100, 200]),
		engine::hud::VertexPosition::new([0, 200]),
	], &[0,1,2, 0,2,3], Path::new("texture.ron")).expect("Error creeating entity");

	let font = engine::text::Font::new("Roboto", engine::text::FontWeight::Thin, engine::text::FontStyle::Regular);
	let mut entity2 = engine::hud::Entity::new_entity_from_string(
		&mut surface,
		"Lies & deception".into(),
		font,
	)
	.unwrap();

	entity2.set_pos([100, 100]);
	entity2.set_depth(-1.0);


	let mut back_buffer = surface.back_buffer().unwrap();

	let mut pos = [0, 0];

	let mut hud_registry = engine::EntityRegistry::new();
	hud_registry.register(&"Playeer", entity);
	hud_registry.register(&"Text", entity2);

	let renderer = engine::hud::Renderer::new();

	'app: loop {
		let mut resized = false;
		// handle events
		for event in surface.poll_events() {
			match event {
				WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
					break 'app
				}
				WindowEvent::Key(Key::W, _, Action::Press, _) => pos[1] += 10,
				WindowEvent::Key(Key::S, _, Action::Press, _) => pos[1] -= 10,
				WindowEvent::Key(Key::A, _, Action::Press, _) => pos[0] -= 10,
				WindowEvent::Key(Key::D, _, Action::Press, _) => pos[0] += 10,
				WindowEvent::Key(Key::W, _, Action::Repeat, _) => pos[1] += 10,
				WindowEvent::Key(Key::S, _, Action::Repeat, _) => pos[1] -= 10,
				WindowEvent::Key(Key::A, _, Action::Repeat, _) => pos[0] -= 10,
				WindowEvent::Key(Key::D, _, Action::Repeat, _) => pos[0] += 10,
				WindowEvent::Key(Key::K, _, Action::Press, _) => {
					hud_registry.get_mut(&"Playeer").unwrap().set_state("2").expect("Error setting state");
				}
				WindowEvent::FramebufferSize(x, y) => {
					size = [x as u32, y as u32];

					resized = true;
				}
				_ => (),
			}
		}

		if resized {
			back_buffer = surface.back_buffer().unwrap();
		}
		// entity.set_pos(pos.clone());

		// rendering code goes here
		let t = start_t.elapsed().as_millis() as f32 * 1e-3;
		let color = [t.cos(), t.sin(), 0.5, 1.];

		surface.pipeline_builder().pipeline(
			&back_buffer,
			&PipelineState::default().set_clear_color(color),
			|pipeline, mut shd_gate| {
				/*shd_gate.shade(&hud_program, |iface, mut rdr_gate| {
					rdr_gate.render(render_st, |mut tess_gate| {
						//tess_gate.render(triangle.slice(..))
						entity.render(&pipeline, &iface, &mut tess_gate, &size);
						entity2.render(&pipeline, &iface, &mut tess_gate, &size);
					})
				});*/
				renderer.render(&hud_registry, &mut shd_gate, &pipeline, &size);
			},
		);

		// swap buffer chains
		surface.swap_buffers();
	}
}
