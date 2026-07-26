use dioxus::prelude::*;
use crate::loading::Loading;
use crate::back_button::BackButton;

#[derive(Debug, Clone, sqlx::FromRow)]
struct PlaceInfo {
    name: String,
    kind: String,
    tagline: String,
    overview: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct Highlight {
    category: String,
    title: String,
    body: String,
}

#[component]
pub fn Place(id: i32) -> Element {
    let place = use_resource(move || async move {
        let pool = crate::DB_POOL.get().expect("pool not initialized");
        let result: PlaceInfo = sqlx::query_as(
            "SELECT name, kind, tagline, overview FROM places WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .expect("place query failed");
        result
    });

    let highlights = use_resource(move || async move {
        let pool = crate::DB_POOL.get().expect("pool not initialized");
        let result: Vec<Highlight> = sqlx::query_as(
            "SELECT category, title, body FROM place_highlights WHERE place_id = $1 ORDER BY id",
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .expect("highlights query failed");
        result
    });

    let place_data = place();
    let highlights_data = highlights();

    rsx! {
        if let (Some(place), Some(highlights)) = (place_data, highlights_data) {
            div {
                class: "place-page",

                BackButton {}

                div {
                    class: "place-hero",
                    span { class: "place-kind", "{place.kind}" }
                    h1 { "{place.name}" }
                    p { class: "place-tagline", "{place.tagline}" }
                    p { class: "place-overview", "{place.overview}" }
                }

                div {
                    class: "place-highlights",
                    for highlight in highlights {
                        div {
                            class: "place-highlight-card",
                            span { class: "place-highlight-category", "{highlight.category}" }
                            h3 { "{highlight.title}" }
                            p { "{highlight.body}" }
                        }
                    }
                }
            }
        } else {
            Loading {}
        }
    }
}
