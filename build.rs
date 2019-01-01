use std::process::Command;
use std::path::Path;

#[cfg(target_os = "freebsd")]
fn freebsd_nop() {

    Command::new("cc").args(&["native/cap_dns_wrap.c", "-c", "-fPIC", "-o", "native/cap_dns_wrap.o"])
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libcap_dns_wrap.a", "cap_dns_wrap.o"])
                      .current_dir(&Path::new("./native"))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", "./native");
    println!("cargo:rustc-link-lib=static=cap_dns_wrap");
}

#[cfg(not(target_os = "freebsd"))]
fn freebsd_nop() {
    panic!("This is a FreeBSD only crate. It will not compile on other OSes.");
}

fn main() {
    freebsd_nop();
}
