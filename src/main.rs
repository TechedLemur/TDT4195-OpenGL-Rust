extern crate nalgebra_glm as glm;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr};

mod mesh;
mod scene_graph;
mod shader;
mod util;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;
use scene_graph::SceneNode;

const SCREEN_W: u32 = 1600;
const SCREEN_H: u32 = 900;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

fn task1_triangles() -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    // Have separate objects for each triangle to make it easier to read and modify
    let triangle1: Vec<f32> = vec![-0.2, -0.2, 0.5, 0.6, -0.1, 0.5, 0.0, 0.5, 0.5];
    let triangle2: Vec<f32> = vec![-0.5, -0.6, 0.3, 0.5, -0.6, 0.3, 0.0, 0.2, 0.3];
    let triangle3: Vec<f32> = vec![-0.0, 0.0, 0.0, 0.4, -0.5, 0.1, -0.3, 0.6, 0.1];
    // let triangle4: Vec<f32> = vec![0.7, -0.5, 0.0, 0.9, -0.3, 0.0, 0.7, -0.1, 0.0];
    // let triangle5: Vec<f32> = vec![0.25, 0.25, 0.0, 0.95, 0.7, 0.0, 0.3, 0.8, 0.0];
    // let triangle6: Vec<f32> = vec![0.0, -0.6, 0.0, 0.5, -0.7, 0.0, 0.3, -0.5, 0.0];
    let mut coordinates_task1: Vec<f32> = Vec::new();
    coordinates_task1.extend(&triangle1);
    coordinates_task1.extend(&triangle2);
    coordinates_task1.extend(&triangle3);
    // coordinates_task1.extend(&triangle4);
    // coordinates_task1.extend(&triangle5);
    // coordinates_task1.extend(&triangle6);

    let indices: Vec<u32> = vec![6, 7, 8, 3, 4, 5, 0, 1, 2, 0, 2, 1, 3, 5, 4, 6, 8, 7];

    let red: Vec<f32> = vec![1.0, 0.0, 0.0, 1.0];
    let green: Vec<f32> = vec![0.0, 1.0, 0.0, 1.0];
    let blue: Vec<f32> = vec![0.0, 0.0, 1.0, 1.0];

    let mut colors: Vec<f32> = Vec::new();
    //Triangle 1
    colors.extend(vec![0.0, 0.0, 1.0, 0.6]);
    colors.extend(vec![1.0, 0.0, 0.0, 0.6]);
    colors.extend(vec![0.0, 1.0, 1.0, 0.6]);
    colors.extend(vec![1.0, 0.0, 1.0, 0.5]);
    colors.extend(vec![0.0, 1.0, 0.0, 0.5]);
    colors.extend(vec![0.5, 1.0, 0.0, 0.5]);
    //Triangle 2
    colors.extend(vec![1.0, 1.0, 1.0, 0.7]);
    colors.extend(vec![1.0, 0.5, 1.0, 0.7]);
    colors.extend(vec![0.4, 0.8, 0.2, 0.7]);
    //Triangle 3

    return (coordinates_task1, indices, colors);
}

fn task2_triangle() -> (Vec<f32>, Vec<u32>) {
    let triangle_task_2a: Vec<f32> = vec![0.6, -0.8, -1.2, 0.0, 0.4, 0.0, -0.8, -0.2, 1.2];
    let indices: Vec<u32> = vec![0, 1, 2];
    return (triangle_task_2a, indices);
}

fn task2b_triangles() -> (Vec<f32>, Vec<u32>) {
    // Have separate objects for each triangle to make it easier to read and modify
    let triangle1: Vec<f32> = vec![-0.2, -0.2, 0.0, 0.2, -0.1, 0.0, 0.0, 0.5, 0.0];
    let triangle2: Vec<f32> = vec![-0.9, -0.6, 0.0, -0.5, -0.6, 0.0, -0.7, 0.2, 0.0];

    let mut coordinates_task1: Vec<f32> = Vec::new();
    coordinates_task1.extend(&triangle1);
    coordinates_task1.extend(&triangle2);

    let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];

    return (coordinates_task1, indices);
}

