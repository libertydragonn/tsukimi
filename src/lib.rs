use std::{
    env,
    sync::LazyLock,
};

mod app;
mod arg;
mod config;
mod gstl;
mod macros;
#[cfg(target_os = "linux")]
mod mpris_common;
mod ui;
mod utils;

pub mod client;

pub use arg::Args;
use clap::Parser;
pub use config::*;
use gettextrs::*;
use gtk::prelude::*;

pub use ui::Window;

pub use app::TsukimiApplication as Application;

use crate::{
    client::runtime::runtime,
    ui::widgets,
};

pub static USER_AGENT: LazyLock<String> =
    LazyLock::new(|| format!("{}/{} - {}", CLIENT_ID, version(), env::consts::OS));

pub const APP_ID: &str = "moe.tsuna.tsukimi";
pub const CLIENT_ID: &str = "Tsukimi";
const APP_RESOURCE_PATH: &str = "/moe/tsuna/tsukimi";
const GRESOURCE_FILE: &str = "tsukimi.gresource";

// Windows: resolve paths relative to the exe (<prefix>/bin/tsukimi.exe -> <prefix>)
// so the bundle stays portable.
#[cfg(windows)]
fn exe_prefix_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .expect("Can not get exe path")
        .ancestors()
        .nth(2)
        .expect("Can not get exe prefix dir")
        .to_path_buf()
}

pub fn run() -> gtk::glib::ExitCode {
    Args::parse().init();

    // Initialize gettext
    setlocale(LocaleCategory::LcAll, String::new());
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Failed to set textdomain codeset");
    #[cfg(not(windows))]
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Invalid argument passed to bindtextdomain");
    #[cfg(windows)]
    bindtextdomain(GETTEXT_PACKAGE, exe_prefix_dir().join("share").join("locale"))
        .expect("Invalid argument passed to bindtextdomain");

    textdomain(GETTEXT_PACKAGE).expect("Invalid string passed to textdomain");

    adw::init().expect("Failed to initialize Adwaita");
    register_gio_resources();

    widgets::init();

    gtk::glib::set_application_name(CLIENT_ID);

    let _tokio_guard = runtime().enter();
    Application::new().run_with_args::<&str>(&[])
}

fn register_gio_resources() {
    #[cfg(not(windows))]
    let path = std::path::Path::new(PKGDATADIR).join(GRESOURCE_FILE);
    #[cfg(windows)]
    let path = exe_prefix_dir()
        .join("share")
        .join("tsukimi")
        .join(GRESOURCE_FILE);
    let resources = gtk::gio::Resource::load(path).expect("Failed to load resources.");
    gtk::gio::resources_register(&resources);
}
