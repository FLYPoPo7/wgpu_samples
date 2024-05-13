use eframe::egui;

use crate::hello_triangle;

/// The type of app to run.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppType {
    BasicGraphics,
    WebGPUFeatures,
    GPGPUDemos,
    GraphicsTechniques,
    Benchmarks,
}

impl std::fmt::Display for AppType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BasicGraphics => write!(f, "Basic Graphics"),
            Self::WebGPUFeatures => write!(f, "WebGPU Features"),
            Self::GPGPUDemos => write!(f, "GPGPU Demos"),
            Self::GraphicsTechniques => write!(f, "Graphics Techniques"),
            Self::Benchmarks => write!(f, "Benchmarks"),
        }
    }
}

/// List of apps to run.
const APPS: [(
    &str,
    AppType,
    fn(&eframe::Frame) -> Option<Box<dyn eframe::App>>,
); 27] = [
    // Basic Graphics
    (
        "helloTriangle",
        AppType::BasicGraphics,
        |frame: &eframe::Frame| {
            Some(Box::new(
                hello_triangle::HelloTriangle::new_with_render_state(
                    &frame.wgpu_render_state().unwrap(),
                )
                .unwrap(),
            ))
        },
    ),
    (
        "helloTriangleMSAA",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "rotatingCube",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "twoCubes",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "texturedCube",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "instancedCube",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "fractalCube",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "cubemap",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    // WebGPU Features
    (
        "samplerParameters",
        AppType::WebGPUFeatures,
        |_frame: &eframe::Frame| None,
    ),
    (
        "reversedZ",
        AppType::WebGPUFeatures,
        |_frame: &eframe::Frame| None,
    ),
    (
        "renderBundles",
        AppType::WebGPUFeatures,
        |_frame: &eframe::Frame| None,
    ),
    // GPGPU Demos
    (
        "computeBoids",
        AppType::GPGPUDemos,
        |_frame: &eframe::Frame| None,
    ),
    (
        "gameOfLife",
        AppType::GPGPUDemos,
        |_frame: &eframe::Frame| None,
    ),
    (
        "bitonicSort",
        AppType::GPGPUDemos,
        |_frame: &eframe::Frame| None,
    ),
    // Graphics Techniques
    (
        "cameras",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "normalMap",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "shadowMapping",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "deferredRendering",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "particles",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "points",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "imageBlur",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "cornell",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "a-buffer",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "skinnedMesh",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "textRenderingMsdf",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "volumeRenderingTexture3D",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    // Benchmarks
    (
        "animometer",
        AppType::Benchmarks,
        |_frame: &eframe::Frame| None,
    ),
];

/// The main app that switches between different apps.
pub struct MainApp {
    current_app: Option<Box<dyn eframe::App>>,
}

/// Implement the main app.
impl MainApp {
    pub fn new(_cc: &eframe::CreationContext) -> Option<Self> {
        Some(Self { current_app: None })
    }

    fn switch_app(&mut self, app_name: &str, frame: &eframe::Frame) {
        APPS.iter().for_each(|(name, _, app)| {
            if *name == app_name {
                self.current_app = app(frame);
            }
        });
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("wgpu samples");
                ui.separator();

                ui.heading(AppType::BasicGraphics.to_string());
                APPS.iter()
                    .filter(|x| (**x).1 == AppType::BasicGraphics)
                    .for_each(|(name, _, _)| {
                        if ui.link(*name).clicked() {
                            self.switch_app(*name, &frame);
                        }
                    });

                ui.heading(AppType::WebGPUFeatures.to_string());
                APPS.iter()
                    .filter(|x| (**x).1 == AppType::WebGPUFeatures)
                    .for_each(|(name, _, _)| {
                        if ui.link(*name).clicked() {
                            self.switch_app(*name, &frame);
                        }
                    });

                ui.heading(AppType::GPGPUDemos.to_string());
                APPS.iter()
                    .filter(|x| (**x).1 == AppType::GPGPUDemos)
                    .for_each(|(name, _, _)| {
                        if ui.link(*name).clicked() {
                            self.switch_app(*name, &frame);
                        }
                    });

                ui.heading(AppType::GraphicsTechniques.to_string());
                APPS.iter()
                    .filter(|x| (**x).1 == AppType::GraphicsTechniques)
                    .for_each(|(name, _, _)| {
                        if ui.link(*name).clicked() {
                            self.switch_app(*name, &frame);
                        }
                    });

                ui.heading(AppType::Benchmarks.to_string());
                APPS.iter()
                    .filter(|x| (**x).1 == AppType::Benchmarks)
                    .for_each(|(name, _, _)| {
                        if ui.link(*name).clicked() {
                            self.switch_app(*name, &frame);
                        }
                    });
            });
        });

        if let Some(app) = self.current_app.as_mut() {
            app.update(ctx, frame);
        }
    }
}
