use glow::*;
use sdl2::{event::WindowEvent, video::SwapInterval};

fn main() {
    unsafe {
        let (gl, window, mut events_loop, _context) = create_window("Falling Sand".to_string(), 1280, 720);

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

        let vbo = gl.create_buffer().unwrap();
        let vao = gl.create_vertex_array().unwrap();

        gl.bind_vertex_array(Some(vao));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
        gl.enable_vertex_attrib_array(0);

        // set window dimension uniform
        let dimensions_loc = gl.get_uniform_location(program, "dimensions");
        gl.uniform_2_f32(dimensions_loc.as_ref(), window.size().0 as f32, window.size().1 as f32);

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        let mut last_frame = std::time::Instant::now();
        'render: loop {
            {
                for event in events_loop.poll_iter() {
                    if let sdl2::event::Event::Quit { .. } = event {
                        break 'render;
                    }
                    if let sdl2::event::Event::Window { win_event, .. } = event {
                        match win_event {
                            WindowEvent::Resized(width, height) => {
                                gl.viewport(0, 0, width, height);
                                gl.uniform_2_f32(dimensions_loc.as_ref(), width as f32, height as f32);
                            }
                            _ => {}
                        };
                    }
                }
            }
            println!("DT: {:?}", 1. / ((last_frame.elapsed().subsec_micros() as f32) * 0.000001));
            last_frame = std::time::Instant::now();
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
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_flags().forward_compatible().set();
    let window = video
        .window(&title, width, height)
        .opengl()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl =
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
    video.gl_set_swap_interval(SwapInterval::Immediate).unwrap();
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
