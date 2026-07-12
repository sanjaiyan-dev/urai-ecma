pub struct CacheEntry {
    pub modified_time: u64,
    pub size: u64,
    pub pruned_code: String,
    pub token_count: usize,
}
