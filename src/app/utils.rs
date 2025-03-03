use chrono::{ DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Utc };
use wasm_bindgen::JsValue;
// #[wasm_bindgen::prelude::wasm_bindgen(module = "/public/utils.js")]
// extern "C" {
//     pub fn setItem(key: &str, value: &str) -> JsValue;
//     pub fn getItem(key: &str) -> JsValue;
//     pub fn removeItem(key: &str);
// }
pub fn calculate_age(birth_date: &str) -> i64 {
    let parsed_date = NaiveDate::parse_from_str(birth_date, "%Y-%m-%d").unwrap();
    let datetime_utc = Utc.with_ymd_and_hms(
        parsed_date.year(),
        parsed_date.month(),
        parsed_date.day(),
        0,
        0,
        0
    ).unwrap();
    // Format as RFC3339
    let birth_datetime = DateTime::parse_from_rfc3339(&datetime_utc.to_rfc3339()).unwrap();
    let now = Utc::now();
    let duration = now.signed_duration_since(birth_datetime);
    duration.num_days() / 365
}
pub fn convert_date_format(input: &String) -> String {
    if input == "Now" {
        "Now".to_string()
    } else {
        let formats = ["%Y-%m-%d", "%m/%d/%Y", "%d-%m-%Y"];
        for fmt in formats {
            if let Ok(date) = NaiveDate::parse_from_str(input, fmt) {
                return date.format("%d/%m/%Y").to_string();
            }
        }
        // Fallback to default date
        NaiveDate::from_ymd_opt(2000, 1, 1).unwrap().format("%d/%m/%Y").to_string()
    }
}
pub fn format_date_for_input(date_str: &str) -> String {
    // Parse the ISO 8601 date string
    if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ") {
        // Format as YYYY-MM-DD
        datetime.format("%Y-%m-%d").to_string()
    } else {
        // Return a default date if parsing fails
        String::from("2000-01-01")
    }
}
