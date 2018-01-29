extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;

pub use gfx_app::{ColorFormat, DepthFormat};

use cgmath::{Deg, Matrix4, Point3, Vector3};
use gfx::Bundle;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tex_coord: [f32; 3] = "a_Color",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        //color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    fn new(p: [f32; 2], c: [i8; 3]) -> Vertex {
        Vertex {
            pos: p,
            tex_coord: [c[0] as f32, c[1] as f32, c[2] as f32],
        }
    }
}

fn default_view() -> Matrix4<f32> {
    Matrix4::look_at(
        Point3::new(1.5f32, -5.0, 3.0),
        Point3::new(0f32, 0.0, 0.0),
        Vector3::unit_z(),
    )
}

struct App<R: gfx::Resources> {
    bundle: Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend, window_targets: gfx_app::WindowTargets<R>) -> Self {
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("../shader/triangle_450.glslv"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("../shader/triangle_450.glslf"),
            .. gfx_app::shade::Source::empty()
        };

        let vertex_data = [
            Vertex::new([-0.5, -0.5], [1, 0, 0]),
            Vertex::new([ 0.5, -0.5], [0, 1, 0]),
            Vertex::new([ 0.0,  0.5], [0, 0, 1]),
        ];
        let index_data: &[u16] = &[
            0, 1, 2,
        ];

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);
        //let texels = [[0x20, 0xA0, 0xC0, 0xFF]];
        // let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
        //     texture::Kind::D2(1, 1, texture::AaMode::Single), texture::Mipmap::Provided, &[&texels]
        // ).unwrap();

        // let sinfo = texture::SamplerInfo::new(
        //     texture::FilterMethod::Bilinear,
        //     texture::WrapMode::Clamp
        // );

        let pso = factory.create_pipeline_simple(
            vs.select(backend).unwrap(),
            ps.select(backend).unwrap(),
            pipe::new()
        ).unwrap();

        let proj = cgmath::perspective(Deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);

        let data = pipe::Data {
            vbuf: vbuf,
            transform: (proj * default_view()).into(),
            locals: factory.create_constant_buffer(1),
            //color: (texture_view, factory.create_sampler(sinfo)),
            out_color: window_targets.color,
            out_depth: window_targets.depth,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
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

fn main() {
    use gfx_app::Application;
    App::launch_simple("Triangle Example");
}