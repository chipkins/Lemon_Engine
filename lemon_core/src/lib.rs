[#macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate cgmath;

use std::time::Instant;

pub use cgmath::{Deg, Matrix4, Point3, Vector3};

pub use gfx_app::{ColorFormat, DepthFormat};

use gfx::Bundle;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
    }

    vertex Instance {
        center: [f32; 4] = "a_Center",
        color: [f32; 3] = "a_Color",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        instance: gfx::InstanceBuffer<Instance> = (),
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    fn new(p: [f32; 4]) -> Vertex {
        Vertex {
            pos: p,
        }
    }
}

fn create_plane<R: gfx::Resources, F: gfx::Factory<R>>(factory: &mut F, size: i8)
    -> (gfx::handle::Buffer<R, Vertex>, gfx::Slice<R>)
{
    use gfx::traits::FactoryExt;
    let vertex_data = [
        Vertex::new([ size, -size, 0]),
        Vertex::new([ size,  size, 0]),
        Vertex::new([-size, -size, 0]),
        Vertex::new([-size,  size, 0]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2,
        2, 1, 3,
    ];

    return factory.create_vertex_buffer_with_slice(&vertex_data, index_data)
}

struct Camera {
    view: Matrix4<f32>,
    projection: cgmath::PerspectiveFov<f32>,
}

pub struct App<R: gfx::Resources> {
    bundle: Bundle<R, pipe::Data<R>>,
    frame_start: Instant
}

impl<R: gfx::Resources> App<R> {
    fn update() {
        let dt = self.frame_start.elapsed();
        self.frame_start = Instant::now();
        let dt = dt.as_secs() as f32 + dt.subsec_nanos() as f32 / 1000000000.;
    }
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend, window_targets: gfx_app::WindowTargets<R>) -> Self {
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("../../shaders/shader.glslv"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("../../shaders/shader.glslf"),
            .. gfx_app::shade::Source::empty()
        };

        let vertex_data = [
            // front
            Vertex::new([ 1.,  1., -1., 1.], [1, 0, 0]),
            Vertex::new([-1.,  1., -1., 1.], [0, 1, 0]),
            Vertex::new([-1.,  1.,  1., 1.], [0, 0, 1]),
            Vertex::new([ 1.,  1.,  1., 1.], [0, 1, 0]),
        ];
        let index_data: &[u16] = &[
            16, 17, 18, 18, 19, 16, //front
        ];

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

        let pso = factory.create_pipeline_simple(
            vs.glsl_150,
            ps.glsl_150,
            pipe::new()
        ).unwrap();

        let proj = cgmath::perspective(Deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);

        let data = pipe::Data {
            vbuf: vbuf,
            instance: instance,
            transform: (proj * default_view()).into(),
            locals: factory.create_constant_buffer(1),
            out_color: window_targets.color,
            out_depth: window_targets.depth,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
            frame_start: Instant::now(),
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        let locals = Locals { transform: self.bundle.data.transform };
        encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
        encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
        self.bundle.encode(encoder);
    }

    fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
        self.bundle.data.out_color = window_targets.color;
        self.bundle.data.out_depth = window_targets.depth;

        let proj = cgmath::perspective(Deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);
        self.bundle.data.transform = (proj * default_view()).into();
    }
}