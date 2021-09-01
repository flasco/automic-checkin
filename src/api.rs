use std::error::Error;

use reqwest::{self, Method};
use serde_json::Value;

pub async fn is_checkin(cookie: &str) -> Result<bool, Box<dyn Error>> {
    let cli = reqwest::Client::new();
    let result = cli
        .request(
            Method::GET,
            "https://api.juejin.cn/growth_api/v1/get_today_status",
        )
        .header("cookie", cookie)
        .send()
        .await?
        .text()
        .await?;

    let res: Value = serde_json::from_str(&result)?;
    // println!("？？？{:#?}", res["data"]);

    Ok(res["data"].as_bool().unwrap_or(false))
}

pub async fn checkin(cookie: &str) -> Result<(), Box<dyn Error>> {
    let cli = reqwest::Client::new();
    let result = cli
        .request(Method::POST, "https://api.juejin.cn/growth_api/v1/check_in")
        .header("cookie", cookie)
        .send()
        .await?
        .text()
        .await?;

    let res: Value = serde_json::from_str(&result)?;

    if res["err_no"].as_i64().unwrap() != (0 as i64) {
        panic!("{}", res["err_msg"].as_str().unwrap_or("fail to get msg"))
    }

    let data = res["data"].as_object().unwrap();
    println!(
        "已成功签到！+{}矿石，目前总矿石为：{}",
        data["incr_point"], data["sum_point"]
    );
    Ok(())
}

pub async fn have_free_lottery_count(cookie: &str) -> Result<bool, Box<dyn Error>> {
    let cli = reqwest::Client::new();
    let result = cli
        .request(
            Method::GET,
            "https://api.juejin.cn/growth_api/v1/lottery_config/get",
        )
        .header("cookie", cookie)
        .send()
        .await?
        .text()
        .await?;

    let res: Value = serde_json::from_str(&result)?;

    if res["err_no"].as_i64().unwrap() != (0 as i64) {
        panic!("{}", res["err_msg"].as_str().unwrap_or("fail to get msg"))
    }

    let data = res["data"].as_object().unwrap();
    // println!("{:?}", data);
    Ok(data["free_count"].as_i64().unwrap_or(0) > 0)
}

pub async fn lottery(cookie: &str) -> Result<(), Box<dyn Error>> {
    let cli = reqwest::Client::new();
    let result = cli
        .request(
            Method::POST,
            "https://api.juejin.cn/growth_api/v1/lottery/draw",
        )
        .header("cookie", cookie)
        .send()
        .await?
        .text()
        .await?;

    let res: Value = serde_json::from_str(&result)?;

    if res["err_no"].as_i64().unwrap() != (0 as i64) {
        panic!("{}", res["err_msg"].as_str().unwrap_or("fail to get msg"))
    }

    let data = res["data"].as_object().unwrap();
    println!("获得 - {}", data["lottery_name"]);
    Ok(())
}
