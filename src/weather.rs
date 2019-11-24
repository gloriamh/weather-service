fn req_get_json(url_str: &str) -> serde_json::Value {
    reqwest::get(url_str).unwrap().json().unwrap()
}

pub fn get_current_temperature(city: String, units: String) -> String {
    let url_str = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&units={}&APPID=347492b4193159e8965692864ec168ea",
        city, units);
    let json_response: serde_json::Value = req_get_json(&url_str);
    json_response["main"]["temp"].to_string()
}

pub fn get_forecast(city: String, units: String) -> String {
    let url_str = format!(
        "http://api.openweathermap.org/data/2.5/forecast?q={}&units={}&APPID=347492b4193159e8965692864ec168ea",
        city, units);
    let json_response: serde_json::Value = req_get_json(&url_str);
    serde_json::json!(json_response["list"]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_temperature() {
        let city = String::from("London");
        let units = String::from("metric");
        let current_temperature = get_current_temperature(city, units);
        // TODO: Mock openweathermap response
        // assert_eq!(current_temperature, "10");
    }
}
