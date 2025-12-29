use nix::unistd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = unistd::getpid();
    let cwd = unistd::getcwd()?;
    let rootfs = cwd.join("rootfs");
    println!("Hello, world! my PID is {}", p);
    match unistd::chroot(&rootfs) {
        Ok(_) => println!(
            "Succesfully changed root filesystem to: {}",
            rootfs.display()
        ),
        Err(e) => println!(
            "Something went wrong attempting to chroot to fs: {}\n Error: {}",
            rootfs.display(),
            e
        ),
    };
    Ok(())
}
