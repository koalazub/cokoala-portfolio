use crate::wasm_bindgen::prelude::*;
use crate::web_sys::{window, HtmlCanvasElement};
use leptos::prelude::*;
use wgpu::{self, WindowHandle};

#[component]
pub fn Home() -> impl IntoView {
    Effect::new(move |_| {});

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="container">
                <picture>
                    <source
                        srcset="https://avatars.githubusercontent.com/u/7111524?v=4"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://avatars.githubusercontent.com/u/7111524?v=4"
                        alt="cokoala Logo"
                        height="100"
                        width="100"
                    />
                </picture>
                <h1>"Hello? Is this thing on?"</h1>
                <div class="webgpu-container">
                    <h2>"WebGPU Demo"</h2>
                </div>
            </div>
        </ErrorBoundary>
    }
}

async fn init_webgpu() -> Result<(), JsValue> {
    // Get the canvas
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("No document found"))?;
    let canvas = document
        .get_element_by_id("webgpu-canvas")
        .ok_or_else(|| JsValue::from_str("Canvas not found"))?;
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    // Create the instance and surface
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = instance
        .create_surface(&window)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Request adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| JsValue::from_str("Failed to get GPU adapter"))?;

    // Request device
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Configure the surface
    let size = wgpu::Extent3d {
        width: canvas.width(),
        height: canvas.height(),
        depth_or_array_layers: 1,
    };

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    // Create and submit command encoder
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let texture = surface
        .get_current_texture()
        .map_err(|e| JsValue::from_str(&format!("Failed to get current texture: {:?}", e)))?;

    let view = texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
    }

    queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

async fn request_adapter() -> Result<wgpu::Adapter, JsValue> {
    wgpu::Instance::new(wgpu::InstanceDescriptor::default())
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| JsValue::from_str("Failed to get GPU adapter"))
}

async fn request_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), JsValue> {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                label: None,
            },
            None,
        )
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

async fn create_surface(
    canvas: &HtmlCanvasElement,
    adapter: &wgpu::Adapter,
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> Result<(wgpu::SurfaceConfiguration, wgpu::Surface), JsValue> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    // Create a window handle from the canvas
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let window_handle = wgpu::WindowHandle::new(window);

    let surface = instance
        .create_surface(SurfaceTarget::Window(Box::new(window_handle)))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let capabilities = surface.get_capabilities(adapter);
    let size = wgpu::Extent3d {
        width: canvas.width(),
        height: canvas.height(),
        depth_or_array_layers: 1,
    };

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: capabilities.formats[0],
        width: size.width,
        height: size.height,
        present_mode: capabilities.present_modes[0],
        alpha_mode: capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(device, &config);

    Ok((config, surface))
}
