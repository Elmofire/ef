pub fn check_supported(arch: &str, platform: &str) -> String {
    let mut combo = String::new();
    let supported = ["aarch64-apple-darwin", "aarch64-apple-ios", "aarch64-linux-android", "aarch64-unknown-linux-musl", "arm-linux-androideabi", "arm-unknown-linux-musleabi", "armv5te-unknown-linux-musleabi", "armv7-linux-androideabi", "armv7-unknown-linux-musleabi", "armv7-unknown-linux-musleabihf", "i586-unknown-linux-musl", "i686-linux-android", "i686-unknown-linux-musl", "mips-unknown-linux-musl", "mips64-unknown-linux-muslabi64", "mips64el-unknown-linux-muslabi64", "mipsel-unknown-linux-musl", "x86_64-apple-darwin", "x86_64-apple-ios", "x86_64-linux-android", "x86_64-pc-solaris", "x86_64-pc-windows-gnu", "x86_64-sun-solaris", "x86_64-unknown-freebsd", "x86_64-unknown-illumos", "x86_64-unknown-linux-musl", "x86_64-unknown-netbsd"];
    for comb in supported {
        if comb.contains(arch) && comb.contains(platform) {
            if &platform == &"linux" {
                let linval = format!("unknown-{}", platform);
                if comb.contains(arch) && comb.contains(&linval) {
                    combo.push_str(&comb.to_string());
                    break
                }
            }
            if &platform == &"android" {
                let andval = format!("linux-{}", platform);
                if comb.contains(arch) && comb.contains(&andval) {
                    if &arch == &"arm" || &arch == &"armv7" {
                        let armval = format!("{}-linux", arch);
                        if comb.contains(&armval) && comb.contains(&andval) {
                            combo.push_str(&comb.to_string());
                            break  
                        }
                    }
                    combo.push_str(&comb.to_string());
                    break
                }
            }
            if &platform == &"windows" {
                if comb.contains(arch) && comb.contains(&platform) {
                    combo.push_str(&comb.to_string());
                    break
                }
            }
            if &platform == &"ios" || &platform == &"darwin" {
                let appval = format!("apple-{}", platform);
                if comb.contains(arch) && comb.contains(&appval) {
                    combo.push_str(&comb.to_string());
                }
            }
        }
    }
    return combo
}