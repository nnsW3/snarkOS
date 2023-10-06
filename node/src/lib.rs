// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]
#![recursion_limit = "256"]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate tracing;

pub use snarkos_node_cdn as cdn;
pub use snarkos_node_consensus as consensus;
pub use snarkos_node_narwhal as narwhal;
pub use snarkos_node_rest as rest;
pub use snarkos_node_router as router;
pub use snarkos_node_tcp as tcp;
pub use snarkvm;

mod client;
pub use client::*;

mod prover;
pub use prover::*;

mod validator;
pub use validator::*;

mod node;
pub use node::*;

mod traits;
pub use traits::*;

/// A helper to log instructions to recover.
pub fn log_clean_error(dev: Option<u16>) {
    match dev {
        Some(id) => error!("Storage corruption detected! Run `snarkos clean --dev {id}` to reset storage"),
        None => error!("Storage corruption detected! Run `snarkos clean` to reset storage"),
    }
}

/// Starts the notification message loop.
pub fn start_notification_message_loop() -> tokio::task::JoinHandle<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(180));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            info!("{}", notification_message());
        }
    })
}

/// Returns the notification message as a string.
pub fn notification_message() -> String {
    use colored::Colorize;

    let mut output = String::new();
    output += &r#"

 ==================================================================================================

                     🚧 Welcome to Aleo Testnet 3 Phase 3 - Calibration Period 🚧

 ==================================================================================================

     During the calibration period, the network will be running in limited capacity.

     This calibration period is to ensure validators are stable and ready for mainnet launch.
     During this period, the objective is to assess, adjust, and align validators' performance,
     stability, and interoperability under varying network conditions.

     Please expect several network resets. With each network reset, software updates will
     be performed to address potential bottlenecks, vulnerabilities, and/or inefficiencies, which
     will ensure optimal performance for the ecosystem of validators, provers, and developers.

 ==================================================================================================

    Duration:
    - Start Date: September 27, 2023
    - End Date: October 18, 2023 (subject to change)

    Participation:
    - Node operators are NOT REQUIRED to participate during this calibration period.

    Network Resets:
    - IMPORTANT: EXPECT MULTIPLE NETWORK RESETS.
    - If participating, BE PREPARED TO RESET YOUR NODE AT ANY TIME.
    - When a reset occurs, RUN THE FOLLOWING TO RESET YOUR NODE:
        - git checkout testnet3 && git pull
        - cargo install --path .
        - snarkos clean
        - snarkos start --nodisplay --client

    Communication:
    - Stay ONLINE and MONITOR our Discord and Twitter for community updates.

    Purpose:
    - This period is STRICTLY FOR NETWORK CALIBRATION.
    - This period is NOT INTENDED for general-purpose usage by developers and provers.

    Incentives:
    - There are NO INCENTIVES during this calibration period.

 ==================================================================================================
"#
    .white()
    .bold();

    output
}
