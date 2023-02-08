use clap::{Arg,App};
struct ElmoOptions {
    listener: String,
    output: String,
    os: String,
    arch: String,
    shell: String
}

impl ElmoOptions {
    fn new(lstnr: &str, otpt: &str, ops: &str, arc: &str, shll: &str) -> ElmoOptions {
        ElmoOptions {
            listener: lstnr.to_string(),
            output: otpt.to_string(),
            os: ops.to_string(),
            arch: arc.to_string(),
            shell: shll.to_string()
        }
    }
}

pub fn getargs() -> String {
    let args = App::new("🔥ElmoFire🔥")
	.version("0.1.0")
	.about("Description: Yet another obfuscated payload generator written in Rust")
	.author("Author: M4x 5yn74x")
    .usage("./ef -p <OS-Option> -a <Architecture> -l <Listener> -s <Shell-Type> -o <Output filename>")
	.args(&[
		Arg::new("OS-Option")
			.short('p')
            .takes_value(true)
            .help("Operating System: windows, linux, darwin, ios, android, solaris, sun-solaris, freebsd, netbsd, illumos"),
        Arg::new("Architecture")
            .short('a')
            .takes_value(true)
            .help("Architecture: x86_64, i586, i686, arm, armv5te, armv7, aarch64, mips, mipsel, mips64, mips64el"),
        Arg::new("Listener")
            .short('l')
            .takes_value(true)
            .help("Listening host: <listening ip:port>"),
        Arg::new("Shell-Type")
			.short('s')
            .takes_value(true)
            .help("Shell type: cmd, powershell, /bin/bash, /bin/sh, /system/bin/sh, /bin/busybox, /usr/bin/zsh"),
        Arg::new("Output")
            .short('o')
            .takes_value(true)
            .help("Output filename: <anything goes>"),
	]).get_matches();
    let (l,o,p,a,s) = (args.value_of("Listener").unwrap(),args.value_of("Output").unwrap(),args.value_of("OS-Option").unwrap(),args.value_of("Architecture").unwrap(),args.value_of("Shell-Type").unwrap());
    let op = ElmoOptions::new(l,o,p,a,s);
    println!("OS: {}\nArch: {}\nListener: {}\nShell: {}\nOutput: {}",op.os,op.arch,op.listener,op.shell,op.output);

    return format!("{} {} {} {} {}",op.os,op.arch,op.listener,op.shell,op.output)
}