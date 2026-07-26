use dioxus::prelude::*;
use crate::flowchart::FlowChart;
use crate::place_nav::{PlaceNav, fetch_places};
use crate::loading::Loading;
use crate::back_button::BackButton;
use crate::compare::{CompareContrast, fetch_comparisons};
use crate::travel_prep::{TravelPrep, fetch_travel_prep};

const REGION_ID: i32 = 8;
const OKINAWA_REGION_ID: i32 = 9;

#[derive(Debug, Clone, sqlx::FromRow)]
struct Fact {
    title: String,
    body: String,
}

async fn fetch_facts(region_id: i32) -> Vec<Fact> {
    let pool = crate::DB_POOL.get().expect("pool not initialized");
    sqlx::query_as("SELECT title, body FROM facts WHERE region_id = $1")
        .bind(region_id)
        .fetch_all(pool)
        .await
        .expect("facts query failed")
}

#[component]
pub fn Kyushu() -> Element {
    let mut selected = use_signal(|| Option::<Fact>::None);
    let mut show_okinawa = use_signal(|| false);
    let mut okinawa_selected = use_signal(|| Option::<Fact>::None);

    let facts = use_resource(move || fetch_facts(REGION_ID));
    let places = use_resource(move || fetch_places(REGION_ID));
    let comparisons = use_resource(move || fetch_comparisons(REGION_ID));
    let travel_prep = use_resource(move || fetch_travel_prep(REGION_ID));

    let okinawa_facts = use_resource(move || fetch_facts(OKINAWA_REGION_ID));
    let okinawa_places = use_resource(move || fetch_places(OKINAWA_REGION_ID));
    let okinawa_comparisons = use_resource(move || fetch_comparisons(OKINAWA_REGION_ID));
    let okinawa_travel_prep = use_resource(move || fetch_travel_prep(OKINAWA_REGION_ID));

    rsx! {
        if let (Some(facts), Some(places), Some(comparisons), Some(travel_prep)) =
            (facts(), places(), comparisons(), travel_prep())
        {
            div {
                class: "region-page",
                BackButton {}

                h1 { "Kyushu " }

                PlaceNav { places }

                for fact in facts {
                    button {
                        class: "fact-btn",
                        onclick: {
                            let fact = fact.clone();
                            move |_| selected.set(Some(fact.clone()))
                        },
                        "{fact.title}"
                    }
                }

                if let Some(fact) = selected() {
                    div {
                        class: "fact-detail",
                        h2 { "{fact.title}" }
                        FlowChart { key: "{fact.title}", steps: fact.body.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>() }
                    }
                }

                CompareContrast { comparisons }
                TravelPrep { items: travel_prep }

                div {
                    class: "okinawa-callout",
                    button {
                        class: "okinawa-btn",
                        onclick: move |_| show_okinawa.toggle(),
                        span { class: "okinawa-btn-icon", "🌊" }
                        "Okinawa"
                    }
                    p {
                        class: "okinawa-blurb",
                        "Okinawa is often grouped with Kyushu, but its island history gave it a culture all its own — click to see how it differs."
                    }
                }

                if show_okinawa() {
                    if let (Some(okinawa_facts), Some(okinawa_places), Some(okinawa_comparisons), Some(okinawa_travel_prep)) =
                        (okinawa_facts(), okinawa_places(), okinawa_comparisons(), okinawa_travel_prep())
                    {
                        div {
                            class: "okinawa-section",
                            h2 { "Okinawa" }

                            PlaceNav { places: okinawa_places }

                            for fact in okinawa_facts {
                                button {
                                    class: "fact-btn okinawa-fact-btn",
                                    onclick: {
                                        let fact = fact.clone();
                                        move |_| okinawa_selected.set(Some(fact.clone()))
                                    },
                                    "{fact.title}"
                                }
                            }

                            if let Some(fact) = okinawa_selected() {
                                div {
                                    class: "fact-detail",
                                    h2 { "{fact.title}" }
                                    FlowChart { key: "{fact.title}", steps: fact.body.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>() }
                                }
                            }

                            CompareContrast { comparisons: okinawa_comparisons }
                            TravelPrep { items: okinawa_travel_prep }
                        }
                    } else {
                        Loading {}
                    }
                }
            }
        } else {
            Loading {}
        }
    }
}
