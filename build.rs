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

    println!("cargo:rustc-env=DATABASE_URL={database_url}");
}
