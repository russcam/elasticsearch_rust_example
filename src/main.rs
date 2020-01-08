use elasticsearch::{
    auth::Credentials, http::transport::Transport,
    params::Refresh, Elasticsearch, IndexParts,
    SearchParts,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cloud_id retrieved from cloud web console
    let cloud_id = "<retrieve the cloud id from the web console>";

    // username and password retrieved from cloud web console
    let credentials = Credentials::Basic("<username>".into(), "<password>".into());
    let transport = Transport::cloud(cloud_id, credentials)?;
    let client = Elasticsearch::new(transport);

    let index_response = client
        .index(IndexParts::IndexId("tweets", "1"))
        .body(json!({
            "user": "kimchy",
            "post_date": "2009-11-15T00:00:00Z",
            "message": "Trying out Elasticsearch, so far so good?"
        }))
        .refresh(Refresh::WaitFor)
        .send()
        .await?;

    if !index_response.status_code().is_success() {
        panic!("indexing document failed")
    }

    let index_response = client
        .index(IndexParts::IndexId("tweets", "2"))
        .body(json!({
            "user": "forloop",
            "post_date": "2020-01-08T00:00:00Z",
            "message": "Indexing with the rust client, yeah!"
        }))
        .refresh(Refresh::WaitFor)
        .send()
        .await?;

    if !index_response.status_code().is_success() {
        panic!("indexing document failed")
    }

    let response = client
        .search(SearchParts::Index(&["tweets"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "match": {
                    "message": "Elasticsearch rust client"
                }
            }
        }))
        .send()
        .await?;

    let response_body = response.read_body::<Value>().await?;

    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        println!(
            "id: {}, message: '{}', score: {}",
            hit["_id"].as_str().unwrap(),
            hit["_source"]["message"].as_str().unwrap(),
            hit["_score"].as_f64().unwrap()
        );
    }

    Ok(())
}
