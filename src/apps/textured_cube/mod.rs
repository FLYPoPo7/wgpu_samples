use eframe::{
    egui,
    egui_wgpu::{self, RenderState},
};
use glam::{Mat4, Quat, Vec3};
use std::f32::consts::PI;
use wgpu::util::DeviceExt;

use crate::meshes::cube;

const CANVAS: (f32, f32) = (600.0, 600.0);

fn get_mvp_matrix(start_time: std::time::Instant) -> Mat4 {
    let now = std::time::Instant::now()
        .duration_since(start_time)
        .as_secs_f32();

    let model_matrix = Mat4::from_rotation_translation(
        Quat::from_axis_angle(Vec3::new(now.sin(), now.cos(), 0.0), 1.0),
        Vec3::ZERO,
    );

    let view_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, -4.0));

    let projection_matrix = Mat4::perspective_rh((2.0 * PI) / 5.0, CANVAS.0 / CANVAS.1, 1.0, 100.0);

    projection_matrix * view_matrix * model_matrix
}

pub struct TexturedCube();

impl TexturedCube {
    pub fn new_with_render_state(wgpu_render_state: &RenderState) -> Option<Self> {
        let device = &wgpu_render_state.device;
        let queue = &wgpu_render_state.queue;

        let vertices_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: 10 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 4 * std::mem::size_of::<f32>() as u64,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 8 * std::mem::size_of::<f32>() as u64,
                    shader_location: 2,
                },
            ],
        };

        let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("TexturedCube Vertex Buffer"),
            #[rustfmt::skip]
            contents: bytemuck::cast_slice(cube::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create the mvp buffer and bind group
        let (mvp_buffer, mvp_bind_group_layout, mvp_bind_group) = {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("MVP Buffer"),
                contents: bytemuck::cast_slice(&[0.0f32; 16 * 4]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("MVP Bind Group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

            (buffer, bind_group_layout, bind_group)
        };

        // Create the diffuse texture and bind group
        let (diffuse_texture, diffuse_bind_group_layout, diffuse_bind_group) = {
            let bytes = include_bytes!("assets/happy-tree.png");
            let image = image::load_from_memory(bytes).unwrap();
            let rgba = image.to_rgba8();
            let dimension = image.to_rgba8().dimensions();
            let size = wgpu::Extent3d {
                width: dimension.0,
                height: dimension.1,
                depth_or_array_layers: 1,
            };

            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("TexturedCube Texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

            queue.write_texture(
                wgpu::ImageCopyTextureBase {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimension.0),
                    rows_per_image: Some(dimension.1),
                },
                size,
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("TexturedCube Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("TexturedCube Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
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
                });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("TexturedCube Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

            (texture, bind_group_layout, bind_group)
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("TexturedCube Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("TexturedCube Pipeline Layout"),
            bind_group_layouts: &[&mvp_bind_group_layout, &diffuse_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("TexturedCube Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertices_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu_render_state.target_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            // depth_stencil: Some(wgpu::DepthStencilState {
            //     format: wgpu::TextureFormat::Depth24Plus,
            //     depth_write_enabled: true,
            //     depth_compare: wgpu::CompareFunction::Less,
            //     stencil: wgpu::StencilState::default(),
            //     bias: wgpu::DepthBiasState::default(),
            // }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(HelloTriangleRenderResources {
                start_time: std::time::Instant::now(),
                pipeline,
                vertices_buffer,
                mvp_buffer,
                mvp_bind_group,
                diffuse_texture,
                diffuse_bind_group,
            });

        Some(Self())
    }
}

impl eframe::App for TexturedCube {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.custom_painting(ui);
                });
            });
        });
        // This is needed to animate the rotating cube. It tells eframe to call update() again on the next event loop iteration.
        ctx.request_repaint();
    }
}

impl TexturedCube {
    fn custom_painting(&self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(egui::Vec2::new(CANVAS.0, CANVAS.1), egui::Sense::click());
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            HelloTriangleCallback(),
        ));
    }
}

struct HelloTriangleCallback();

impl egui_wgpu::CallbackTrait for HelloTriangleCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &HelloTriangleRenderResources = callback_resources.get().unwrap();
        queue.write_buffer(
            &resources.mvp_buffer,
            0,
            bytemuck::cast_slice(&[get_mvp_matrix(resources.start_time)]),
        );
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: eframe::egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        let resources: &HelloTriangleRenderResources = callback_resources.get().unwrap();
        render_pass.set_pipeline(&resources.pipeline);
        render_pass.set_bind_group(0, &resources.mvp_bind_group, &[]);
        render_pass.set_bind_group(1, &resources.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, resources.vertices_buffer.slice(..));
        render_pass.draw(0..cube::VERTEX_COUNT, 0..1);
    }
}

struct HelloTriangleRenderResources {
    pub start_time: std::time::Instant,
    pub pipeline: wgpu::RenderPipeline,
    pub vertices_buffer: wgpu::Buffer,
    pub mvp_buffer: wgpu::Buffer,
    pub mvp_bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    pub diffuse_texture: wgpu::Texture,
    pub diffuse_bind_group: wgpu::BindGroup,
}
