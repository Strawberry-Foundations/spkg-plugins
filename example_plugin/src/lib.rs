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

    fn execute(&self, args: &[String]) {
        match args.first().unwrap().as_str() {
            "test" => {
                println!("Example spkg plugin")
            },
            "foo" => {
                println!("Bar!")
            }
            _ => self.help()
        }
    }

    fn help(&self) {
        println!("Example help message")
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(ExamplePlugin)
}