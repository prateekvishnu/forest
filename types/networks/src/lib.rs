// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

#[macro_use]
extern crate lazy_static;

use beacon::{BeaconPoint, BeaconSchedule, DrandBeacon, DrandConfig};
use clock::ChainEpoch;
use fil_types::NetworkVersion;
use std::{error::Error, sync::Arc};
mod drand;

#[cfg(all(
    not(feature = "interopnet"),
    not(feature = "devnet"),
    not(feature = "mainnet"),
    not(feature = "conformance"),
    not(feature = "calibnet")
))]
compile_error!(
    "No network feature selected. Exactly one of \"mainnet\", \"devnet\", \"interopnet\", or \"conformance\" must be enabled for this crate."
);

#[cfg(all(
    feature = "mainnet",
    any(
        feature = "interopnet",
        feature = "devnet",
        feature = "conformance",
        feature = "calibnet"
    )
))]
compile_error!(
    "\"mainnet\" feature cannot be combined with \"devnet\", \"interopnet\", or \"conformance\", or \"calibnet\"."
);

#[cfg(all(
    feature = "conformance",
    any(
        feature = "interopnet",
        feature = "devnet",
        feature = "mainnet",
        feature = "calibnet"
    )
))]
compile_error!(
    "\"conformance\" feature cannot be combined with \"devnet\", \"interopnet\", or \"mainnet\", \"calibnet\"."
);

#[cfg(all(
    feature = "devnet",
    any(
        feature = "interopnet",
        feature = "conformance",
        feature = "mainnet",
        feature = "calibnet"
    )
))]
compile_error!(
    "\"devnet\" feature cannot be combined with \"conformance\", \"interopnet\", or \"mainnet\", \"calibnet\"."
);

#[cfg(all(
    feature = "interopnet",
    any(
        feature = "conformance",
        feature = "devnet",
        feature = "mainnet",
        feature = "calibnet"
    )
))]
compile_error!(
    "\"interopnet\" feature cannot be combined with \"devnet\", \"conformance\", or \"mainnet\", \"calibnet\"."
);

#[cfg(all(
    feature = "calibnet",
    any(
        feature = "conformance",
        feature = "devnet",
        feature = "mainnet",
        feature = "interopnet"
    )
))]
compile_error!(
    "\"interopnet\" feature cannot be combined with \"devnet\", \"conformance\", or \"mainnet\", \"interopnet\"."
);

// Both mainnet and conformance parameters are kept in the 'mainnet' module.
#[cfg(any(feature = "mainnet", feature = "conformance"))]
mod mainnet;
#[cfg(any(feature = "mainnet", feature = "conformance"))]
pub use self::mainnet::*;

#[cfg(feature = "interopnet")]
mod interopnet;
#[cfg(feature = "interopnet")]
pub use self::interopnet::*;

#[cfg(feature = "devnet")]
mod devnet;
#[cfg(feature = "devnet")]
pub use self::devnet::*;

#[cfg(feature = "calibnet")]
mod calibnet;
#[cfg(feature = "calibnet")]
pub use self::calibnet::*;

/// Defines the different hard fork parameters.
struct Upgrade {
    /// When the hard fork will happen
    height: ChainEpoch,
    /// The version of the fork
    network: NetworkVersion,
}

struct DrandPoint<'a> {
    pub height: ChainEpoch,
    pub config: &'a DrandConfig<'a>,
}

const VERSION_SCHEDULE: [Upgrade; 14] = [
    Upgrade {
        height: UPGRADE_BREEZE_HEIGHT,
        network: NetworkVersion::V1,
    },
    Upgrade {
        height: UPGRADE_SMOKE_HEIGHT,
        network: NetworkVersion::V2,
    },
    Upgrade {
        height: UPGRADE_IGNITION_HEIGHT,
        network: NetworkVersion::V3,
    },
    Upgrade {
        height: UPGRADE_ACTORS_V2_HEIGHT,
        network: NetworkVersion::V4,
    },
    Upgrade {
        height: UPGRADE_TAPE_HEIGHT,
        network: NetworkVersion::V5,
    },
    Upgrade {
        height: UPGRADE_KUMQUAT_HEIGHT,
        network: NetworkVersion::V6,
    },
    Upgrade {
        height: UPGRADE_CALICO_HEIGHT,
        network: NetworkVersion::V7,
    },
    Upgrade {
        height: UPGRADE_PERSIAN_HEIGHT,
        network: NetworkVersion::V8,
    },
    Upgrade {
        height: UPGRADE_ORANGE_HEIGHT,
        network: NetworkVersion::V9,
    },
    Upgrade {
        height: UPGRADE_ACTORS_V3_HEIGHT,
        network: NetworkVersion::V10,
    },
    Upgrade {
        height: UPGRADE_NORWEGIAN_HEIGHT,
        network: NetworkVersion::V11,
    },
    Upgrade {
        height: UPGRADE_ACTORS_V4_HEIGHT,
        network: NetworkVersion::V12,
    },
    Upgrade {
        height: UPGRADE_HYPERDRIVE_HEIGHT,
        network: NetworkVersion::V13,
    },
    Upgrade {
        height: UPGRADE_ACTORS_V6_HEIGHT,
        network: NetworkVersion::V14,
    },
];

/// Gets network version from epoch using default Mainnet schedule.
pub fn get_network_version_default(epoch: ChainEpoch) -> NetworkVersion {
    VERSION_SCHEDULE
        .iter()
        .rev()
        .find(|upgrade| epoch > upgrade.height)
        .map(|upgrade| upgrade.network)
        .unwrap_or(NetworkVersion::V0)
}

/// Constructs a drand beacon schedule based on the build config.
pub async fn beacon_schedule_default(
    genesis_ts: u64,
) -> Result<BeaconSchedule<DrandBeacon>, Box<dyn Error>> {
    let mut points = BeaconSchedule(Vec::with_capacity(DRAND_SCHEDULE.len()));
    for dc in DRAND_SCHEDULE.iter() {
        points.0.push(BeaconPoint {
            height: dc.height,
            beacon: Arc::new(DrandBeacon::new(genesis_ts, BLOCK_DELAY_SECS, dc.config).await?),
        });
    }
    Ok(points)
}
