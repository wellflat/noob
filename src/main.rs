extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct ResultSet {
    result: Result
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Result {
    ranking_info: RankingInfo
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct RankingInfo {
    last_modified: String,
    start_date: String,
    end_date: String,
    category_id: u32,
    gender: String,
    generation: String,
    period: String
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://shopping.yahooapis.jp/ShoppingWebService/V1/categoryRanking?appid={appid}&category_id={catid}",
        appid = "9gQVI.qxg64ZKg8nmFmO.nPdg.Lk6TYyCl8jxN5WUyEKANFMQd35ElMWiEj4dfoe6g--",
        catid= 2161.to_string()
    );
    println!("{:#?}", url);
    let res = reqwest::get(&url).await?.text().await?;
    let _result_set: ResultSet = from_str(&res).unwrap();

    //println!("{:#?}", res);
    println!("{:#?}", _result_set.result.ranking_info.last_modified);
    Ok(())
}
