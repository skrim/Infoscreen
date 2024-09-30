use std::{sync::Arc, time::{self, Duration}};

use async_trait::async_trait;

use crate::{api::ApiQuery, config::InfoscreenConfig};

pub struct DigitransitBikes;

async fn digitransit_query(query_template: &str, ids: &str, key: &str) -> String {
    let url = format!("https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql?digitransit-subscription-key={}", key);

    let id_list =
            ids
            .split(",")
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<String>>()
            .join(",");

    let content = str::replace(query_template, "{IDS}", id_list.as_str());

    println!("Calling api at {}", url);

    let client = reqwest::Client::new();
    let response = client.post(url)
        .body(content)
        .header("Content-Type", "application/graphql")
        .send()
        .await
        .expect("failed to send request");

    response.text().await.expect("failed to read response text")
}

#[async_trait]
impl ApiQuery for DigitransitBikes {
    fn get_name(&self) -> String {
        "bikes".to_string()
    }

    async fn get_response(&self, config: Arc<InfoscreenConfig>) -> (String, std::time::Instant) {
        const BIKE_STATION_QUERY: &str = "{ bikeRentalStations(ids:[{IDS}]) { name bikesAvailable spacesAvailable } }";
        let response_text = digitransit_query(BIKE_STATION_QUERY, &config.digitransit_query_bike_stations, &config.digitransit_key).await;

        let valid_until = time::Instant::now() + Duration::from_secs(2 * 60);
        (response_text, valid_until)
    }
}

pub struct DigitransitTrams;

#[async_trait]
impl ApiQuery for DigitransitTrams {
    fn get_name(&self) -> String {
        "trams".to_string()
    }

    async fn get_response(&self, config: Arc<InfoscreenConfig>) -> (String, std::time::Instant) {
        const TRAM_QUERY: &str = "{
            stops(ids: [{IDS}]) {
              name
              stoptimesWithoutPatterns {
                scheduledDeparture realtimeDeparture serviceDay headsign
                trip {
                  routeShortName directionId
                }
              }
            }
        }";
        let response_text = digitransit_query(TRAM_QUERY, &config.digitransit_query_stops, &config.digitransit_key).await;

        let valid_until = time::Instant::now() + Duration::from_secs(60);
        (response_text, valid_until)
    }
}
