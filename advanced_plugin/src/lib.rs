use lazy_static::lazy_static;
use libspkg::plugin::{Plugin, PluginProperties};

lazy_static! {
    pub static ref PROPERTIES: PluginProperties = PluginProperties {
        name: "Advanced Plugin",
        id: "advanced-plugin",
        package_id: "com.example.advancedplugin",
        version: env!("CARGO_PKG_VERSION"),
        library_version: libspkg::VERSION,
    };
}

pub struct AdvancedPlugin;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Test,
    Foo,
    Help
}

impl Plugin for AdvancedPlugin {
    fn execute(&self, args: &[String]) {
        match self.parse_args(args) {
            Command::Test => self.test(),
            Command::Foo => self.foo(),
            Command::Help => self.help()
        }
    }

    fn help(&self) {
        println!("Example help message")
    }
}

impl AdvancedPlugin {
    pub fn parse_args(&self, args: &[String]) -> Command {
        match args.first().unwrap().as_str() {
            "test" => Command::Test,
            "foo" => Command::Foo,
            _ => Command::Help
        }
    }
    
    pub fn foo(&self) {
        println!("Bar!")
    }

    pub fn test(&self) {
        println!("You are using an extended plugin which is based on {} of libspkg", PROPERTIES.library_version)
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn create_plugin() -> (Box<dyn Plugin>, PluginProperties) {
    (Box::new(AdvancedPlugin), *PROPERTIES)
}