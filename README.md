# GeoConnect

A mobile app for American-Japanese cultural exchange. Users are able to click on Japan's 8 regions, read cultural and historical information about each one, and compare US vs. Japanese customs with each other. The app is being built with Dioxus (Rust) for the UI and a PostgreSQL database (hosted on Railway).

## Download the Android app

**[Download GeoConnect for Android](https://github.com/5quidL0rd/GeoConnect/releases/tag/android-v0.1.0)** — grab `GeoConnect.apk` from the release, open it on your phone, and allow "install unknown apps" when prompted (normal for a sideloaded app, not a sign of a problem).

## Repository

https://github.com/logerator/GeoConnect

## Requirements

- Rust + Cargo (stable toolchain)
- Dioxus CLI: `cargo install dioxus-cli --locked`
- A `.env` file in the project root containing `DATABASE_URL=<postgres connection string>` pointing at a database hosted on Railway (link included in submission)

Note: `DATABASE_URL` is read from `.env` at **build time** (by `build.rs`) and baked directly into the compiled binary, not read at runtime. This means `.env` only needs to exist on the machine that *builds* the app, not on the machine that later runs the installed app.

## Development Build & Run

cargo install dioxus-cli --locked
dx serve

## Production Release Build (Windows MSI installer)

This produces the signed, optimized `.msi` installer for distribution. Build this natively on Windows (not inside WSL/Linux) — the desktop app uses a native webview, which cross-compiles unreliably.

1. Install prerequisites on the Windows machine:
   - Rust: https://rustup.rs
   - Dioxus CLI: `cargo install dioxus-cli --locked`
2. Copy/clone this source tree onto that machine.
3. Create a `.env` file in the project root:
   ```
   DATABASE_URL=postgresql://user:password@host:port/dbname
   ```
4. Build the release installer:
   ```
   dx bundle --release --platform windows --package-types msi
   ```
   (The first run will automatically download the WiX Toolset needed to build the `.msi`.)
5. The finished installer will be at:
   ```
   target\dx\geoconnect\release\windows\app\bundle\msi\*.msi
   ```

## Database Schema

regions - the 8 regions of Japan (id, name, overview)
categories - content topics like Food, Etiquette, Festivals, History (id, name)
facts - cultural/historical content per region and category (id, region_id, category_id, title, body)
comparisons - US vs. Japanese custom comparisons, optionally tied to a region (id, category_id, region_id, title, us_practice, jp_practice, us_region, key_difference)
places - notable places to visit per region (id, region_id, name, kind, tagline, overview)
place_highlights - place specific information (id, place_id, category, title, body)
travel_prep - region-specific travel preparation guidance (id, region_id, category, title, body)
