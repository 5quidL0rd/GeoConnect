use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Comparison {
    pub title: String,
    pub us_practice: String,
    pub jp_practice: String,
    pub us_region: String,
    pub key_difference: String,
}

pub async fn fetch_comparisons(region_id: i32) -> Vec<Comparison> {
    let pool = crate::DB_POOL.get().expect("pool not initialized");
    sqlx::query_as(
        "SELECT title, us_practice, jp_practice, us_region, key_difference FROM comparisons WHERE region_id = $1 ORDER BY id",
    )
    .bind(region_id)
    .fetch_all(pool)
    .await
    .expect("comparisons query failed")
}

/// Toggleable "Compare with the USA" section for a region page. Renders
/// pre-fetched rows from the `comparisons` table (category 5, "Regional
/// Analog") that pair this region against the US region it most resembles,
/// so visitors can see what's familiar and what's different before they
/// land in Japan.
#[component]
pub fn CompareContrast(comparisons: Vec<Comparison>) -> Element {
    let mut show = use_signal(|| false);

    let us_region = comparisons.first().map(|c| c.us_region.clone());

    rsx! {
        div {
            class: "compare-callout",
            button {
                class: "compare-btn",
                onclick: move |_| show.toggle(),
                span { class: "compare-btn-icon", "🇺🇸" }
                "Compare with the USA"
            }
            p {
                class: "compare-blurb",
                if let Some(us_region) = us_region {
                    "Closest US analog: {us_region} — click to see what's familiar and what's different."
                } else {
                    "See what's familiar and what's different before you go."
                }
            }
        }

        if show() {
            div {
                class: "compare-section",
                h2 { "USA vs. Japan" }

                div {
                    class: "compare-grid",
                    for comparison in comparisons {
                        div {
                            class: "compare-card",
                            h3 { "{comparison.title}" }
                            div {
                                class: "compare-columns",
                                div {
                                    class: "compare-column compare-column--us",
                                    span { class: "compare-column-label", "🇺🇸 {comparison.us_region}" }
                                    p { "{comparison.us_practice}" }
                                }
                                div {
                                    class: "compare-column compare-column--jp",
                                    span { class: "compare-column-label", "🇯🇵 Japan" }
                                    p { "{comparison.jp_practice}" }
                                }
                            }
                            div {
                                class: "compare-difference",
                                span { class: "compare-difference-label", "⚠️ Where it actually differs" }
                                p { "{comparison.key_difference}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
