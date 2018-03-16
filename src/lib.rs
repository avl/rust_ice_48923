
extern crate serde;
#[macro_use]
extern crate serde_derive;



//#[macro_use]
//extern crate savefile_derive;


mod vec;
//mod vec_helper;




//extern crate nalgebra;
//use glium::framebuffer::SimpleFrameBuffer;


#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct WaterCell {
    pos: [f32; 2],
    next: i32,
    sediment: f32,
    vel: [f32; 2],
    scratch_force: [f32; 2],
}

//implement_uniform_block!(WaterCell, pos, next, sediment, vel, scratch_force);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    size: f32,
}
//implement_vertex!(Vertex, position, color, size);

struct BitmapVertex {
    position: [f32; 2],
    size: [f32; 2],
    tex_offset: [f32; 2],
    frag_tex0: u32,
    coloring: [f32; 3],
}

