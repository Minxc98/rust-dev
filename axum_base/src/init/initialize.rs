use tracing::{info, level_filters::LevelFilter as Level};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
pub fn init() {
    init_log();
}
pub fn init_log() {
    let layer = Layer::new().with_filter(Level::INFO);
    tracing_subscriber::registry().with(layer).init();
}
