use glow::*;
use particle::Particle;
use sdl2::event::WindowEvent;

mod particle;
mod macros;

const CELL_SIZE: u32 = 1;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

const MATRIX_ROWS: u32 = HEIGHT / CELL_SIZE;
const MATRIX_COLS: u32 = WIDTH / CELL_SIZE;

fn main() {
    unsafe {
        let (gl, window, mut events_loop, _context) = create_window("Falling Sand".to_string(), WIDTH, HEIGHT);

        /*let vertices = [
             0.5,  0.5, 0.0,
             0.5, -0.5, 0.0,
            -0.5,  0.5, 0.0,
            -0.5, -0.5, 0.0f32,
             0.5, -0.5, 0.0,
            -0.5,  0.5, 0.0,
        ];*/
        let vertices = [
             1.0,  1.0, 0.0,
             1.0, -1.0, 0.0,
            -1.0,  1.0, 0.0,
            -1.0, -1.0, 0.0f32,
             1.0, -1.0, 0.0,
            -1.0,  1.0, 0.0,
        ];
        let vertices_u8: &[u8] = core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<f32>(),
        );

        let vertex_shader = std::fs::read_to_string(std::path::Path::new("shaders/vertex.glsl"))
            .expect("Could not read vertex shader");
        let fragment_shader = std::fs::read_to_string(std::path::Path::new("shaders/fragment.glsl"))
            .expect("Could not read fragment shader");

        let program = create_program(&gl, &vertex_shader, &fragment_shader);
        gl.use_program(Some(program));

        let simulate_shader_src = std::fs::read_to_string(std::path::Path::new("shaders/simulation.glsl"))
            .expect("Could not read simulation shader");

        let simulate_program = gl.create_program().expect("Could not create program");
        let simulate_shader = gl.create_shader(glow::COMPUTE_SHADER).expect("Could not create simulate shader");
        gl.shader_source(simulate_shader, &simulate_shader_src);
        gl.compile_shader(simulate_shader);
        if !gl.get_shader_compile_status(simulate_shader) {
            panic!("{}", gl.get_shader_info_log(simulate_shader));
        }
        gl.attach_shader(simulate_program, simulate_shader);
        gl.link_program(simulate_program);

        let vbo = gl.create_buffer().unwrap();
        let vao = gl.create_vertex_array().unwrap();

        gl.bind_vertex_array(Some(vao));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
        gl.enable_vertex_attrib_array(0);

        // init simulation logic
        let mut matrix = vec![Particle::Blank; (MATRIX_COLS * MATRIX_ROWS) as usize];
        /*for i in 0..9 {
            matrix[index_matrix!(i+6, 41)] = Particle::Stone;
        }
        matrix[index_matrix!(6, 40)] = Particle::Stone;
        matrix[index_matrix!(14, 40)] = Particle::Stone;
        for i in 0..30 {
            matrix[index_matrix!(10, i*2)] = Particle::Sand;
        }*/

        let matrix_buffer_1 = gl.create_buffer().unwrap();
        let matrix_buffer_2 = gl.create_buffer().unwrap();
        let matrix_data: Vec<u32> = matrix.iter().map(|particle| particle.to_id() as u32).collect::<Vec<_>>();
        let matrix_data_copy: Vec<u32> = matrix_data.clone();
        let matrix_data: &[u8] = core::slice::from_raw_parts(
            matrix_data.as_ptr() as *const u8,
            matrix_data.len() * core::mem::size_of::<u32>(),
        );
        let matrix_data_copy: &[u8] = core::slice::from_raw_parts(
            matrix_data_copy.as_ptr() as *const u8,
            matrix_data_copy.len() * core::mem::size_of::<u32>(),
        );
        gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(matrix_buffer_1));
        gl.buffer_data_u8_slice(glow::SHADER_STORAGE_BUFFER, matrix_data, glow::DYNAMIC_COPY);
        gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(matrix_buffer_2));
        gl.buffer_data_u8_slice(glow::SHADER_STORAGE_BUFFER, matrix_data_copy, glow::STATIC_COPY);

        gl.bind_buffer_base(glow::SHADER_STORAGE_BUFFER, 2, Some(matrix_buffer_1));
        gl.bind_buffer_base(glow::SHADER_STORAGE_BUFFER, 3, Some(matrix_buffer_2));
        // end

        // set window dimension uniform
        let dimensions_loc = gl.get_uniform_location(program, "dimensions");
        gl.uniform_2_f32(dimensions_loc.as_ref(), window.size().0 as f32, window.size().1 as f32);
        gl.use_program(Some(simulate_program));
        let simulate_dimensions_loc = gl.get_uniform_location(simulate_program, "dimensions");
        gl.uniform_2_f32(simulate_dimensions_loc.as_ref(), window.size().0 as f32, window.size().1 as f32);

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        let start = std::time::Instant::now();
        let mut frame_count = 0;
        let mut pressed = false;
        let mut mouse_x = 0;
        let mut mouse_y = 0;
        'render: loop {
            {
                for event in events_loop.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => {
                            break 'render;
                        }
                        sdl2::event::Event::Window { win_event, .. } => {
                            match win_event {
                                WindowEvent::Resized(width, height) => {
                                    gl.viewport(0, 0, width, height);
                                    gl.uniform_2_f32(dimensions_loc.as_ref(), width as f32, height as f32);
                                    gl.uniform_2_f32(simulate_dimensions_loc.as_ref(), width as f32, height as f32);
                                }
                                _ => {}
                            };
                        }
                        sdl2::event::Event::MouseMotion { mousestate, x, y, .. } => {
                            if mousestate.left() {
                                mouse_x = x;
                                mouse_y = y;
                            }
                        }
                        sdl2::event::Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                            if mouse_btn == sdl2::mouse::MouseButton::Left {
                                pressed = true;
                                mouse_x = x;
                                mouse_y = y;
                            }
                        }
                        sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
                            if mouse_btn == sdl2::mouse::MouseButton::Left {
                                pressed = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            if pressed {
                let x = (mouse_x as f32 / CELL_SIZE as f32).floor();
                let y = (mouse_y as f32 / CELL_SIZE as f32).floor();
                let i = ((y * MATRIX_COLS as f32) + x).floor() as usize;
                gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(matrix_buffer_1));
                gl.buffer_sub_data_u8_slice(glow::SHADER_STORAGE_BUFFER, (i*4) as i32, &[1]);
            }
            println!("DT: {:?}", 1. / ((start.elapsed().as_micros() as f32 * 0.000001) / frame_count as f32));
            frame_count += 1;

            gl.use_program(Some(simulate_program));

            gl.dispatch_compute(MATRIX_COLS, MATRIX_ROWS, 1);

            let matrix_data: &mut [u8] = &mut [0u8; (MATRIX_COLS * MATRIX_ROWS * 4) as usize];
            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(matrix_buffer_1));
            gl.get_buffer_sub_data(glow::SHADER_STORAGE_BUFFER, 0, matrix_data);

            gl.bind_buffer(glow::SHADER_STORAGE_BUFFER, Some(matrix_buffer_2));
            gl.buffer_data_u8_slice(glow::SHADER_STORAGE_BUFFER, matrix_data, glow::STATIC_COPY);
            gl.bind_buffer_base(glow::SHADER_STORAGE_BUFFER, 3, Some(matrix_buffer_2));

            gl.use_program(Some(program));
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
            window.gl_swap_window();
        }
        gl.delete_program(program);
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
    }
}

unsafe fn create_window(title: String, width: u32, height: u32) -> (
    glow::Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 3);
    gl_attr.set_context_flags().forward_compatible().set();
    let window = video
        .window(&title, width, height)
        .opengl()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl =
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
    video.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate).unwrap();
    let event_loop = sdl.event_pump().unwrap();
    (gl, window, event_loop, gl_context)
}

unsafe fn create_program(
    gl: &glow::Context,
    vertex_shader: &str,
    fragment_shader: &str,
) -> NativeProgram {
    let program = gl.create_program().expect("Could not create program");

    let shader_sources = [
        (glow::VERTEX_SHADER, vertex_shader),
        (glow::FRAGMENT_SHADER, fragment_shader),
    ];

    let mut shaders = Vec::with_capacity(shader_sources.len());

    for (shader_type, shader_source) in shader_sources.iter() {
        let shader = gl
            .create_shader(*shader_type)
            .expect("Could not create shader");
        gl.shader_source(shader, &shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        gl.attach_shader(program, shader);
        shaders.push(shader);
    }

    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }

    for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
    }

    program
}
