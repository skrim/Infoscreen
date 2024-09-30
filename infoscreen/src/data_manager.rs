use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use futures_util::future::join_all;
use once_cell::sync::Lazy;

use crate::api_wodconnect;
use crate::api_digitransit;
use crate::api_openweathermap;
use crate::api::ApiQuery;
use crate::config::InfoscreenConfig;

pub static DATA_MANAGER: Lazy<DataManager> = Lazy::new(|| {
    DataManager::new(confy::load("infoscreen", None).expect("failed to load config"))
});

pub struct DataManager {
    data_cache: Arc<Mutex<HashMap<String, (String, Instant)>>>,
    //sources: Vec<Box<dyn ApiQuery>>,
    config: Arc<InfoscreenConfig>
}

impl DataManager {
    pub fn new(config: InfoscreenConfig) -> Self {
        DataManager {
            data_cache: Arc::new(Mutex::new(HashMap::new())),
            //sources,
            config: Arc::new(config)
        }
    }

    pub async fn get_data(&self) -> String {
        let api_queries: Vec<Box<dyn ApiQuery>> = vec![
            Box::new(api_openweathermap::OpenWeatherMap),
            Box::new(api_digitransit::DigitransitBikes),
            Box::new(api_digitransit::DigitransitTrams),
            Box::new(api_wodconnect::WodConnect)
        ];

        let mut result = Vec::new();
        let mut fetch_tasks = Vec::new();

        for api_query in api_queries {
            let config_arc = Arc::clone(&self.config);
            let source_name = api_query.get_name();

            let valid_until = self.data_cache.lock().unwrap().get(&source_name).map(|x| x.1);

            match valid_until {
                Some(valid_until) if Instant::now() < valid_until => {
                    if let Some(data) = self.data_cache.lock().unwrap().get(&source_name) {
                        result.push((source_name, data.0.clone()));
                    }
                    ()
                }
                _ => {
                    let task =
                        tokio::spawn(async move {
                            let response = api_query.get_response(config_arc).await;
                            (source_name, response.0, response.1)
                        });
                    fetch_tasks.push(task);
                }
            }
        }

        let query_results = join_all(fetch_tasks).await;
        for query_result in query_results {
            let r = query_result.expect("No result");
            self.data_cache.lock().unwrap().insert(r.0.clone(), (r.1.clone(), r.2));
            result.push((r.0, r.1));
        }

        let json_results: Vec<String> = result.into_iter().map(|r| { format!("\"{}\": {}", r.0, r.1) }).collect();
        format!("{{{}}}", json_results.join(","))
    }
}

