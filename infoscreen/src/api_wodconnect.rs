use std::sync::Arc;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc, DateTime, Timelike};
use chrono_tz::{Europe::Helsinki, Tz};
use serde_derive::Serialize;

use crate::{api::ApiQuery, config::InfoscreenConfig};

pub struct WodConnect;

#[derive(Serialize)]
pub struct Workout {
    title: String,
    description: String
}

#[derive(Serialize)]
pub struct WorkoutsOfDate {
    date: String,
    workouts: Vec<Workout>
}

impl WodConnect {
    const API_PATH: &'static str = "https://www.wodconnect.com";

    async fn load_from_wodconnect(&self, local_path: String, cookie: String) -> String {

        let url = format!("{}{}", Self::API_PATH, local_path);
        println!("Calling api at {}", url);

        reqwest::Client::new()
            .get(&url)
            .header("Cookie", cookie)
            .send()
            .await
            .expect("failed to send request")
            .text()
            .await
            .expect("failed to read response text")
    }

    fn get_workout_infos(&self, html: String) -> Option<Vec<Workout>> {
        let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).ok()?;
        let parser = dom.parser();

        let infos =
            dom.get_elements_by_class_name("workout_info").into_iter().filter_map(|element| {
                let child_nodes =
                    element
                    .get(parser)?
                    .children()?
                    .all(parser);

                let title = child_nodes.into_iter().find_map(|child| {
                    match child.as_tag()?.name() {
                        name if name == "a" => Some(child.inner_text(parser).to_string()),
                        _ => None
                    }
                })?;

                let description = child_nodes.into_iter().find_map(|child| {
                    match child.as_tag() {
                        Some(tag) if tag.name() == "div" && tag.attributes().get("class").flatten()? == "markdown_content" => Some(child.inner_text(parser).to_string()),
                        _ => None
                    }
                })?;

                Some(Workout { title, description })
            }).collect();

        Some(infos)
    }

    async fn get_feed_for_date(&self, date: NaiveDate, gym_id: String, cookie: String) -> Option<String> {
        let date_string = date.format("%Y-%m-%d").to_string();

        let url =
            if date == Utc::now().with_timezone(&Helsinki).date_naive() {
                format!("/{}/feed/{}", gym_id, date_string)
            } else {
                let html = self.load_from_wodconnect(String::from(format!("/{}/feed", gym_id)), cookie.clone()).await;
                let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).ok()?;
                let parser = dom.parser();

                let table_tags =
                    dom
                    .query_selector("table.scheduled_workouts")?
                    .nth(0)?
                    .get(parser)?
                    .children()?
                    .all(parser)
                    .into_iter()
                    .filter(|child| {
                        let tag = child.as_tag();
                        tag.is_some() && (tag.unwrap().name() == "thead" || tag.unwrap().name() == "tbody")
                    });

                let position = table_tags.clone().into_iter().position(|child| {
                    let tag = child.as_tag().expect("must be tag");
                    let dd = tag.attributes().get("data-day").flatten();
                    tag.name() == "thead" && dd.is_some() && dd.unwrap() == date_string.as_str()
                })?;

                let tbody = table_tags.clone().into_iter().nth(position + 1)?;
                let link =
                    tbody
                    .as_tag()?
                    .query_selector(parser, "a")?
                    .nth(0)?
                    .get(parser)?
                    .as_tag()?;

                link.attributes().get("href").flatten()?.as_utf8_str().to_ascii_lowercase()
            };

        let response = self.load_from_wodconnect(url, cookie).await;

        let workouts = self.get_workout_infos(response)?;
        let workouts_of_date = WorkoutsOfDate { date: date_string.to_string(), workouts };

        let json = match serde_json::to_string(&workouts_of_date) {
            Ok(json) => json,
            Err(err) => format!("Error serializing json: {}", err)
        };
        Some(json)
    }
}

#[async_trait]
impl ApiQuery for WodConnect {

    fn get_name(&self) -> String {
        "wodconnect".to_string()
    }

    async fn get_response(&self, config: Arc<InfoscreenConfig>) -> (String, std::time::Instant) {
        let helsinki_time: DateTime<Tz> = Utc::now().with_timezone(&Helsinki);
        let query_date = if helsinki_time.hour() < 18 { helsinki_time.date_naive() } else { helsinki_time.date_naive() + chrono::Duration::days(1) };

        let response_text = self.get_feed_for_date(query_date.clone(), config.wodconnect_gym_id.clone(), config.wodconnect_cookie.clone()).await;

        let now = Utc::now();
        let secs_to_next_hour = 3600 - (now.time().minute() * 60 + now.time().second());
        let valid_until = std::time::Instant::now() + std::time::Duration::from_secs(secs_to_next_hour as u64);

        (response_text.unwrap_or(String::from("Could not load")), valid_until)
    }
}
