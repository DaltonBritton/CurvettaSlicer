use three_d::*;

use tet_visulizer::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Tet Visualizer".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(3.0, 2.5, 4.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 20.0);

    let tet_mesh = gen_tet().unwrap();
    let cpu_mesh = tet_to_mesh(tet_mesh);
    let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        for event in frame_input.events.iter_mut() {
            if let Event::MouseMotion {
                delta,
                button,
                handled,
                ..
            } = event
            {
                if !*handled && *button == Some(MouseButton::Right) {
                    let distance = camera.position().distance(camera.target());
                    let speed = distance * 0.0015;
                    let right = camera.right_direction();
                    let up = right.cross(camera.view_direction());
                    let translation = -right * delta.0 * speed + up * delta.1 * speed;

                    camera.translate(translation);
                    control.target += translation;

                    *handled = true;
                }
            }
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.85, 0.85, 0.85, 1.0, 1.0))
            .render(&camera, &model, &[]);

        FrameOutput::default()
    });
}
