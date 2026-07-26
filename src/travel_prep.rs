use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct TravelPrepItem {
    pub category: String,
    pub title: String,
    pub body: String,
}

pub async fn fetch_travel_prep(region_id: i32) -> Vec<TravelPrepItem> {
    let pool = crate::DB_POOL.get().expect("pool not initialized");
    sqlx::query_as("SELECT category, title, body FROM travel_prep WHERE region_id = $1 ORDER BY id")
        .bind(region_id)
        .fetch_all(pool)
        .await
        .expect("travel_prep query failed")
}

/// Toggleable "Travel Preparation" section for a region page. Renders
/// pre-fetched rows from the `travel_prep` table so visitors know what to
/// actually pack and plan for in this specific region — packing for
/// Okinawa and Hokkaido looks nothing alike.
#[component]
pub fn TravelPrep(items: Vec<TravelPrepItem>) -> Element {
    let mut show = use_signal(|| false);

    rsx! {
        div {
            class: "prep-callout",
            button {
                class: "prep-btn",
                onclick: move |_| show.toggle(),
                span { class: "prep-btn-icon", "🎒" }
                "Travel Preparation"
            }
            p {
                class: "prep-blurb",
                "What to pack, when to go, and how to get around this specific region."
            }
        }

        if show() {
            div {
                class: "prep-section",
                h2 { "Getting Ready for This Region" }

                div {
                    class: "prep-grid",
                    for item in items {
                        div {
                            class: "prep-card",
                            span { class: "prep-category", "{item.category}" }
                            h3 { "{item.title}" }
                            p { "{item.body}" }
                        }
                    }
                }
            }
        }
    }
}
