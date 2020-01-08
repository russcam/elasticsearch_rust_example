# elasticsearch_rust_example

An example of how to use the Elasticsearch rust client.

1. Set up an [Elasticsearch cluster on Elastic Cloud](https://cloud.elastic.co/)
2. Copy the cloud_id from the cloud web console and paste into main.rs
3. Copy the `elastic` username and generated password and paste into main.rs (_an application would not use the `elastic` superuser credentials under normal circumstances; create an API key, token, or specific user with less privileges instead! But this is just a demo_)
4. Run the binary with `cargo run`
