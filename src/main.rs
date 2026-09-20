use binrw::BinRead;
use std::{env, fs::File, time::Instant};
use three_d::*;

use tet_visulizer::{data::TetGraph, *};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RenderMode {
    FullTets,
    TetGraphNodes,
    TetGraphEdges,
    TetGraphBoundaryFaces,
}

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

    // Big enough to comfortably span the model from any plane orientation.
    let _plane_size = 64.;

    let mut planes: Vec<ClipPlane> = Vec::new();
    let mut _applied_planes: Option<Vec<ClipPlane>> = None;
    let mut _plane_models: Vec<Gm<Mesh, ColorMaterial>> = Vec::new();

    let mut gui = GUI::new(&context);

    let neighbor_edges_mesh = construct_neighbor_edges_mesh(&context, &tet_graph);
    let graph_nodes_mesh = construct_graph_nodes_mesh(&context, &tet_graph);
    let graph_boundary_faces_mesh = construct_graph_boundary_faces_mesh(&context, &tet_graph);

    let mut show_neighbor_edges_mesh = false;
    let mut show_graph_nodes_mesh = false;
    let mut show_graph_boundary_faces_mesh = false;

    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                egui::Panel::left("clip_planes_panel").show_inside(gui_context, |ui| {
                    ui.heading("Render Modes");

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut show_graph_boundary_faces_mesh, "Surface");
                        ui.checkbox(&mut show_neighbor_edges_mesh, "Graph Edges");
                        ui.checkbox(&mut show_graph_nodes_mesh, "Graph Nodes");
                    })
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

        let mut model: Vec<&dyn Object> = Vec::new();
        if show_graph_nodes_mesh {
            model.push(&graph_nodes_mesh);
        }
        if show_graph_boundary_faces_mesh {
            model.push(&graph_boundary_faces_mesh);
        }
        if show_neighbor_edges_mesh {
            model.push(&neighbor_edges_mesh);
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.85, 0.85, 0.85, 1.0, 1.0))
            .render(
                &camera,
                model, //.chain(plane_models.iter()),
                &[],
            )
            .write(|| gui.render())
            .unwrap();

        FrameOutput::default()
    });
}

fn construct_neighbor_edges_mesh(
    context: &Context,
    tet_graph: &TetGraph,
) -> Gm<InstancedMesh, ColorMaterial> {
    let instances = tet_graph._render_neighbor_edges();

    let model = Gm::new(
        InstancedMesh::new(context, &instances, &CpuMesh::cylinder(8)),
        ColorMaterial::default(),
    );
    model
}

fn construct_graph_nodes_mesh(
    context: &Context,
    tet_graph: &TetGraph,
) -> Gm<InstancedMesh, ColorMaterial> {
    let instances = tet_graph._render_tets_as_nodes();

    let model = Gm::new(
        InstancedMesh::new(context, &instances, &CpuMesh::sphere(16)),
        ColorMaterial::default(),
    );
    model
}

fn construct_graph_boundary_faces_mesh(
    context: &Context,
    tet_graph: &TetGraph,
) -> Gm<Mesh, ColorMaterial> {
    let instances = tet_graph._render_boundary_faces();

    let model = Gm::new(Mesh::new(context, &instances), ColorMaterial::default());
    model
}
