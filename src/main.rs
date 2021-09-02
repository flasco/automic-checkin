use std::env;
use std::error::Error;
use std::time::Duration;

mod api;

use api::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let cookie = args.next().unwrap_or("".to_string());

    if cookie.len() < 1 {
        eprintln!("need cookie");
        return Ok(());
    }

    if !is_checkin(&cookie).await? {
        checkin(&cookie).await?;
    } else {
        println!("今日已签到！");
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    checkin_status(&cookie).await?;

    if have_free_lottery_count(&cookie).await? {
        println!("还有免费的抽奖机会~");

        tokio::time::sleep(Duration::from_millis(600)).await;

        lottery(&cookie).await?;
    } else {
        println!("今日免费一抽已抽取~");
    }

    Ok(())
}
