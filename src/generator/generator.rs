use super::Config;

use std::{
    thread,
    time::{Duration, SystemTime},
};

/// Generator provides an instance that can generate unique ids for a given node_id.
pub struct Generator {
    config: Config,
    node_id: u64,
    last_ts: u64,
    last_seq_id: u64,
}

impl Generator {
    /// Returns a generator with given config for a given node-id.
    /// Same config should be used across all nodes of a cluster.
    /// Number of nodes is constrained by the `node_id_bit_count` in
    /// config.
    pub fn from(config: Config, node_id: u64) -> Self {
        let max_node_id = (1 << config.node_id_bit_count()) - 1;
        if node_id > max_node_id {
            panic!("Node id out of range - [0, {max_node_id}].")
        }
        Self {
            config,
            node_id,
            last_ts: 0,
            last_seq_id: 0,
        }
    }

    /// Get returns a unique id which is generated in a sequence maintained per millisecond window.
    /// If the load is higher than the number of unique ids that can be generated for a specific
    /// millisecond window, then the methods sleeps till next millisecond window to ensure uniqueness.
    pub fn get(&mut self) -> u64 {
        let curr_ts = self.ms_since_epoch() as u64;
        if curr_ts <= self.last_ts {
            self.last_seq_id += 1;
            if self.last_seq_id == (1 << self.config.node_id_shift() - 1) {
                thread::sleep(Duration::from_millis(1));
                self.last_ts = curr_ts;
                self.last_seq_id = 0;
            }
        } else {
            self.last_ts = curr_ts;
            self.last_seq_id = 0;
        }
        return curr_ts << self.config.timestamp_shift()
            | self.node_id << self.config.node_id_shift()
            | self.last_seq_id;
    }

    fn ms_since_epoch(&self) -> u128 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_millis(),
            Err(_) => 0,
        }
    }
}
