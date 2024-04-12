use clap::Parser;

#[derive(Debug, Parser)]
pub enum Args {
    Reset,
    Get,
    Set { value: f32 },
    ZoomIn { delta: f32 },
    ZoomOut { delta: f32 },
}
