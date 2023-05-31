use rand::Rng;
use rand::distributions::{Distribution, Standard};


use bytemuck::{Pod, Zeroable};
use wgpu::{RenderPipeline, BindGroup};
use std::{borrow::Cow, mem};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window, dpi::{LogicalSize, LogicalPosition},
};
use wgpu::util::DeviceExt;

use wave1::objs;

#[path ="./framework.rs"]
mod framework;

const WINDOW_WIDTH: f64 = 500.;

fn main() {
    let area = (WINDOW_WIDTH as f64, WINDOW_WIDTH as f64);

    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new()
                .with_decorations(true)
                .with_always_on_top(false)
                .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_WIDTH))
                .with_transparent(false);
    builder = builder.with_title("Bondings");
    let window = builder.build(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(framework::run(event_loop, window));
    }

}
