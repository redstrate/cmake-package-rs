use itertools::chain;
use scopeguard::{guard, ScopeGuard};
use std::{iter::once, path::PathBuf};

pub fn use_cmake(name: &str) -> ScopeGuard<(), impl FnOnce(())> {
    let path = std::env::var("PATH").expect("Path not set?");
    let cmake_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("common")
        .join(name);
    assert!(cmake_path.exists());
    assert!(cmake_path.is_dir());

    // Prepend the custom cmake path to the PATH in platform-independent way
    let modfied_path = std::env::join_paths(chain!(
        once(PathBuf::from(cmake_path)),
        std::env::split_paths(&path)
    ))
    .unwrap();

    std::env::set_var("PATH", modfied_path);

    guard((), |_| {
        std::env::set_var("PATH", path);
    })
}

pub fn set_outdir() -> ScopeGuard<(), impl FnOnce(())> {
    std::env::set_var("OUT_DIR", std::env::temp_dir());

    guard((), |_| {
        std::env::remove_var("OUT_DIR");
    })
}

pub enum Profile {
    Debug,
    Release
}

pub fn set_profile(profile: Profile) -> ScopeGuard<(), impl FnOnce(())> {
    std::env::set_var("PROFILE", match profile {
        Profile::Debug => "debug",
        Profile::Release => "release"
    });

    guard((), |_| {
        std::env::remove_var("PROFILE");
    })
}

pub fn set_opt_level(opt_level: &str) -> ScopeGuard<(), impl FnOnce(())> {
    std::env::set_var("OPT_LEVEL", opt_level);

    guard((), |_| {
        std::env::remove_var("OPT_LEVEL");
    })
}

pub fn enable_debug() -> ScopeGuard<(), impl FnOnce(())> {
    std::env::set_var("DEBUG", "full");

    guard((), |_| {
        std::env::remove_var("DEBUG");
    })
}