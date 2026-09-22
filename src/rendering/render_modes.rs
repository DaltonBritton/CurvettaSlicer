use std::fmt::Display;

use crate::rendering::render_object::RenderObject;

#[derive(Debug, PartialEq, Eq)]
pub enum RenderMode {
    FullTets,
    TetGraph(TetGraphRenderSettings),
    TetGraphBoundaryFaces,
}

impl RenderMode {
    pub fn should_render(&self, object: &RenderObject) -> bool {
        match (self, object) {
            (RenderMode::FullTets, RenderObject::TetSurface(_)) => true,
            (RenderMode::TetGraph(tet_graph_render_settings), RenderObject::TetNodes(_, _)) => {
                tet_graph_render_settings.show_nodes
            }
            (RenderMode::TetGraph(tet_graph_render_settings), RenderObject::TetEdges(_, _)) => {
                tet_graph_render_settings.show_edges
            }
            (RenderMode::TetGraphBoundaryFaces, RenderObject::TetBoundarySurface(_)) => true,
            _ => false,
        }
    }
}

impl Display for RenderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let render_mode_name = match self {
            RenderMode::FullTets => "FullTets",
            RenderMode::TetGraph(_) => "TetGraph",
            RenderMode::TetGraphBoundaryFaces => "TetGraphBoundaryFaces",
        };

        f.write_str(render_mode_name)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TetGraphRenderSettings {
    pub show_nodes: bool,
    pub show_edges: bool,
}

impl Default for TetGraphRenderSettings {
    fn default() -> Self {
        Self {
            show_nodes: false,
            show_edges: true,
        }
    }
}
