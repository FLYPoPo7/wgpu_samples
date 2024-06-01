use eframe::egui;

use crate::apps::{hello_triangle, rotating_cube, textured_cube, two_cubes};

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
        "helloTriangleMSAA(WIP)",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "rotatingCube",
        AppType::BasicGraphics,
        |frame: &eframe::Frame| {
            Some(Box::new(
                rotating_cube::RotatingCube::new_with_render_state(
                    &frame.wgpu_render_state().unwrap(),
                )
                .unwrap(),
            ))
        },
    ),
    (
        "twoCubes",
        AppType::BasicGraphics,
        |frame: &eframe::Frame| {
            Some(Box::new(
                two_cubes::TwoCubes::new_with_render_state(&frame.wgpu_render_state().unwrap())
                    .unwrap(),
            ))
        },
    ),
    (
        "texturedCube",
        AppType::BasicGraphics,
        |frame: &eframe::Frame| {
            Some(Box::new(
                textured_cube::TexturedCube::new_with_render_state(
                    &frame.wgpu_render_state().unwrap(),
                )
                .unwrap(),
            ))
        },
    ),
    (
        "instancedCube(WIP)",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "fractalCube(WIP)",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    (
        "cubemap(WIP)",
        AppType::BasicGraphics,
        |_frame: &eframe::Frame| None,
    ),
    // WebGPU Features
    (
        "samplerParameters(WIP)",
        AppType::WebGPUFeatures,
        |_frame: &eframe::Frame| None,
    ),
    (
        "reversedZ(WIP)",
        AppType::WebGPUFeatures,
        |_frame: &eframe::Frame| None,
    ),
    (
        "renderBundles(WIP)",
        AppType::WebGPUFeatures,
        |_frame: &eframe::Frame| None,
    ),
    // GPGPU Demos
    (
        "computeBoids(WIP)",
        AppType::GPGPUDemos,
        |_frame: &eframe::Frame| None,
    ),
    (
        "gameOfLife(WIP)",
        AppType::GPGPUDemos,
        |_frame: &eframe::Frame| None,
    ),
    (
        "bitonicSort(WIP)",
        AppType::GPGPUDemos,
        |_frame: &eframe::Frame| None,
    ),
    // Graphics Techniques
    (
        "cameras(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "normalMap(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "shadowMapping(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "deferredRendering(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "particles(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "points(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "imageBlur(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "cornell(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "a-buffer(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "skinnedMesh(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "textRenderingMsdf(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    (
        "volumeRenderingTexture3D(WIP)",
        AppType::GraphicsTechniques,
        |_frame: &eframe::Frame| None,
    ),
    // Benchmarks
    (
        "animometer(WIP)",
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
