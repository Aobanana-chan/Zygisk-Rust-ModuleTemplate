mod api;
mod binding;
#[doc(hidden)]
pub mod macros;
mod module;

#[macro_use] extern crate log;
extern crate android_logger;

use android_logger::Config;
use log::LevelFilter;

pub use api::ZygiskApi;
pub use binding::{AppSpecializeArgs, ServerSpecializeArgs, StateFlags, ZygiskOption, API_VERSION};
pub use module::ZygiskModule;
use jni::JNIEnv;

fn native_activity_create() {
    android_logger::init_once(
    Config::default().with_max_level(LevelFilter::Trace));
    trace!(target:"zygisk_template","this is a verbose {}", "message");
    error!(target:"zygisk_template","this is printed by default");
}

mod test {
    use std::os::unix::io::RawFd;
    use crate::MyModule;

    static MODULE: MyModule = MyModule{};
    crate::zygisk_module!(&MODULE);

    fn companion(_socket: RawFd) {}
    crate::zygisk_companion!(companion);
}

struct MyModule{}

impl ZygiskModule for MyModule{
    fn on_load(&self, api: ZygiskApi, env: JNIEnv) {
        native_activity_create();
        info!(target:"zygisk_template","HelloWorld!");
    }
}