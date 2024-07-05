
#[allow(dead_code)]
pub(crate) fn ads_lookup_days_ahead() -> i8 {
    return 1;
}

#[allow(dead_code)]
pub(crate) fn ad_is_ready_for_transfer(ads_info: &mut serde_json::Value) -> bool {
    let category_id = ads_info["category"]["id"][0].as_str().unwrap_or("").to_string();

    ads_info["status"] = ads_info["status"].as_str().unwrap_or("").to_lowercase().into();
    ads_info["production_status"] = ads_info["production_status"].as_str().unwrap_or("").to_lowercase().into();

    ads_info["online_publish"].as_bool().unwrap_or(false) &&
        !ads_info["online_stop"].as_bool().unwrap_or(false) &&
        (ads_info["status"] == "buchhung" || ads_info["status"] == "fertig") &&
        ads_info["production_status"] == "fertig" &&
        ["2", "4", "6", "7"].contains(&category_id.as_str())
}