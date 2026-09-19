use binrw::BinRead;
use std::{env, fs::File, time::Instant};
use three_d::*;

use tet_visulizer::{data::TetGraph, *};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("File Path not provided");

    let mut file = File::open(filename).expect("Unable to Open File");
    let stl_file = StlFile::read(&mut file).expect("Unable to Parse Stl File");

    let tet_mesh = gen_tet(stl_file).expect("Error Occured while generating tets");

    let start = Instant::now();
    let tet_graph = TetGraph::new(&tet_mesh);
    let duration = start.elapsed();
    println!(
        "Time elapsed in your_expensive_function() is: {:?}",
        duration
    );

    let cpu_mesh = tet_graph._render_neighbor_edges();

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
        100000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 1000.0);

    let model = Gm::new(
        InstancedMesh::new(&context, &cpu_mesh, &CpuMesh::cylinder(8)),
        ColorMaterial::default(),
    ); //Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    // Big enough to comfortably span the model from any plane orientation.
    let _plane_size = 64.;

    let mut planes: Vec<ClipPlane> = Vec::new();
    let mut _applied_planes: Option<Vec<ClipPlane>> = None;
    let mut _plane_models: Vec<Gm<Mesh, ColorMaterial>> = Vec::new();

    let mut gui = GUI::new(&context);

    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                egui::Panel::left("clip_planes_panel").show_inside(gui_context, |ui| {
                    ui.heading("Clipping Planes");
                    ui.label("Drag the offset to slide a plane along its normal.");
                    ui.label("Drag the angles to rotate it.");
                    ui.separator();

                    let mut remove_index: Option<usize> = None;
                    for (i, plane) in planes.iter_mut().enumerate() {
                        ui.push_id(i, |ui| {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut plane.enabled, format!("Plane {}", i + 1));
                                    if ui.button("Remove").clicked() {
                                        remove_index = Some(i);
                                    }
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Offset");
                                    ui.add(egui::DragValue::new(&mut plane.offset).speed(0.01));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Azimuth");
                                    ui.add(
                                        egui::DragValue::new(&mut plane.azimuth_deg)
                                            .speed(1.0)
                                            .suffix("\u{b0}"),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Elevation");
                                    ui.add(
                                        egui::DragValue::new(&mut plane.elevation_deg)
                                            .speed(1.0)
                                            .range(-90.0..=90.0)
                                            .suffix("\u{b0}"),
                                    );
                                });
                            });
                        });
                    }
                    if let Some(i) = remove_index {
                        planes.remove(i);
                    }

                    ui.separator();
                    if ui.button("Add plane").clicked() {
                        planes.push(ClipPlane::default());
                    }
                });
                panel_width = gui_context.globally_used_rect().width();
            },
        );

        let panel_width_physical = (panel_width * frame_input.device_pixel_ratio) as u32;
        let viewport = Viewport {
            x: panel_width_physical as i32,
            y: 0,
            width: frame_input
                .viewport
                .width
                .saturating_sub(panel_width_physical),
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
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
            .render(
                &camera,
                std::iter::once(&model), //.chain(plane_models.iter()),
                &[],
            )
            .write(|| gui.render())
            .unwrap();

        FrameOutput::default()
    });
}
