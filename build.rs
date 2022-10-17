use chrono::prelude::*;

fn main() {
    /*
      build info:
        version (1.0.0)
        commit (hash)
        build timestamp (time)

    */
    let dt = Local::now();
    println!("cargo:rustc-env=BUILD_INFO={}", dt.format("%Y-%m-%dT%H:%M:%S%.f%:z"));
}
