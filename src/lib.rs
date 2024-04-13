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
    use std::{os::unix::io::RawFd, ptr};
    use crate::MyModule;

    static mut MODULE: MyModule = MyModule{ env: None };
    crate::zygisk_module!(ptr::addr_of_mut!(MODULE));  // https://github.com/rust-lang/rust/issues/114447

    fn companion(_socket: RawFd) {}
    crate::zygisk_companion!(companion);
}

struct MyModule{
    env: Option<JNIEnv<'static>>
}

impl ZygiskModule for MyModule{
    fn on_load(&mut self, api: ZygiskApi, env: JNIEnv<'static>) {
        self.env = Some(env);
        native_activity_create();
        info!(target:"zygisk_template","HelloWorld!");
    }

    fn pre_app_specialize(&mut self, api: ZygiskApi, args: &mut AppSpecializeArgs) {
        if let Some(env) = &mut self.env{
            let app_name: String = env.get_string(&args.nice_name).unwrap().into();
            info!("Run on {}", app_name);
        }
    }
}