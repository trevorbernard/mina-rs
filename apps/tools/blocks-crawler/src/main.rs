// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use blocks::{check_block, fetch_block_info, fetch_next_block_info, BlockInfo};
use clap::Parser;
use futures::{stream, StreamExt};
use std::fs::create_dir_all;
#[macro_use]
extern crate log;

mod blocks;

const INIT_STATE_HASH: &'static str = "3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ";

#[derive(Parser)]
struct Cli {
    #[clap(short = 'l', long = "limit")]
    limit: String,
    #[clap(short = 'c', long = "concurrency", default_value_t = 8)]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();
    // Create output folders
    create_dir_all(&"output/succeeded/").unwrap();
    create_dir_all(&"output/failed/").unwrap();

    // Read limit value from cli
    let limit = args.limit;

    // Fetch n blocks info from graphql api
    let mut blocks = fetch_next_block_info(INIT_STATE_HASH).await.unwrap().data.blocks;
    while !blocks.is_empty() {
        stream::iter(blocks)
            .for_each_concurrent(args.concurrency, |block| async move {
                check_block("mainnet", block).await;
            })
            .await;
        blocks = fetch_next_block_info(INIT_STATE_HASH).await.unwrap().data.blocks;
    } 
}
