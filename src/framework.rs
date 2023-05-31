use glam::Vec3;
use rand::Rng;
use rand::distributions::{Distribution, Standard};


use bytemuck::{Pod, Zeroable};
use wgpu::{RenderPipeline, BindGroup, Surface, Buffer};
use std::f32::consts::PI;
use std::{borrow::Cow, mem, f32::consts};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window, dpi::{LogicalSize, LogicalPosition},
};
use wgpu::util::DeviceExt;

use crate::objs::*;

use ndarray::array;


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Globals {
    mvp: [[f32; 4]; 4], // matrix
    size: [f32; 2], // textureのサイズ
    pad: [f32; 2], // structのメモリ配列のあまりかな
}

/**
 * 座標(x, y): [f32, 2]
 * 
 */
#[repr(C, align(256))]
#[derive(Clone, Copy, Zeroable, Debug)]
struct Locals {
    position: [f32; 3],
    velocity: [f32; 2],
    color: u32,
}

/**
 * 座標(x, y): [f32, 2]
 * 
 */
#[repr(C, align(256))]
#[derive(Clone, Copy, Zeroable)]
struct Line {
    p: [f32; 4],
    color: u32,
    _pad: u32,
}


struct DataWriter<T> {
    buffer: Buffer,
    data: Vec<T>,
}
impl<T> DataWriter<T> {
    fn write_buf(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let uniform_alignment = device.limits().min_uniform_buffer_offset_alignment;
        queue.write_buffer(&self.buffer, 0, unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * uniform_alignment as usize,
            )
        });
    }
}

struct ViewState {
    config: wgpu::SurfaceConfiguration,
    surface: Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipelines: Vec<RenderPipeline>,
    bgs: Vec<BindGroup>,
}

impl ViewState {
}

fn redraw(vs: &ViewState, vertex_buf: &Buffer, index_buf: &Buffer, obj: &dyn IGeometry) {
    let frame = vs.surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
    vs.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {

        // localsに書き込む
        let mut rng = rand::thread_rng();
        let color = rng.gen::<u32>();
        let mut rng = rand::thread_rng();
        

        let uniform_alignment = vs.device.limits().min_uniform_buffer_offset_alignment;

        // STEP: sceneを作る
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    //load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        // STEP: pipelineを設定する(シェーダー)
        rpass.set_pipeline(&vs.pipelines[0]);
        // STEP: シェーダーにデータをバインドする
        rpass.set_bind_group(0, &vs.bgs[0], &[]);
        rpass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);
        rpass.set_vertex_buffer(0, vertex_buf.slice(..));
        println!("vertex:: {:?}", vertex_buf.slice(..));
        println!("indexbuf:: {:?}", index_buf.slice(..));
        //let index_count = Cube::indices().len();
        //let index_count = index_buf.size();
        let index_count = obj.indices().len();
        rpass.insert_debug_marker("Draw!");
        println!("index_count:: {}", index_count);
        rpass.set_pipeline(&vs.pipelines[1]);
        rpass.draw_indexed(0..index_count as u32, 0, 0..1);
    }

    vs.queue.submit(Some(encoder.finish()));

    // 多分フレーム反映
    frame.present();
}


