use crate::models::PositionView;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct PositionCache {
    cache: Arc<RwLock<HashMap<String, PositionView>>>,
}

impl PositionCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &str) -> Option<PositionView> {
        self.cache.read().await.get(key).cloned()
    }

    pub async fn set(&self, key: String, position: PositionView) {
        self.cache.write().await.insert(key, position);
    }

    pub async fn batch_update(&self, updates: Vec<(String, PositionView)>) {
        let mut cache = self.cache.write().await;
        for (key, position) in updates {
            cache.insert(key, position);
        }
    }
}

pub struct BatchProcessor {
    batch_size: usize,
}

impl BatchProcessor {
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

    pub async fn process_positions<F, Fut>(&self, positions: Vec<PositionView>, processor: F) 
    where
        F: Fn(Vec<PositionView>) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let chunks: Vec<_> = positions.chunks(self.batch_size).collect();
        
        let tasks: Vec<_> = chunks.into_iter().map(|chunk| {
            let chunk = chunk.to_vec();
            tokio::spawn(processor(chunk))
        }).collect();

        for task in tasks {
            let _ = task.await;
        }
    }
}

pub struct QueryOptimizer;

impl QueryOptimizer {
    pub fn build_position_query(symbols: &[String], owners: &[String]) -> String {
        format!(
            "SELECT * FROM positions WHERE symbol IN ({}) AND owner IN ({}) ORDER BY symbol, owner",
            symbols.iter().map(|_| "?").collect::<Vec<_>>().join(","),
            owners.iter().map(|_| "?").collect::<Vec<_>>().join(",")
        )
    }

    pub fn build_index_hints() -> Vec<String> {
        vec![
            "CREATE INDEX IF NOT EXISTS idx_positions_symbol ON positions(symbol)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_positions_owner ON positions(owner)".to_string(),
            "CREATE INDEX IF NOT EXISTS idx_positions_symbol_owner ON positions(symbol, owner)".to_string(),
        ]
    }
}