use std::{
    thread,
    time::{Duration, SystemTime},
};

const UID_BIT_COUNT: u64 = 63;
const MILLISECONDS_IN_YEAR: u64 = 1_000 * 86_400 * 365;

const DEFAULT_TIMESTAMP_BIT_COUNT: u64 = 42;
const DEFAULT_NODE_ID_BIT_COUNT: u64 = 11;
const MIN_SEQUENCE_ID_BIT_COUNT: u64 = 4;

fn main() {
    let cfg = Config::default();
    cfg.pprint();
    let mut gen = Generator::from(cfg, 12);
    let mut i = 0;
    loop {
        let id = gen.get();
        println!("Id #{i}: {id}");
        i += 1;
    }
}

/// Generator provides an instance that can generate unique ids for a given node_id.
struct Generator {
    config: Config,
    node_id: u64,
    last_ts: u64,
    last_seq_id: u64,
}

impl Generator {
    fn from(config: Config, node_id: u64) -> Self {
        let max_node_id = (1 << config.node_id_bit_count) - 1;
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
    fn get(&mut self) -> u64 {
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

// End UID Generator

/// Config controls the UID generation, mainly controlling the number of nodes and number of
/// unique ids per millisecond that a node can generate. This is tradeoff between how fast
/// you need to generate unique ids and across how many servers.
struct Config {
    timestamp_bit_count: u64,
    node_id_bit_count: u64,
}

impl Config {
    fn default() -> Self {
        Self::from(DEFAULT_TIMESTAMP_BIT_COUNT, DEFAULT_NODE_ID_BIT_COUNT)
    }

    /// Config::from generates a config to be used by generator to generate IDs.
    ///  - Use higher or lower timestamp_bit_count based on how long (generally in years) the lifecycle of a generated unique id should be.
    ///  - Use higher or lower node_id_bit_count based on how many servers are going to be involved in unique id generation.
    ///  - Above two values directly impact the range of unique ids that one server can generate per millisecond while being within the constraint of 64 bit.
    /// Choose above wisely as higher range per server gives better performance in high throughput systems.
    fn from(timestamp_bit_count: u64, node_id_bit_count: u64) -> Self {
        if timestamp_bit_count + node_id_bit_count + MIN_SEQUENCE_ID_BIT_COUNT > UID_BIT_COUNT {
            panic!("Unable to accomodate the given config in {UID_BIT_COUNT} bit id.")
        }
        Self {
            timestamp_bit_count,
            node_id_bit_count,
        }
    }

    fn timestamp_shift(&self) -> u64 {
        UID_BIT_COUNT - self.timestamp_bit_count
    }

    fn node_id_shift(&self) -> u64 {
        UID_BIT_COUNT - (self.timestamp_bit_count + self.node_id_bit_count)
    }

    fn pprint(&self) {
        let timestamp_bit_count = self.timestamp_bit_count;
        let node_id_bit_count = self.node_id_bit_count;
        let node_id_shift = self.node_id_shift();
        let max_load = 1 << (self.node_id_bit_count + self.node_id_shift());
        let max_nodes = 1 << self.node_id_bit_count;
        let max_load_per_node = 1 << self.node_id_shift();
        let max_lifetime_ms = 1 << self.timestamp_bit_count;
        let max_lifetime = max_lifetime_ms / MILLISECONDS_IN_YEAR;
        println!(
            "==============================================================================
            Initialising Snowflake Unique ID Generator Config
            ==============================================================================
            Config (64 bit ID)
            +------------------------------------------------------------------------+
            | 1 Bit Unused | {timestamp_bit_count} Bit Timestamp |  {node_id_bit_count} Bit NodeID  | {node_id_shift} Bit Sequence ID |
            +------------------------------------------------------------------------+
            Output (Load = requests per millisecond)
            +---------------------------------------------------------------------------------+
            | {max_lifetime} Years of uniqueness lifetime | {max_load} load across {max_nodes} nodes  | {max_load_per_node} load per node |
            +---------------------------------------------------------------------------------+
            ==============================================================================
        `"
        );
    }
}

// End Config