fn circle_coordinates(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    resolution: u32,
) -> (Vec<f32>, Vec<u32>) {
    let mut coordinates = vec![x, y, z];
    let mut indices: Vec<u32> = vec![0];

    for i in 0..resolution + 1 {
        let t = -2.0 * std::f32::consts::PI * (i as f32) / (resolution as f32);
        let x1 = x + radius * f32::sin(t);
        let y2 = y + radius * f32::cos(t);

        indices.push((i + 1) as u32);
        coordinates.extend(vec![x1, y2, 0.0]);
    }
    return (coordinates, indices);
}

fn sine_function(min: f32, max: f32, resolution: u32) -> (Vec<f32>, Vec<u32>) {
    let mut coordinates = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let len = max - min;
    for i in 0..resolution {
        let x = min + (i as f32) * len / (resolution as f32);
        let y = 0.7 * f32::sin(20.0 * (i as f32) * len / (resolution as f32));
        coordinates.extend(vec![x, y, 0.0]);
        indices.push(i)
    }
    return (coordinates, indices);
}

// == // Modify and complete the function below for the first task
unsafe fn set_up_vao(
    coordinates: &Vec<f32>,
    indices: &Vec<u32>,
    colors: &Vec<f32>,
    normals: &Vec<f32>,
) -> u32 {
    let mut arrayID: u32 = 0;
    gl::GenVertexArrays(1, &mut arrayID as *mut u32);
    gl::BindVertexArray(arrayID);

    let mut bufferIDs: u32 = 0;
    gl::GenBuffers(1, &mut bufferIDs as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, bufferIDs);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(coordinates),
        pointer_to_array(coordinates),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12, ptr::null());
    gl::EnableVertexAttribArray(0);
    // Set up color buffer
    let mut color_bufferIds: u32 = 0;
    gl::GenBuffers(1, &mut color_bufferIds as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, color_bufferIds);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(colors),
        pointer_to_array(colors),
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 16, ptr::null());
    gl::EnableVertexAttribArray(1);
    // Set up normal buffer
    let mut normal_bufferIds: u32 = 0;
    gl::GenBuffers(2, &mut normal_bufferIds as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, normal_bufferIds);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        pointer_to_array(normals),
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 12, ptr::null());
    gl::EnableVertexAttribArray(2);
    let mut index_bufferIds: u32 = 0;
    gl::GenBuffers(1, &mut index_bufferIds as *mut u32);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_bufferIds);

    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );
    return arrayID;
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!(
                "{}: {}",
                util::get_gl_string(gl::VENDOR),
                util::get_gl_string(gl::RENDERER)
            );
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!(
                "GLSL\t: {}",
                util::get_gl_string(gl::SHADING_LANGUAGE_VERSION)
            );
        }

        // Use ShaderBuilder to load and link shaders
        let shader_pair;
        unsafe {
            shader_pair = shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link();
            shader_pair.activate(); // activate shaders
        }
        // == // Set up your VAO here
        let surface_vao_id: u32;
        let body_vao_id: u32;
        let door_vao_id: u32;
        let main_rotor_vao_id: u32;
        let tail_rotor_vao_id: u32;

        let mut surface_node;
        let mut body_node;
        let mut door_node;
        let mut main_rotor_node;
        let mut tail_rotor_node;

        let terrain = mesh::Terrain::load("./resources/lunarsurface.obj");
        let helicopter = mesh::Helicopter::load("./resources/helicopter.obj");

        unsafe {
            // set up the vertex array objects
            surface_vao_id = set_up_vao(
                &terrain.vertices,
                &terrain.indices,
                &terrain.colors,
                &terrain.normals,
            );
            body_vao_id = set_up_vao(
                &helicopter.body.vertices,
                &helicopter.body.indices,
                &helicopter.body.colors,
                &helicopter.body.normals,
            );
            door_vao_id = set_up_vao(
                &helicopter.door.vertices,
                &helicopter.door.indices,
                &helicopter.door.colors,
                &helicopter.door.normals,
            );
            main_rotor_vao_id = set_up_vao(
                &helicopter.main_rotor.vertices,
                &helicopter.main_rotor.indices,
                &helicopter.main_rotor.colors,
                &helicopter.main_rotor.normals,
            );
            tail_rotor_vao_id = set_up_vao(
                &helicopter.tail_rotor.vertices,
                &helicopter.tail_rotor.indices,
                &helicopter.tail_rotor.colors,
                &helicopter.tail_rotor.normals,
            );
        }

        surface_node = SceneNode::from_vao(surface_vao_id, terrain.index_count);
        body_node = SceneNode::from_vao(body_vao_id, helicopter.body.index_count);
        door_node = SceneNode::from_vao(door_vao_id, helicopter.door.index_count);
        main_rotor_node = SceneNode::from_vao(main_rotor_vao_id, helicopter.main_rotor.index_count);
        tail_rotor_node = SceneNode::from_vao(tail_rotor_vao_id, helicopter.tail_rotor.index_count);

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;
        let mut pitch: f32 = 0.0;
        let mut yaw: f32 = 0.0;

        let mut speed = 70.0;
        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::A => {
                            z += speed * delta_time * yaw.sin();
                            x += speed * delta_time * yaw.cos();
                        }
                        VirtualKeyCode::D => {
                            z -= speed * delta_time * yaw.sin();
                            x -= speed * delta_time * yaw.cos();
                        }
                        VirtualKeyCode::W => {
                            z += speed * delta_time * yaw.cos() * pitch.cos();
                            x += speed * delta_time * -yaw.sin() * pitch.cos();
                            y += speed * delta_time * pitch.sin();
                        }
                        VirtualKeyCode::S => {
                            z -= speed * delta_time * yaw.cos();
                            x -= speed * delta_time * -yaw.sin();
                            y -= speed * delta_time * pitch.sin();
                        }
                        VirtualKeyCode::Q => {
                            y += speed * delta_time;
                        }
                        VirtualKeyCode::E => {
                            y -= speed * delta_time;
                        }
                        VirtualKeyCode::Up => {
                            if pitch > -1.5 {
                                pitch -= 1.5 * delta_time;
                            }
                        }
                        VirtualKeyCode::Down => {
                            if pitch < 1.5 {
                                pitch += 1.5 * delta_time;
                            }
                        }
                        VirtualKeyCode::Left => {
                            yaw -= 1.5 * delta_time;
                        }
                        VirtualKeyCode::Right => {
                            yaw += 1.5 * delta_time;
                        }
                        VirtualKeyCode::Space => {
                            speed += 20.0;
                        }
                        VirtualKeyCode::LControl => {
                            speed -= if speed > 20.0 { 20.0 } else { 0.0 };
                        }

                        _ => {}
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                *delta = (0.0, 0.0);
            }

            unsafe {
                // gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::ClearColor(0.6, 0.71372549, 0.94901961, 0.7);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                //let scaling: glm::Mat4 = glm::scaling(&glm::vec3(1.0, 1.0, -20.0));
                let translation: glm::Mat4 = glm::translation(&glm::vec3(x, y, z - 5.0));
                let perspective: glm::Mat4 =
                    glm::perspective(SCREEN_W as f32 / SCREEN_H as f32, 1.0, 1.0, 1000.0);

                let mut rotation: glm::Mat4 = glm::identity();

                rotation =
                    glm::rotation(pitch, &(&rotation * &glm::vec4(1.0, 0.0, 0.0, 1.0)).xyz())
                        * rotation;
                rotation = glm::rotation(yaw, &(&rotation * &glm::vec4(0.0, 1.0, 0.0, 1.0)).xyz())
                    * rotation;
                // let pitch_rotation: glm::Mat4 = glm::rotation(pitch, &glm::vec3(1.0, 0.0, 0.0));
                // let yaw_rotation: glm::Mat4 = glm::rotation(yaw, &glm::vec3(0.0, 1.0, 0.0));

                let matrix: glm::Mat4 = perspective * rotation * translation;
                let location = shader_pair.get_uniform_location("matrix");
                gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
                // Draw elements
                gl::BindVertexArray(surface_vao_id);

                gl::DrawElements(
                    gl::TRIANGLES,
                    terrain.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
                gl::BindVertexArray(body_vao_id);

                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.body.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
                gl::BindVertexArray(door_vao_id);

                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.door.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
                gl::BindVertexArray(main_rotor_vao_id);

                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.main_rotor.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

                gl::BindVertexArray(tail_rotor_vao_id);

                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.tail_rotor.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}
