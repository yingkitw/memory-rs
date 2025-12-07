//! Distributed memory management

use async_trait::async_trait;
use crate::Result;

/// Node role in the cluster
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    /// Primary node (leader)
    Primary,
    /// Secondary node (replica)
    Secondary,
    /// Arbiter node (for consensus)
    Arbiter,
}

impl NodeRole {
    /// Get role name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Arbiter => "arbiter",
        }
    }
}

/// Node information
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Node ID
    pub id: String,
    /// Node address
    pub address: String,
    /// Node role
    pub role: NodeRole,
    /// Is node healthy
    pub healthy: bool,
    /// Last heartbeat timestamp
    pub last_heartbeat: i64,
}

impl NodeInfo {
    /// Create a new node info
    pub fn new(id: String, address: String, role: NodeRole) -> Self {
        Self {
            id,
            address,
            role,
            healthy: true,
            last_heartbeat: chrono::Utc::now().timestamp(),
        }
    }
}

/// Replication strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationStrategy {
    /// No replication
    None,
    /// Single replica
    Single,
    /// Multiple replicas
    Multiple(usize),
}

impl ReplicationStrategy {
    /// Get replica count
    pub fn replica_count(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Single => 1,
            Self::Multiple(count) => *count,
        }
    }
}

/// Sharding strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShardingStrategy {
    /// Range-based sharding
    Range,
    /// Hash-based sharding
    Hash,
    /// Directory-based sharding
    Directory,
}

impl ShardingStrategy {
    /// Get strategy name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Range => "range",
            Self::Hash => "hash",
            Self::Directory => "directory",
        }
    }
}

/// Shard information
#[derive(Debug, Clone)]
pub struct ShardInfo {
    /// Shard ID
    pub id: usize,
    /// Shard range (for range-based sharding)
    pub range: Option<(String, String)>,
    /// Primary node for this shard
    pub primary_node: String,
    /// Replica nodes for this shard
    pub replica_nodes: Vec<String>,
}

impl ShardInfo {
    /// Create a new shard info
    pub fn new(id: usize, primary_node: String) -> Self {
        Self {
            id,
            range: None,
            primary_node,
            replica_nodes: Vec::new(),
        }
    }

    /// Add replica node
    pub fn add_replica(mut self, node_id: String) -> Self {
        self.replica_nodes.push(node_id);
        self
    }
}

/// Consensus protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsensusProtocol {
    /// Raft consensus
    Raft,
    /// Paxos consensus
    Paxos,
    /// Quorum-based consensus
    Quorum,
}

impl ConsensusProtocol {
    /// Get protocol name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Raft => "raft",
            Self::Paxos => "paxos",
            Self::Quorum => "quorum",
        }
    }
}

/// Distributed configuration
#[derive(Debug, Clone)]
pub struct DistributedConfig {
    /// Node ID
    pub node_id: String,
    /// Cluster name
    pub cluster_name: String,
    /// Replication strategy
    pub replication: ReplicationStrategy,
    /// Sharding strategy
    pub sharding: ShardingStrategy,
    /// Consensus protocol
    pub consensus: ConsensusProtocol,
    /// Number of shards
    pub shard_count: usize,
}

impl DistributedConfig {
    /// Create a new distributed config
    pub fn new(node_id: String, cluster_name: String) -> Self {
        Self {
            node_id,
            cluster_name,
            replication: ReplicationStrategy::Single,
            sharding: ShardingStrategy::Hash,
            consensus: ConsensusProtocol::Raft,
            shard_count: 16,
        }
    }

    /// Set replication strategy
    pub fn with_replication(mut self, replication: ReplicationStrategy) -> Self {
        self.replication = replication;
        self
    }

    /// Set sharding strategy
    pub fn with_sharding(mut self, sharding: ShardingStrategy) -> Self {
        self.sharding = sharding;
        self
    }

    /// Set consensus protocol
    pub fn with_consensus(mut self, consensus: ConsensusProtocol) -> Self {
        self.consensus = consensus;
        self
    }

