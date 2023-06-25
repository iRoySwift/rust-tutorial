// 解析URL地址参数
pub fn parse_url(url: &str) -> Option<(String, String)> {
    let mut url_split = url.split("?");
    // let url_path = url_split.next()?;
    let url_query = url_split.next()?;
    let url_query_split = url_query.split("&");
    let mut url_query_kv = Vec::new();
    for query in url_query_split {
        url_query_kv.push(query);
    }
    let mut url_kv = Vec::new();
    for kv in url_query_kv {
        let mut kv_split = kv.split("=");
        let key = kv_split.next()?;
        let value = kv_split.next()?;
        url_kv.push((key, value));
    }
    println!("{:?}", url_kv);
    Some((url_kv[0].0.to_string(), url_kv[0].1.to_string()))
}
