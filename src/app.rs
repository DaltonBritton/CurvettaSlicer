use three_d::{
    Camera, ClearState, ColorMaterial, Context, CpuMesh, Event, FrameOutput, GUI, Gm,
    InstancedMesh, Mesh, MetricSpace, MouseButton, Object, OrbitControl, Viewport, Window,
    WindowSettings, degrees, egui, vec3,
};

use crate::data::TetGraph;

#[derive(Debug, Default)]
struct RenderSettings {
    show_graph_boundary_faces_mesh: bool,
    show_neighbor_edges_mesh: bool,
    show_graph_nodes_mesh: bool,
}

enum RenderObject {
    TetNodes(Gm<InstancedMesh, ColorMaterial>),
    TetEdges(Gm<InstancedMesh, ColorMaterial>),
    TetBoundarySurface(Gm<Mesh, ColorMaterial>),
    TetSurface(Gm<Mesh, ColorMaterial>),
}

impl RenderObject {
    fn as_object(&self) -> &dyn Object {
        match self {
            RenderObject::TetNodes(gm) => gm,
            RenderObject::TetEdges(gm) => gm,
            RenderObject::TetBoundarySurface(gm) => gm,
            RenderObject::TetSurface(gm) => gm,
        }
    }
}

struct Scene {
    camera: Camera,
    camera_control: OrbitControl,
    objects: Vec<RenderObject>,
}

pub struct App {
    tet_graph: TetGraph,
    window: Window,
    gl_context: Context,
    gui: GUI,

    scene: Scene,
    render_settings: RenderSettings,
}

impl App {
    pub fn new(tet_graph: TetGraph) -> Self {
        let window = Window::new(WindowSettings {
            title: "Tet Visualizer".to_string(),
            ..Default::default()
        })
        .unwrap();

        let gl_context = window.gl();

        let camera = Camera::new_perspective(
            window.viewport(),
            vec3(3.0, 2.5, 4.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            100000.0,
        );

        let camera_control = OrbitControl::new(camera.target(), 1.0, 1000.0);

        let gui = GUI::new(&gl_context);

        let objects = vec![
            Self::construct_graph_nodes_mesh(&gl_context, &tet_graph),
            Self::construct_neighbor_edges_mesh(&gl_context, &tet_graph),
            Self::construct_graph_boundary_faces_mesh(&gl_context, &tet_graph),
        ];

        Self {
            tet_graph,
            scene: Scene {
                camera,
                camera_control,
                objects,
            },
            gui,
            window,
            gl_context,
            render_settings: Default::default(),
        }
    }

    pub fn run(mut self) {
        self.window.render_loop(move |mut frame_input| {
            let mut panel_width = 0.0;
            self.gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {
                    egui::Panel::left("clip_planes_panel").show_inside(gui_context, |ui| {
                        ui.heading("Render Modes");

                        ui.horizontal(|ui| {
                            ui.checkbox(
                                &mut self.render_settings.show_graph_boundary_faces_mesh,
                                "Surface",
                            );
                            ui.checkbox(
                                &mut self.render_settings.show_neighbor_edges_mesh,
                                "Graph Edges",
                            );
                            ui.checkbox(
                                &mut self.render_settings.show_graph_nodes_mesh,
                                "Graph Nodes",
                            );
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
            self.scene.camera.set_viewport(viewport);
            self.scene
                .camera_control
                .handle_events(&mut self.scene.camera, &mut frame_input.events);

            for event in frame_input.events.iter_mut() {
                if let Event::MouseMotion {
                    delta,
                    button,
                    handled,
                    ..
                } = event
                {
                    if !*handled && *button == Some(MouseButton::Right) {
                        let distance = self
                            .scene
                            .camera
                            .position()
                            .distance(self.scene.camera.target());
                        let speed = distance * 0.0015;
                        let right = self.scene.camera.right_direction();
                        let up = right.cross(self.scene.camera.view_direction());
                        let translation = -right * delta.0 * speed + up * delta.1 * speed;

                        self.scene.camera.translate(translation);
                        self.scene.camera_control.target += translation;

                        *handled = true;
                    }
                }
            }

            if self.render_settings.show_graph_nodes_mesh {
                //model.push(&graph_nodes_mesh);
            }
            if self.render_settings.show_graph_boundary_faces_mesh {
                //model.push(&graph_boundary_faces_mesh);
            }
            if self.render_settings.show_neighbor_edges_mesh {
                //model.push(&neighbor_edges_mesh);
            }

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.85, 0.85, 0.85, 1.0, 1.0))
                .render(
                    &self.scene.camera,
                    self.scene.objects.iter().map(RenderObject::as_object), //.chain(plane_models.iter()),
                    &[],
                )
                .write(|| self.gui.render())
                .unwrap();

            FrameOutput::default()
        });
    }

    fn construct_neighbor_edges_mesh(context: &Context, tet_graph: &TetGraph) -> RenderObject {
        let instances = tet_graph._render_neighbor_edges();

        let model = Gm::new(
            InstancedMesh::new(context, &instances, &CpuMesh::cylinder(8)),
            ColorMaterial::default(),
        );
        RenderObject::TetEdges(model)
    }

    fn construct_graph_nodes_mesh(context: &Context, tet_graph: &TetGraph) -> RenderObject {
        let instances = tet_graph._render_tets_as_nodes();

        let model = Gm::new(
            InstancedMesh::new(context, &instances, &CpuMesh::sphere(16)),
            ColorMaterial::default(),
        );

        RenderObject::TetNodes(model)
    }

    fn construct_graph_boundary_faces_mesh(
        context: &Context,
        tet_graph: &TetGraph,
    ) -> RenderObject {
        let instances = tet_graph._render_boundary_faces();

        let model = Gm::new(Mesh::new(context, &instances), ColorMaterial::default());
        RenderObject::TetBoundarySurface(model)
    }
}
