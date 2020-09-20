use crate::{Plugin};

#[repr(C)]
pub struct PluginContainer {
    name: &'static str,
}

impl From<Plugin> for PluginContainer {
    fn from(plugin: Plugin) -> Self {
        PluginContainer {
            name: plugin.name,
        }
    }
}