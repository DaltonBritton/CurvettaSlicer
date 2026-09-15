# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project purpose

This is a **learning project** for the repo owner. It is a Rust desktop app that loads a binary STL mesh, tetrahedralizes its interior volume with `tritet` (a TetGen binding), and renders the resulting tetrahedra with `three-d`, with an egui side panel for interactively adding/editing clipping planes that hide whole tetrahedra.

## Scope of Claude's contributions

**Only implement UI/interface code.** Do not implement or modify program logic — meshing, geometry math, tet generation, data structures for tets/graphs, clip-plane math, STL parsing, etc. For anything in that category, only provide feedback or suggestions; leave the implementation to the user.

"Interface" here means: the `egui`/`three-d` window setup and widgets in `src/main.rs` (panels, controls, camera/window wiring), and similar UI-facing glue. Anything under `src/data/`, and the geometry/meshing modules (`gen_tet.rs`, `stl_to_tet_input.rs`, `tet_to_mesh.rs`, `sphere_tet.rs`, `clip_plane.rs`'s math), is logic — hands off unless asked to review.

The user is explicitly using this project to practice **professional Rust**: typestate patterns, clean structure, and readability. Favor suggestions (not unsolicited rewrites) that point toward those goals when reviewing logic code.

## Commands

- Build: `cargo build`
- Run: `cargo run` (note: `src/main.rs` currently opens a hardcoded/empty file path via `File::open("")` — this needs to be pointed at a real `.stl` file before it will run)
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Check (fast, no codegen): `cargo check`
- There are currently no tests in the repo (no `#[test]` functions).

## Architecture

Binary crate (`main.rs`) driven by library crate `tet_visulizer` (`lib.rs` re-exports the modules below).

Pipeline, in order:

1. **`stl_serialization.rs`** — `StlFile`/`StlTriangle`, binary-STL read/write via `binrw`.
2. **`stl_to_tet_input.rs`** — converts a parsed `StlFile` into `tritet::InputDataTetMesh` (dedupes shared vertices by quantized position into a `points`/`facets` list).
3. **`gen_tet.rs`** — runs `tritet::Tetgen` over that input to produce a volumetric tetrahedral mesh (`Tetgen`).
4. **`tet_to_mesh.rs`** — converts the `Tetgen` output into a `three_d::CpuMesh` for rendering, *and* a parallel `Vec<[Vector3<f32>; 4]>` of per-tet corner points. Each tet contributes exactly 12 vertices to the mesh (4 faces × 3 verts, unshared, so each triangle can be flat-colored independently) in a fixed order — tet `i` owns vertex range `[i*12, i*12+12)`. This indexing convention is load-bearing: `clip_plane.rs` depends on it to map "hide this tet" to "hide these 12 mesh indices."
5. **`clip_plane.rs`** — `ClipPlane` (angle/offset-based plane definition) and `visible_triangle_indices`, which filters `tet_corners` against the active planes and returns the surviving triangle index buffer for the mesh. A tet is hidden entirely if *any* corner is on the clipped side of *any* enabled plane.
6. **`main.rs`** — owns the `three-d` window/camera/orbit-control loop and the egui side panel (add/remove/edit clip planes). Recomputes the mesh's index buffer and the semi-transparent plane preview meshes only when the plane list actually changes (`applied_planes` cache check against `planes`).

Also present, not yet wired into `main.rs`:
- **`sphere_tet.rs`** — standalone icosphere-as-tetrahedra generator (subdivided icosahedron fanned to the origin), used for experimentation/testing rendering rather than the STL pipeline.
- **`src/data/`** — new, in-progress module (`data::tet`, `data::tet_graph`) intended to hold richer tet/mesh data structures; currently minimal/empty stubs.

### Coordinate conventions

Tetgen/STL data is Z-up. `tet_to_mesh::read_point` swaps Y/Z (negating the new Z) once, at the boundary, so everything downstream (mesh vertices, `tet_corners`, clip-plane math, the camera) is consistently Y-up.
