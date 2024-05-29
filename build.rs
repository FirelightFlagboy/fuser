fn main() {
    #[cfg(all(not(feature = "libfuse"), not(target_os = "linux")))]
    unimplemented!("Building without libfuse is only supported on Linux");

    #[cfg(feature = "libfuse")]
    {
        #[cfg(target_os = "macos")]
        {
            #[cfg(not(feature = "macos-use-fuse-t"))]
            {
                if pkg_config::Config::new()
                    .atleast_version("2.6.0")
                    .probe("fuse") // for macFUSE 4.x
                    .map_err(|e| eprintln!("{}", e))
                    .is_ok()
                {
                    println!("Using macFUSE 4.x for macos");
                    println!("cargo:rustc-cfg=feature=\"libfuse2\"");
                } else {
                    pkg_config::Config::new()
                        .atleast_version("2.6.0")
                        .probe("osxfuse") // for osxfuse 3.x
                        .map_err(|e| eprintln!("{}", e))
                        .unwrap();
                    println!("Using osxfuse 3.x for macos");
                    println!("cargo:rustc-cfg=feature=\"libfuse2\"");
                }
            }
            #[cfg(feature = "macos-use-fuse-t")]
            {
                pkg_config::Config::new()
                    .atleast_version("1.0.38")
                    .probe("fuse-t")
                    .map_err(|e| eprintln!("{e}"))
                    .unwrap();
                println!("Using fuse-t 1.x for macos");
                println!("cargo:rustc-cfg=feature=\"libfuse2\"");
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            // First try to link with libfuse3
            if pkg_config::Config::new()
                .atleast_version("3.0.0")
                .probe("fuse3")
                .map_err(|e| eprintln!("{e}"))
                .is_ok()
            {
                println!("cargo:rustc-cfg=feature=\"libfuse3\"");
            } else {
                // Fallback to libfuse
                pkg_config::Config::new()
                    .atleast_version("2.6.0")
                    .probe("fuse")
                    .map_err(|e| eprintln!("{e}"))
                    .unwrap();
                println!("cargo:rustc-cfg=feature=\"libfuse2\"");
            }
        }
    }
}
