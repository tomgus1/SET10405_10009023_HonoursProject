use std::fs;
use std::path::Path;

fn main() {
    let lib_dir = Path::new("lib");
    if !lib_dir.exists() {
        let _ = fs::create_dir_all(lib_dir);
    }

    let symlink_path = lib_dir.join("libxkbcommon-x11.so");
    if !symlink_path.exists() {
        let sys_target = Path::new("/usr/lib64/libxkbcommon-x11.so.0");
        if sys_target.exists() {
            #[cfg(unix)]
            let _ = std::os::unix::fs::symlink(sys_target, &symlink_path);
        }
    }

    println!("cargo:rustc-link-search=native=lib");
}
