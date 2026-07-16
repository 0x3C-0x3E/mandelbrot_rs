use macroquad::prelude::*;

use macroquad::ui::hash;
use macroquad::ui::root_ui;

use num::{Complex, complex::ComplexFloat};

fn window_conf() -> Conf {
    Conf {
        window_title: "Function Plotter".to_owned(),
        window_width: 600.0 as i32,
        window_height: 600.0 as i32,
        window_resizable: true,
        sample_count: 1,
        ..Default::default()
    }
}

#[allow(unused)]
fn function(c: Complex<f32>) -> f32 {
    ((5.0_f32 * c).powf(5.0) + (8.0_f32 * c).powf(4.0) + (6.0_f32 * c).powf(2.0)).re()
}

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

const FRAGMENT_SHADER: &str = include_str!("frag.glsl");

#[macroquad::main(window_conf())]
async fn main() {
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);

    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("zoom", UniformType::Float1),
                UniformDesc::new("pos", UniformType::Float2),
                UniformDesc::new("iterations", UniformType::Int1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut zoom: f32 = 2.0_f32;
    let mut pos: (f32, f32) = (0.0, 0.0);

    let mut hide_ui = false;

    let mut iterations: i32 = 2_000_i32;

    loop {
        clear_background(BLACK);

        if get_last_key_pressed().is_some() {
            hide_ui = false;
        }

        let wheel = mouse_wheel().1;

        if wheel != 0.0 {
            let old_zoom = zoom;
            zoom *= 1.1_f32.powf(-wheel);

            let (mx, my) = mouse_position();

            let uv_x = (mx - 0.5 * screen_width()) / screen_height();
            let uv_y = -(my - 0.5 * screen_height()) / screen_height();

            pos.0 += uv_x * (zoom - old_zoom);
            pos.1 += uv_y * (zoom - old_zoom);
        }

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("zoom", zoom);
        material.set_uniform("pos", pos);
        material.set_uniform("iterations", iterations);

        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        gl_use_default_material();

        set_default_filter_mode(FilterMode::Nearest);

        if !hide_ui {
            root_ui().window(
                hash!(),
                Vec2::new(20.0, 20.0),
                Vec2::new(200.0, 200.0),
                |ui| {
                    ui.label(None, &format!("FPS: {}", get_fps()));

                    let (mouse_x, mouse_y) = mouse_position();
                    ui.label(None, &format!("Mouse Position: {mouse_x}, {mouse_y}"));
                    ui.label(None, &format!("Zoom: {zoom}"));
                    ui.label(None, &format!("Iterations: {iterations}"));
                    // ui.slider(
                    //     hash!(),
                    //     &format!("iterations: {iterations}"),
                    //     0..10000,
                    //     data,
                    // );

                    if ui.button(None, "Hide Ui") {
                        hide_ui = true;
                    }

                    if ui.button(None, "Reset") {
                        pos = (0.0, 0.0);
                        zoom = 2.0;
                    }
                },
            );
        }

        next_frame().await
    }
}
