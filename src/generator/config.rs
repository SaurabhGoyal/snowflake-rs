const UID_BIT_COUNT: u64 = 63;
const MILLISECONDS_IN_YEAR: u64 = 1_000 * 86_400 * 365;

const DEFAULT_TIMESTAMP_BIT_COUNT: u64 = 42;
const DEFAULT_NODE_ID_BIT_COUNT: u64 = 11;
const MIN_SEQUENCE_ID_BIT_COUNT: u64 = 4;

/// Config controls the UID generation, mainly controlling the number of nodes and number of
/// unique ids per millisecond that a node can generate. This is tradeoff between how fast
/// you need to generate unique ids and across how many servers.
pub struct Config {
    timestamp_bit_count: u64,
    node_id_bit_count: u64,
}

impl Config {
    /// Config::default generates a config with default values of 42 bits for timestamp and 11 bits for nodes
    /// giving a millisecond window with 2048 nodes at a time. This can be customised using the `from` method.
    pub fn default() -> Self {
        Self::from(DEFAULT_TIMESTAMP_BIT_COUNT, DEFAULT_NODE_ID_BIT_COUNT)
    }

    /// Config::from generates a config to be used by generator to generate IDs.
    ///  - Use higher or lower timestamp_bit_count based on how long (generally in years) the lifecycle of a generated unique id should be.
    ///  - Use higher or lower node_id_bit_count based on how many servers are going to be involved in unique id generation.
    ///  - Above two values directly impact the range of unique ids that one server can generate per millisecond while being within the constraint of 64 bit.
    /// Choose above wisely as higher range per server gives better performance in high throughput systems.
    pub fn from(timestamp_bit_count: u64, node_id_bit_count: u64) -> Self {
        if timestamp_bit_count + node_id_bit_count + MIN_SEQUENCE_ID_BIT_COUNT > UID_BIT_COUNT {
            panic!("Unable to accomodate the given config in {UID_BIT_COUNT} bit id.")
        }
        Self {
            timestamp_bit_count,
            node_id_bit_count,
        }
    }

    pub fn timestamp_bit_count(&self) -> u64 {
        self.timestamp_bit_count
    }

    pub fn node_id_bit_count(&self) -> u64 {
        self.node_id_bit_count
    }

    pub fn timestamp_shift(&self) -> u64 {
        UID_BIT_COUNT - self.timestamp_bit_count
    }

    pub fn node_id_shift(&self) -> u64 {
        UID_BIT_COUNT - (self.timestamp_bit_count + self.node_id_bit_count)
    }

    pub fn pprint(&self) {
        let timestamp_bit_count = self.timestamp_bit_count;
        let node_id_bit_count = self.node_id_bit_count;
        let node_id_shift = self.node_id_shift();
        let max_load = 1 << (self.node_id_bit_count + self.node_id_shift());
        let max_nodes = 1 << self.node_id_bit_count;
        let max_load_per_node = 1 << self.node_id_shift();
        let max_lifetime_ms = 1 << self.timestamp_bit_count;
        let max_lifetime = max_lifetime_ms / MILLISECONDS_IN_YEAR;
        println!(
            "
==============================================================================
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