pub async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();
    println!("size: {:?}", size);
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::POLYGON_MODE_LINE,
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./shader.wgsl"))),
    });


    let global_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(mem::size_of::<Globals>() as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                ],
                label: None,
        });

    let local_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: wgpu::BufferSize::new(mem::size_of::<Locals>() as _),
                },
                count: None,
            }],
            label: None,
        });


    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: None,
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let swapchain_capabilities = surface.get_supported_formats(&adapter);
    let swapchain_format = swapchain_capabilities[0];
    println!("swapchain_format: {:?}", swapchain_format);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        // alpha_mode: swapchain_capabilities.alpha_modes[0],
        alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
    };

    let tex_width = 100.;

    let mut vs = ViewState {
        config: config,
        surface: surface,
        device: device,
        queue: queue,
        pipelines: vec![],
        bgs: vec![],
    };


    //let m_scale = glam::Mat4::from_scale(Vec3 {x: 0.5, y: 0.5, z: 0.0});
    let m_scale = glam::Mat4::from_scale(Vec3 {x: 1.0, y: 1.0, z: 0.0});
    let m_rot_y = glam::Mat4::from_rotation_y(PI/4.);
    let m_rot_z = glam::Mat4::from_rotation_z(PI/4.);
    let m_rot_x = glam::Mat4::from_rotation_x(PI/4.);
    let m4 = glam::Mat4::orthographic_rh(
                 0.0,
                 size.width as f32,
                 0.0,
                 size.height as f32,
                 -1.0,
                 1.0,
                 );
    let m = m_rot_x * m_rot_z * m_rot_y * m4;
    let m = m4;


    let projection = glam::Mat4::perspective_rh(consts::FRAC_PI_4, size.width as f32 /size.height as f32, 1.0, 10.0);
    // Z軸が上で、カメラ位置が(-1,5, -5.0, 3.0)で、Zの位置を見る行列
    let mview = glam::Mat4::look_at_rh(
        glam::Vec3::new(1.5f32, -5.0, 3.0),
        glam::Vec3::ZERO,
        glam::Vec3::Z,
    );
    let mmview = projection * mview;


    let globals = Globals {
        mvp: mmview.to_cols_array_2d(),
            size: [tex_width; 2],
            pad: [0.0; 2],
    };

    let global_buffer = vs.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("global"),
        contents: bytemuck::bytes_of(&globals),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
    });


    let n_max = 5000;
    let uniform_alignment =
        vs.device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
    let local_buffer = vs.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("local"),
        size: (n_max as wgpu::BufferAddress) * uniform_alignment,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });
    //let mut cube = Cube::new();
    let mut obj = Sphere::new(20, 20);
    //let mut obj = Cylinder::new(20, 0.2, 1.);
    //let mut obj = Plane::new(10, 10);
    //let mut obj = Sphere::new(5, 5);
        println!("vertex:: {:?}", obj.vertices());
        println!("indices:: {:?}", obj.indices());
    //cube.scale(array![100., 100., 100.]);
    let vertex_buf = vs.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&obj.vertices()),
        usage: wgpu::BufferUsages::VERTEX,
    });
    

    let index_buf = vs.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        //contents: bytemuck::cast_slice(&Cube::indices()),
        contents: bytemuck::cast_slice(&obj.indices()),
        usage: wgpu::BufferUsages::INDEX,
    });


    let global_group = vs.device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &global_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: global_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: None,
    });



    // グループをセットしておく？
    vs.bgs.push(global_group);

    let pipeline_layout = vs.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&global_bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertex_size = mem::size_of::<Vertex>();
    let vertex_buffers = [wgpu::VertexBufferLayout {
        array_stride: vertex_size as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 0,
            },
        ],
    }];


    // テクスチャの表示
    let render_pipeline = vs.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &vertex_buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        // 矩形の4点で描画する
        //primitive: wgpu::PrimitiveState {
        //    topology: wgpu::PrimitiveTopology::TriangleStrip,
        //    strip_index_format: Some(wgpu::IndexFormat::Uint16),
        //    ..wgpu::PrimitiveState::default()
        //},
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            //polygon_mode: wgpu::PolygonMode::Line,
            ..wgpu::PrimitiveState::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let pipeline_wire = vs.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: vs.config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            operation: wgpu::BlendOperation::Add,
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        },
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Line,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
    vs.pipelines.push(render_pipeline);
    vs.pipelines.push(pipeline_wire);

    vs.surface.configure(&vs.device, &vs.config);

    // メインループ
    event_loop.run(move |event, _, control_flow| {

        let _ = (&instance, &adapter, &shader, &vs.pipelines[0]);

        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Window sizeにする
                vs.surface.configure(&vs.device, &vs.config);
                window.request_redraw();
            }
            // update
            Event::RedrawRequested(_) => {
                redraw(&vs, &vertex_buf, &index_buf, &obj);
                // 無限ループ
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });

}

fn start() {
    const WINDOW_WIDTH: f32  = 800.0;

    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new()
                .with_decorations(false)
                .with_always_on_top(true)
                .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_WIDTH))
                .with_transparent(true);
    builder = builder.with_title("hello-triangle");
    #[cfg(windows_OFF)] // TODO
    {
        use winit::platform::windows::WindowBuilderExtWindows;
        builder = builder.with_no_redirection_bitmap(true);
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(run(event_loop, window));
    }
}
