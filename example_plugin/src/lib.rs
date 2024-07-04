use libspkg::plugin::{Plugin, PluginProperties};

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn properties(&self) -> PluginProperties {
        PluginProperties {
            name: "Example Plugin",
            id: "example-plugin",
            package_id: "com.example.exampleplugin",
            version: "0.1.0",
        }
    }

    fn execute(&self) {
        println!("Example spkg plugin")
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(ExamplePlugin)
}