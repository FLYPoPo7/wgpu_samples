mod cube_texture;

use cube_texture::CubeTexture;
use eframe::{
    egui,
    egui_wgpu::{self, RenderState},
};
use glam::{vec3, Mat4, Vec3};
use std::f32::consts::PI;
use wgpu::util::DeviceExt;

use crate::meshes::cube;

const CANVAS: (f32, f32) = (800.0, 800.0);

fn get_mvp_matrix(start_time: std::time::Instant) -> Mat4 {
    let now = std::time::Instant::now()
        .duration_since(start_time)
        .as_secs_f32();

    let mut model_matrix = Mat4::from_rotation_x((PI / 10.0) * now.sin());
    model_matrix = model_matrix * Mat4::from_rotation_y(now * 0.2);
    model_matrix = model_matrix * Mat4::from_scale(vec3(1000.0, 1000.0, 1000.0));

    let view_matrix = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0));

    let projection_matrix =
        Mat4::perspective_rh((2.0 * PI) / 5.0, CANVAS.0 / CANVAS.1, 1.0, 3000.0);

    projection_matrix * view_matrix * model_matrix
}

pub struct Cubemap();

impl Cubemap {
    pub fn new_with_render_state(wgpu_render_state: &RenderState) -> Option<Self> {
        let device = &wgpu_render_state.device;
        let queue = &wgpu_render_state.queue;

        // Create the vertex buffer and layout
        let (vertex_buffer, vertex_buffer_layout) = {
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cubemap Vertex Buffer"),
                contents: bytemuck::cast_slice(cube::VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let vertex_buffer_layout = wgpu::VertexBufferLayout {
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

            (vertex_buffer, vertex_buffer_layout)
        };

        // Create the MVP buffer and bind group
        let (mvp_buffer, mvp_bind_group_layout, mvp_bind_group) = {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("MVP Buffer"),
                contents: bytemuck::cast_slice(&[0.0f32; 16 * 4]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("MPV Bind Group Layout"),
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

        let (cubemap_bind_group, cubemap_bind_group_layout) = {
            let images = vec![
                image::load_from_memory(include_bytes!("assets/cubemap/posx.jpg")).unwrap(),
                image::load_from_memory(include_bytes!("assets/cubemap/negx.jpg")).unwrap(),
                image::load_from_memory(include_bytes!("assets/cubemap/posy.jpg")).unwrap(),
                image::load_from_memory(include_bytes!("assets/cubemap/negy.jpg")).unwrap(),
                image::load_from_memory(include_bytes!("assets/cubemap/posz.jpg")).unwrap(),
                image::load_from_memory(include_bytes!("assets/cubemap/negz.jpg")).unwrap(),
            ];
            let rgbas = images
                .iter()
                .map(|image| image.to_rgba8())
                .collect::<Vec<_>>();
            let dimesions = rgbas[0].dimensions();

            let cube_texture = CubeTexture::new(
                device,
                dimesions.0,
                dimesions.1,
                wgpu::TextureFormat::Rgba8Unorm,
                1,
                wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
                wgpu::FilterMode::Linear,
                Some("Cube Texture"),
            );

            for (i, rgba) in rgbas.iter().enumerate() {
                queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &cube_texture.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: i as u32,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    rgba,
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * dimesions.0),
                        rows_per_image: Some(dimesions.1),
                    },
                    wgpu::Extent3d {
                        width: dimesions.0,
                        height: dimesions.1,
                        depth_or_array_layers: 1,
                    },
                );
            }

            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("MPV Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::Cube,
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
                label: Some("MVP Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&cube_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&cube_texture.sampler),
                    },
                ],
            });

            (bind_group, bind_group_layout)
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cubemap Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cubemap Pipeline Layout"),
            bind_group_layouts: &[&mvp_bind_group_layout, &cubemap_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cubemap Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu_render_state.target_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
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
            .insert(AppRenderResources {
                start_time: std::time::Instant::now(),
                pipeline,
                vertex_buffer,
                mvp_buffer,
                mvp_bind_group,
                cubemap_bind_group,
            });

        Some(Self())
    }
}

impl eframe::App for Cubemap {
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

impl Cubemap {
    fn custom_painting(&self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(egui::Vec2::new(CANVAS.0, CANVAS.1), egui::Sense::click());
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            CustomPaintCallback(),
        ));
    }
}

struct CustomPaintCallback();

impl egui_wgpu::CallbackTrait for CustomPaintCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &AppRenderResources = callback_resources.get().unwrap();
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
        let resources: &AppRenderResources = callback_resources.get().unwrap();
        render_pass.set_pipeline(&resources.pipeline);
        render_pass.set_bind_group(0, &resources.mvp_bind_group, &[]);
        render_pass.set_bind_group(1, &resources.cubemap_bind_group, &[]);
        render_pass.set_vertex_buffer(0, resources.vertex_buffer.slice(..));
        render_pass.draw(0..cube::VERTEX_COUNT, 0..1);
    }
}

struct AppRenderResources {
    pub start_time: std::time::Instant,
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub mvp_buffer: wgpu::Buffer,
    pub mvp_bind_group: wgpu::BindGroup,
    pub cubemap_bind_group: wgpu::BindGroup,
}
