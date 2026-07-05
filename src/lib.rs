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
pub use config::GETTEXT_PACKAGE;
use config::{
    LOCALEDIR,
    PKGDATADIR,
    version,
};
use once_cell::sync::OnceCell;

use clap::Parser;
use gettextrs::*;
use gtk::prelude::*;

pub use ui::Window;

pub use app::TsukimiApplication as Application;

use crate::ui::widgets;

pub static USER_AGENT: LazyLock<String> =
    LazyLock::new(|| format!("{}/{} - {}", CLIENT_ID, version(), env::consts::OS));

pub const APP_ID: &str = "moe.tsuna.tsukimi";
pub const CLIENT_ID: &str = "Tsukimi";
const APP_RESOURCE_PATH: &str = "/moe/tsuna/tsukimi";
const GRESOURCE_FILE: &str = "tsukimi.gresource";

pub fn locale_dir() -> &'static str {
    static FLOCALEDIR: OnceCell<&'static str> = OnceCell::new();
    FLOCALEDIR.get_or_init(|| {
        #[cfg(not(windows))]
        {
            LOCALEDIR
        }
        #[cfg(windows)]
        {
            let locale_path = exe_prefix_dir().join("share").join("locale");
            Box::leak(locale_path.into_boxed_path())
                .to_str()
                .expect("Can not get locale dir")
        }
    })
}

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
    bindtextdomain(GETTEXT_PACKAGE, locale_dir())
        .expect("Invalid argument passed to bindtextdomain");

    textdomain(GETTEXT_PACKAGE).expect("Invalid string passed to textdomain");

    adw::init().expect("Failed to initialize Adwaita");
    register_gio_resources();

    widgets::init();

    // Initialize the GTK application
    gtk::glib::set_application_name(CLIENT_ID);

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
