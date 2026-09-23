use binrw::binrw;

#[binrw]
#[brw(little)]
pub struct StlTriangle {
    pub normal: [f32; 3],
    pub vertices: [[f32; 3]; 3],
    pub attr: u16,
}

#[binrw]
#[brw(little)]
pub struct StlFile {
    #[brw(pad_before = 80)]
    #[bw(try_calc(u32::try_from(triangles.len())))]
    pub n_tri: u32,

    #[br(count=n_tri)]
    pub triangles: Vec<StlTriangle>,
}
