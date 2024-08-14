use libspkg::plugin::{Plugin, PluginProperties};

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn execute(&self, args: &[String]) {
        match args.first().unwrap().as_str() {
            "test" => {
                println!("todo")
            },
            "foo" => {
                println!("todo")
            }
            _ => self.help()
        }
    }

    fn help(&self) {
        println!("todo")
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn create_plugin() -> (Box<dyn Plugin>, PluginProperties) {
    let properties: PluginProperties = PluginProperties {
        name: "StrawberryOS",
        id: "strawberryos",
        package_id: "org.strawberryfoundations.spkg.plugins.strawberryos",
        version: env!("CARGO_PKG_VERSION"),
        library_version: libspkg::VERSION,
    };

    (Box::new(ExamplePlugin), properties)
}