//! LocalGloss 的离线门面；不公开可注入网络或记录服务的核心 Engine。
mod engine;
mod error;
mod frame;
mod key;
mod outcome;
mod row;
mod settings;
mod term;

pub use engine::OfflineEngine;
pub use error::LoadError;
pub use frame::Frame;
pub use key::Key;
pub use outcome::Outcome;
pub use row::Row;
pub use settings::Settings;
pub use term::Term;
