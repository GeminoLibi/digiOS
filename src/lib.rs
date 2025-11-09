pub mod core;
pub mod action;
pub mod vision;
pub mod state;
pub mod task;
pub mod event;
pub mod memory;
pub mod api;
pub mod boot;
pub mod model;
pub mod self_improve;
pub mod human_interface;
pub mod interaction;

pub use core::aios::aiOS;
pub use boot::InitSystem;
pub use interaction::InteractionManager;
pub use interaction::ToolManager;

