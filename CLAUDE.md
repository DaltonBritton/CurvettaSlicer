# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project purpose

This is a **learning project** for the repo owner. It is a Rust desktop app that loads a binary STL mesh, tetrahedralizes its interior volume with `tritet` (a TetGen binding), and renders the resulting tetrahedra with `three-d`, with an egui side panel for interactively adding/editing clipping planes that hide whole tetrahedra.

## Scope of Claude's contributions

**Only implement UI/interface code.** Do not implement or modify program logic — meshing, geometry math, tet generation, data structures for tets/graphs, clip-plane math, STL parsing, etc. For anything in that category, only provide feedback or suggestions; leave the implementation to the user.

The crate split mirrors this boundary: **everything in `crates/tet-visulizer-gui/` is fair game; everything in `crates/tet-visulizer-core/` is not.**

"Interface" here means the `egui`/`three-d` window setup and widgets in `tet-visulizer-gui` (panels, controls, camera/window wiring, the `rendering/` glue that turns core data into `three-d` meshes/instances), and similar UI-facing code. Anything under `tet-visulizer-core` — `data/`, and the geometry/meshing modules (`gen_tet.rs`, `stl_to_tet_input.rs`, `tet_to_mesh.rs`, `sphere_tet.rs`, `clip_plane.rs`'s math) — is logic; hands off unless asked to review.

The user is explicitly using this project to practice **professional Rust**: typestate patterns, clean structure, and readability. Favor suggestions (not unsolicited rewrites) that point toward those goals when reviewing logic code.

## Commands

All commands run from the workspace root.

- Build: `cargo build` (whole workspace)
- Run: `cargo run -- path/to/mesh.stl` — the STL path is a required positional argument
- Lint: `cargo clippy --workspace`
- Format: `cargo fmt --all`
- Check (fast, no codegen): `cargo check --workspace`, or `cargo check -p tet-visulizer-core` to check the logic crate alone (this also verifies core stays free of `egui`)
- There are currently no tests in the repo (no `#[test]` functions).

## Architecture

Cargo workspace (virtual manifest at the root) with two members under `crates/`:

- **`tet-visulizer-core`** — library crate `tet_visulizer_core`: STL parsing, tet generation, and the tet/graph data structures. Depends on `three-d` only for its `cgmath` re-exports (`Vector3`, `CpuMesh`), never on `egui`.
- **`tet-visulizer-gui`** — the frontend: library modules `app` and `rendering`, plus the `tet-visulizer` binary (`src/main.rs`). Owns all `egui`/`three-d` window, camera, and widget code.

Because `TetGraph` now lives in another crate, `rendering/tet_graph.rs` exposes its mesh builders through the `TetGraphRender` extension trait instead of an inherent `impl` — callers must import the trait.

Pipeline, in order (items 1–5 in core, 6 in gui):

1. **`stl_serialization.rs`** — `StlFile`/`StlTriangle`, binary-STL read/write via `binrw`.
2. **`stl_to_tet_input.rs`** — converts a parsed `StlFile` into `tritet::InputDataTetMesh` (dedupes shared vertices by quantized position into a `points`/`facets` list).
3. **`gen_tet.rs`** — runs `tritet::Tetgen` over that input to produce a volumetric tetrahedral mesh (`Tetgen`).
4. **`tet_to_mesh.rs`** — converts the `Tetgen` output into a `three_d::CpuMesh` for rendering, *and* a parallel `Vec<[Vector3<f32>; 4]>` of per-tet corner points. Each tet contributes exactly 12 vertices to the mesh (4 faces × 3 verts, unshared, so each triangle can be flat-colored independently) in a fixed order — tet `i` owns vertex range `[i*12, i*12+12)`. This indexing convention is load-bearing: `clip_plane.rs` depends on it to map "hide this tet" to "hide these 12 mesh indices."
5. **`clip_plane.rs`** — `ClipPlane` (angle/offset-based plane definition) and `visible_triangle_indices`, which filters `tet_corners` against the active planes and returns the surviving triangle index buffer for the mesh. A tet is hidden entirely if *any* corner is on the clipped side of *any* enabled plane.
6. **`app.rs`** (gui) — `App` owns the `three-d` window/camera/orbit-control loop and the egui side panel (render mode + color mode selection); `main.rs` is a thin entry point that loads the STL, builds the `TetGraph`, and hands it to `App::new(..).run()`.

Also present, not wired into `App`:
- **`sphere_tet.rs`** (core) — standalone icosphere-as-tetrahedra generator (subdivided icosahedron fanned to the origin), used for experimentation/testing rendering rather than the STL pipeline.
- **`clip_plane.rs`** / **`tet_to_mesh.rs`** (core) — from the earlier pre-`TetGraph` rendering path; still compiled but no longer called by the gui.

### Coordinate conventions

Tetgen/STL data is Z-up. `tet_to_mesh::read_point` swaps Y/Z (negating the new Z) once, at the boundary, so everything downstream (mesh vertices, `tet_corners`, clip-plane math, the camera) is consistently Y-up.