    /// Set shard count
    pub fn with_shard_count(mut self, count: usize) -> Self {
        self.shard_count = count;
        self
    }
}

/// Base trait for distributed store implementations
#[async_trait]
pub trait DistributedStoreBase: Send + Sync {
    /// Join cluster
    async fn join_cluster(&self, seed_nodes: Vec<String>) -> Result<()>;

    /// Leave cluster
    async fn leave_cluster(&self) -> Result<()>;

    /// Get cluster status
    async fn cluster_status(&self) -> Result<ClusterStatus>;

    /// Get node list
    async fn get_nodes(&self) -> Result<Vec<NodeInfo>>;

    /// Get shard list
    async fn get_shards(&self) -> Result<Vec<ShardInfo>>;

    /// Replicate data to node
    async fn replicate(&self, node_id: &str, data: Vec<u8>) -> Result<()>;

    /// Get replication status
    async fn replication_status(&self) -> Result<ReplicationStatus>;

    /// Trigger rebalancing
    async fn rebalance(&self) -> Result<()>;

    /// Get node health
    async fn node_health(&self, node_id: &str) -> Result<bool>;
}

/// Cluster status
#[derive(Debug, Clone)]
pub struct ClusterStatus {
    /// Cluster name
    pub name: String,
    /// Total nodes
    pub total_nodes: usize,
    /// Healthy nodes
    pub healthy_nodes: usize,
    /// Total shards
    pub total_shards: usize,
    /// Leader node
    pub leader: Option<String>,
}

/// Replication status
#[derive(Debug, Clone)]
pub struct ReplicationStatus {
    /// Total items replicated
    pub total_replicated: usize,
    /// Pending replications
    pub pending: usize,
    /// Failed replications
    pub failed: usize,
    /// Replication lag (ms)
    pub lag_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_role_names() {
        assert_eq!(NodeRole::Primary.name(), "primary");
        assert_eq!(NodeRole::Secondary.name(), "secondary");
        assert_eq!(NodeRole::Arbiter.name(), "arbiter");
    }

    #[test]
    fn test_node_info_creation() {
        let node = NodeInfo::new(
            "node_1".to_string(),
            "localhost:7000".to_string(),
            NodeRole::Primary,
        );

        assert_eq!(node.id, "node_1");
        assert_eq!(node.role, NodeRole::Primary);
        assert!(node.healthy);
    }

    #[test]
    fn test_replication_strategy() {
        assert_eq!(ReplicationStrategy::None.replica_count(), 0);
        assert_eq!(ReplicationStrategy::Single.replica_count(), 1);
        assert_eq!(ReplicationStrategy::Multiple(3).replica_count(), 3);
    }

    #[test]
    fn test_sharding_strategy_names() {
        assert_eq!(ShardingStrategy::Range.name(), "range");
        assert_eq!(ShardingStrategy::Hash.name(), "hash");
        assert_eq!(ShardingStrategy::Directory.name(), "directory");
    }

    #[test]
    fn test_shard_info_creation() {
        let shard = ShardInfo::new(0, "node_1".to_string())
            .add_replica("node_2".to_string());

        assert_eq!(shard.id, 0);
        assert_eq!(shard.replica_nodes.len(), 1);
    }

    #[test]
    fn test_distributed_config() {
        let config = DistributedConfig::new(
            "node_1".to_string(),
            "cluster_1".to_string(),
        )
        .with_shard_count(32)
        .with_replication(ReplicationStrategy::Multiple(3));

        assert_eq!(config.shard_count, 32);
        assert_eq!(config.replication.replica_count(), 3);
    }

    #[test]
    fn test_consensus_protocol_names() {
        assert_eq!(ConsensusProtocol::Raft.name(), "raft");
        assert_eq!(ConsensusProtocol::Paxos.name(), "paxos");
        assert_eq!(ConsensusProtocol::Quorum.name(), "quorum");
    }
}
