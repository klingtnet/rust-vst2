extern crate vst2;

use std::sync::{Arc, Mutex};
use std::path::Path;
use std::error::Error;

use vst2::host::{Host, PluginLoader};
use vst2::plugin::Plugin;

struct SampleHost;

impl Host for SampleHost {
    fn automate(&mut self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

fn main() {
    // This is an example of a plugin being loaded. Change this to the appropriate path.
    let path = Path::new("/Library/Audio/Plug-Ins/VST/beyerdynamicVS.vst/Contents/MacOS/beyerdynamicVS");

    // Create the host
    let host = Arc::new(Mutex::new(SampleHost));

    println!("Loading {}...", path.to_str().unwrap());

    // Load the plugin
    let mut loader = PluginLoader::load(path, host.clone())
        .unwrap_or_else(|e| panic!("Failed to load plugin: {}", e.description()));

    // Create an instance of the plugin
    let mut instance = loader.instance().unwrap();

    // Get the plugin information
    let info = instance.get_info();

    println!("Loaded '{}':\n\t\
              Vendor: {}\n\t\
              Presets: {}\n\t\
              Parameters: {}\n\t\
              VST ID: {}\n\t\
              Version: {}\n\t\
              Initial Delay: {} samples",
             info.name,
             info.vendor,
             info.presets,
             info.parameters,
             info.unique_id,
             info.version,
             info.initial_delay);

    // Initialize the instance
    instance.init();
    println!("Initialized instance!");

    println!("Closing instance...");
    // Close the instance. This is not necessary as the instance is shut down when
    // it is dropped as it goes out of scope.
    // drop(instance);
}
