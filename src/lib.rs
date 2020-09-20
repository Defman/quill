#![feature(const_type_name)]

pub use quill_macros::*;

pub mod layout;
pub mod internal;
use layout::*;

pub struct Spawner<T>(T);
pub struct Query<T>(T);

pub struct CommandBuilder;

impl PluginBuilder {
    pub fn system<Args, F: SystemLayout<Args>>(&mut self, _: F) -> &mut Self {
        self.systems.push(&F::LAYOUT);
        self
    }

    pub fn setup<T>(&mut self, _: T) -> &mut Self {
        self
    }

    pub fn command<T>(&mut self, _: T) -> &mut Self {
        self
    }

    pub fn build(self) -> Plugin {
        Plugin {
            systems: self.systems,
            name: self.name.expect("missing plugin name"),
        }
    }
}


#[derive(Default)]
pub struct PluginBuilder {
    pub(crate) systems: Vec<&'static [&'static Layout]>,
    pub(crate) name: Option<&'static str>,
}

pub struct Plugin {
    pub(crate) systems: Vec<&'static [&'static Layout]>,
    pub(crate) name: &'static str,
}