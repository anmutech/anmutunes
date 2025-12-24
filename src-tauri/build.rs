fn main() {
    tauri_build::build();

    // TODO: what about other target_os? or platform like deb aur rpm?
    #[cfg(target_os = "macos")]
    {
        use std::env;
        use std::path::Path;
        // TODO: copy respective libs into target folder as well for execution
        // TODO: support x86_64 and aarch64
        // TODO: copy vlc plugins folder into target as well, so we can execute without the user having to download VLC.app
        //      Does VLC license allow to redistribute?
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir)
                .join("vlc")
                .join("macos")
                .join("lib")
                .display()
        );
        // Ensures the app searches for libvlc in its own resources folder ($ORIGIN on other platforms?)
        #[cfg(not(debug_assertions))]
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../Resources/vlc/macos/lib");
        #[cfg(debug_assertions)]
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/vlc/macos/lib");
        // TODO: for debug build the contents of libvlc should be next to the anmutunes binary
    }
}
