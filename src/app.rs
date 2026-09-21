use three_d::{
    Camera, ClearState, ColorMaterial, Context, CpuMesh, Event, FrameOutput, GUI, Gm,
    InstancedMesh, Mesh, MetricSpace, MouseButton, Object, OrbitControl, Viewport, Window,
    WindowSettings, degrees,
    egui::{self, ComboBox},
    vec3,
};

use crate::{
    data::TetGraph,
    rendering::{
        render_modes::{RenderMode, TetGraphRenderSettings},
        render_object::RenderObject,
    },
};

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
    render_mode: RenderMode,
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
            render_mode: RenderMode::TetGraphBoundaryFaces,
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
                    egui::Panel::left("render_mode_settings").show_inside(gui_context, |ui| {
                        ui.heading("Render Modes");
                        ComboBox::from_label("RenderMode")
                            .selected_text(format!("{}", &mut self.render_mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.render_mode,
                                    RenderMode::TetGraphBoundaryFaces,
                                    "BoundaryFaces",
                                );

                                ui.selectable_value(
                                    &mut self.render_mode,
                                    RenderMode::FullTets,
                                    "FullTets",
                                );

                                ui.selectable_value(
                                    &mut self.render_mode,
                                    RenderMode::TetGraph(Default::default()),
                                    "TetGraph",
                                );
                            });

                        if let RenderMode::TetGraph(TetGraphRenderSettings {
                            show_nodes,
                            show_edges,
                        }) = &mut self.render_mode
                        {
                            ui.checkbox(show_nodes, "Show Nodes");
                            ui.checkbox(show_edges, "Show Edges");
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

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.85, 0.85, 0.85, 1.0, 1.0))
                .render(
                    &self.scene.camera,
                    self.scene
                        .objects
                        .iter()
                        .filter(|obj| self.render_mode.should_render(obj))
                        .map(RenderObject::as_object),
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
