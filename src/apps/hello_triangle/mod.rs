use eframe::{
    egui,
    egui_wgpu::{self, RenderState},
};

const CANVAS_SIZE: f32 = 600.0;

pub struct HelloTriangle();

impl HelloTriangle {
    pub fn new_with_render_state(wgpu_render_state: &RenderState) -> Option<Self> {
        let device = &wgpu_render_state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("HelloTriangle Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("HelloTriangle Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("HelloTriangle Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
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
            .insert(HelloTriangleRenderResources { pipeline });

        Some(Self())
    }
}

impl eframe::App for HelloTriangle {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.custom_painting(ui);
                });
            });
        });
    }
}

impl HelloTriangle {
    fn custom_painting(&self, ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(egui::Vec2::splat(CANVAS_SIZE), egui::Sense::click());
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
        _queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
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
        render_pass.draw(0..3, 0..1);
    }
}

struct HelloTriangleRenderResources {
    pub pipeline: wgpu::RenderPipeline,
}
