use nix::unistd;

fn main() {
    let p = unistd::getpid();
    let rootfs = "/Users/wyattlamberth/dev/parallax/rootfs/";
    println!("Hello, world! my PID is {}", p);
    match unistd::chroot(rootfs) {
        Ok(_) => println!("Succesfully changed root filesystem to: {}", rootfs),
        Err(e) => println!(
            "Something went wrong attempting to chroot to fs: {}\n Error: {}",
            rootfs, e
        ),
    };
}
