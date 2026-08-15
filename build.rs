fn main() {
    println!("cargo:rerun-if-changed=.env");

    let database_url = dotenvy::from_filename_iter(".env")
        .ok()
        .and_then(|iter| {
            iter.flatten()
                .find(|(key, _)| key == "DATABASE_URL")
                .map(|(_, value)| value)
        })
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .expect("DATABASE_URL not set in .env or environment");

    // GUARD: DATABASE_URL is compiled verbatim into the binary, and our
    // binaries are distributed publicly (APK/MSI). Anyone can extract this
    // string with `strings`. Refuse to build with anything other than the
    // least-privilege read-only role, so an admin credential can never be
    // baked into a distributable again.
    if !database_url.contains("geoconnect_readonly") {
        panic!(
            "\n\nDATABASE_URL does not use the 'geoconnect_readonly' role.\n\
             This URL is compiled INTO the binary and is extractable from any\n\
             shipped APK/MSI with `strings`. Building with an admin credential\n\
             would leak it to everyone who downloads a release (this happened\n\
             once already). Put the geoconnect_readonly connection string in\n\
             .env; keep the admin credential only in .env.admin (gitignored,\n\
             never read by the build).\n"
        );
    }

    println!("cargo:rustc-env=DATABASE_URL={database_url}");
}
