extern crate clap;
extern crate sda_server;
extern crate sda_server_http;
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;

use std::sync;
use slog::*;

fn tmp_server() -> sda_server::SdaServer {
    let agents = sda_server::jfs_stores::JfsAgentsStore::new("tmp/agents").unwrap();
    let auth = sda_server::jfs_stores::JfsAuthTokensStore::new("tmp/auths").unwrap();
    let aggregation = sda_server::jfs_stores::JfsAggregationsStore::new("tmp/aggs").unwrap();
    sda_server::SdaServer {
        agents_store: Box::new(agents),
        auth_tokens_store: Box::new(auth),
        aggregation_store: Box::new(aggregation),
    }
}

fn main() {
    let root = Logger::root(slog_term::streamer().stderr().use_utc_timestamp().build().fuse(),
                            o!());
    slog_scope::set_global_logger(root);
    let server = tmp_server();
    sda_server_http::listen("0.0.0.0:8888", sync::Arc::new(server))
}
