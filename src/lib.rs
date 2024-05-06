//! # Snowflake-UID
//! Snowflake-UID is a crate that provides a simple implementation of unique-id generation based on [Twitter's snowflake](https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake) logic.
//! ```
//! +--------------------------------------------------------------------------+
//! | 1 Bit Unused | 41 Bit Timestamp |  10 Bit NodeID  |   12 Bit Sequence ID |
//! +--------------------------------------------------------------------------+
//! ```
//! ## Logic
//! - There are two components - `Generator` and `Config`.
//! - `Config` has a default implementation which provides 2 ^ 21 (2097152) unique ids per millisecond (1024 per node, across 2048 nodes) till year 2149 (~139 years from epoch). These values can be customised to support higher throughput or longer period of valid generation of unique IDs. Default config uses following details -
//!   - timestamp_bit_count - 42
//!   - node_id_bit_count - 11
//!
//! ## Example
//! ```
//! use snowflake_uid::{Config, Generator};
//!
//! // Default config
//! let cfg = Config::default();
//! let mut gen = Generator::from(cfg, env::var_os("HOST_NODE_ID"));
//! let uid = gen.get();
//!
//! // Custom config for more coarse window size and larger number of nodes in the cluster.
//! let cfg_2 = Config::from(40, 13);
//! let mut gen = Generator::from(cfg_2, env::var_os("HOST_NODE_ID"));
//! let uid = gen.get();
//!
//! ```

mod generator;

pub use generator::Config;
pub use generator::Generator;
