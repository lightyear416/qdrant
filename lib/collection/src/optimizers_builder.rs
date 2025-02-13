use crate::collection_manager::optimizers::indexing_optimizer::IndexingOptimizer;
use crate::collection_manager::optimizers::merge_optimizer::MergeOptimizer;
use crate::collection_manager::optimizers::segment_optimizer::OptimizerThresholds;
use crate::collection_manager::optimizers::vacuum_optimizer::VacuumOptimizer;
use crate::config::CollectionParams;
use crate::update_handler::Optimizer;
use schemars::JsonSchema;
use segment::types::HnswConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
pub struct OptimizersConfig {
    /// The minimal fraction of deleted vectors in a segment, required to perform segment optimization
    pub deleted_threshold: f64,
    /// The minimal number of vectors in a segment, required to perform segment optimization
    pub vacuum_min_vector_number: usize,
    /// Target amount of segments optimizer will try to keep.
    /// Real amount of segments may vary depending on multiple parameters:
    ///  - Amount of stored points
    ///  - Current write RPS
    ///
    /// It is recommended to select default number of segments as a factor of the number of search threads,
    /// so that each segment would be handled evenly by one of the threads
    pub default_segment_number: usize,
    /// Do not create segments larger this number of points.
    /// Large segments might require disproportionately long indexation times,
    /// therefore it makes sense to limit the size of segments.
    ///
    /// If indexation speed have more priority for your - make this parameter lower.
    /// If search speed is more important - make this parameter higher.
    pub max_segment_size: usize,
    /// Maximum number of vectors to store in-memory per segment.
    /// Segments larger than this threshold will be stored as read-only memmaped file.
    pub memmap_threshold: usize,
    /// Maximum number of vectors allowed for plain index.
    /// Default value based on https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md
    pub indexing_threshold: usize,
    /// Starting from this amount of vectors per-segment the engine will start building index for payload.
    pub payload_indexing_threshold: usize,
    /// Minimum interval between forced flushes.
    pub flush_interval_sec: u64,
    /// Maximum available threads for optimization workers
    pub max_optimization_threads: usize,
}

pub fn build_optimizers(
    shard_path: &Path,
    collection_params: &CollectionParams,
    optimizers_config: &OptimizersConfig,
    hnsw_config: &HnswConfig,
) -> Arc<Vec<Arc<Optimizer>>> {
    let segments_path = shard_path.join("segments");
    let temp_segments_path = shard_path.join("temp_segments");

    let threshold_config = OptimizerThresholds {
        memmap_threshold: optimizers_config.memmap_threshold,
        indexing_threshold: optimizers_config.indexing_threshold,
        payload_indexing_threshold: optimizers_config.payload_indexing_threshold,
    };

    Arc::new(vec![
        Arc::new(MergeOptimizer::new(
            optimizers_config.default_segment_number,
            optimizers_config.max_segment_size,
            threshold_config.clone(),
            segments_path.clone(),
            temp_segments_path.clone(),
            collection_params.clone(),
            *hnsw_config,
        )),
        Arc::new(IndexingOptimizer::new(
            threshold_config.clone(),
            segments_path.clone(),
            temp_segments_path.clone(),
            collection_params.clone(),
            *hnsw_config,
        )),
        Arc::new(VacuumOptimizer::new(
            optimizers_config.deleted_threshold,
            optimizers_config.vacuum_min_vector_number,
            threshold_config,
            segments_path,
            temp_segments_path,
            collection_params.clone(),
            *hnsw_config,
        )),
    ])
}
