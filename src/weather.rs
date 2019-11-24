pub fn get_current_temperature(city: String, units: String) -> String {
    let url_str = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&units={}&APPID=347492b4193159e8965692864ec168ea",
        city, units);
    let json_response: serde_json::Value = reqwest::get(&url_str).unwrap().json().unwrap();
    json_response["main"]["temp"].to_string()
}

pub fn get_forecast(city: String, units: String) -> String {
    let url_str = format!(
        "http://api.openweathermap.org/data/2.5/forecast?q={}&units={}&APPID=347492b4193159e8965692864ec168ea",
        city, units);
    let json_response: serde_json::Value = reqwest::get(&url_str).unwrap().json().unwrap();
    serde_json::json!(json_response["list"]).to_string()
}
