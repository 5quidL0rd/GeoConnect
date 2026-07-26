use dioxus::prelude::*;
mod map;
mod regions;
mod flowchart;
mod place;
mod place_nav;
mod loading;
mod back_button;
mod compare;
mod travel_prep;
use map::JapanMap;
use place::Place;
mod hokkaido;
use hokkaido::Hokkaido;
mod tohoku;
use tohoku::Tohoku;
mod chubu;
use chubu::Chubu;
mod kanto;
use kanto::Kanto;
mod kansai;
use kansai::Kansai;
mod chugoku;
use chugoku::Chugoku;
mod shikoku;
use shikoku::Shikoku;
mod kyushu;
use kyushu::Kyushu;
static DB_POOL: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/map/:id")]
    Map { id: i32 },
    #[route("/place/:id")]
    Place { id: i32 },
    #[route("/hokkaido")]
    Hokkaido {},
    #[route("/tohoku")]
    Tohoku {},
    #[route("/chubu")]
    Chubu {},
    #[route("/kanto")]
    Kanto {},
    #[route("/kansai")]
    Kansai {},
    #[route("/chugoku")]
    Chugoku {},
    #[route("/shikoku")]
    Shikoku {},
    #[route("/kyushu")]
    Kyushu {},
}


const FAVICON: Asset = asset!("/assets/favicon.ico");

const STYLES_CSS: Asset = asset!("/assets/styles.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[tokio::main]
async fn main() {
    let database_url = env!("DATABASE_URL");

    let pool = sqlx::PgPool::connect(database_url)
        .await
        .expect("Database connection failed");

    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Test query failed");
    println!("Test query returned: {}", row.0);

    DB_POOL.set(pool).expect("Pool already initialized");
    println!("Connected to database!");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: STYLES_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            span { class: "hero-emoji", "🌍" }
            h1 { "GeoConnect" }
            p { class: "hero-tagline", "Understand Japan before you go" }

            div { id: "links",
                div { class: "info-item",
                    span { class: "info-icon", "📚" }
                    span { "Learn the WHY in Japanese culture" }
                }
                div { class: "info-item",
                    span { class: "info-icon", "👺" }
                    span { "Explore different Region's Distinct Culture" }
                }
                div { class: "info-item",
                    span { class: "info-icon", "👹" }
                    span { "Prepare for Life in Japan" }
                }
                div { class: "info-item",
                    span { class: "info-icon", "🎌" }
                    span { "Minimize Culture Shock" }
                }
            }

            Link { class: "cta-button", to: Route::Map { id: 1 }, "Go to the Map" }
        }
    }
}

// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

// Map page
#[component]
pub fn Map(id: i32) -> Element {
    rsx! {
        div { id: "Map", JapanMap {} }
    }
}

// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
        }
        Outlet::<Route> {}
    }
}