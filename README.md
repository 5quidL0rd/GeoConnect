# GeoConnect

A mobile and desktop app for American-Japanese cultural exchange. Users can click on Japan's 8 regions, read cultural and historical information about each one, and compare US vs. Japanese customs side by side. The app is built with Dioxus (Rust) for the UI and a PostgreSQL database hosted on Railway.

## Downloads

Grab the latest installers from the [Releases page](https://github.com/5quidL0rd/GeoConnect/releases):

- **Android APK** — download `GeoConnect.apk`, open it on your phone, and allow "install unknown apps" when prompted (normal for a sideloaded app, not a sign of a problem).
- **Windows MSI** — download the `.msi` installer and run it.

## Architecture

The app connects **directly to Postgres** using a dedicated least-privilege role
(`geoconnect_readonly`) that can only SELECT from the app's content tables — no
INSERT, UPDATE, DELETE, or DDL. That role's connection string is compiled into
the binary at build time by design: anyone can extract it from a shipped
APK/MSI, so it is deliberately scoped to be harmless (worst case, someone can
read the same public content the app already shows). `build.rs` refuses to
build a binary with any other database credential.

This is a known tradeoff, not a security feature: a distributed binary that
talks straight to the database can never hold a truly private credential. A
server-side API layer is planned to remove the direct database connection
entirely.

## Build from source

### Requirements

- Rust + Cargo (stable toolchain)
- Dioxus CLI: `cargo install dioxus-cli --locked`
- A `.env` file in the project root containing
  `DATABASE_URL=<geoconnect_readonly postgres connection string>`

Note: `DATABASE_URL` is read from `.env` at **build time** (by `build.rs`) and
baked directly into the compiled binary, not read at runtime. `.env` only needs
to exist on the machine that *builds* the app, not on the machine that later
runs it. The build fails unless the URL uses the `geoconnect_readonly` role, so
an admin credential can't end up inside a distributable binary. Admin
credentials belong in `.env.admin` (gitignored, never read by the build).

### Development build & run

```
cargo install dioxus-cli --locked
dx serve
```

### Android APK

```
dx bundle --release --platform android
scripts/patch_android_icon.sh   # dx doesn't wire the bundle icon through to Android yet
dx bundle --release --platform android   # repackage with the patched icon
```

### Windows MSI installer

Build natively on Windows (not inside WSL/Linux) — the desktop app uses a
native webview, which cross-compiles unreliably.

1. Install prerequisites on the Windows machine:
   - Rust: https://rustup.rs
   - Dioxus CLI: `cargo install dioxus-cli --locked`
2. Copy/clone this source tree onto that machine.
3. Create the `.env` file as described above.
4. Build the release installer:
   ```
   dx bundle --release --platform windows --package-types msi
   ```
   (The first run automatically downloads the WiX Toolset needed to build the `.msi`.)
5. The finished installer will be at:
   ```
   target\dx\geoconnect\release\windows\app\bundle\msi\*.msi
   ```

## Database Schema

- `regions` — the regions of Japan (id, name, overview)
- `categories` — content topics like History, Food & Dining, Etiquette (id, name)
- `facts` — cultural/historical content per region and category (id, region_id, category_id, title, body)
- `comparisons` — US vs. Japanese custom comparisons per region (id, category_id, region_id, title, us_practice, jp_practice, us_region, key_difference)
- `places` — notable places to visit per region (id, region_id, name, kind, tagline, overview)
- `place_highlights` — highlights for each place (id, place_id, category, title, body)
- `travel_prep` — region-specific travel preparation guidance (id, region_id, category, title, body)

The read-only role used by shipped binaries is created by
`scripts/readonly_role.sql` (run it as the database admin; set the role's
password separately so no credential lives in the repo).
