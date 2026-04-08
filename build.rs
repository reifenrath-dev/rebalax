use leptos_i18n_build::{Config, TranslationsInfos};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let i18n_mod_directory = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("i18n");

    let config = Config::new("en")?.add_locale("de")?;
    let translations_infos = TranslationsInfos::parse(config)?;

    translations_infos.rerun_if_locales_changed();

    translations_infos.generate_i18n_module(i18n_mod_directory)?;

    Ok(())
}
