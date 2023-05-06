pub mod errors;
pub mod settings;
pub mod startup;
pub mod store;
pub mod telemetry;
pub mod types;
pub mod utils;

pub static ENV: once_cell::sync::Lazy<minijinja::Environment<'static>> =
    once_cell::sync::Lazy::new(|| {
        let mut env = minijinja::Environment::new();
        env.set_source(minijinja::Source::from_path("templates"));
        env
    });
