use crate::models::PositionView;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PositionSnapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub positions: Vec<PositionView>,
    pub version: u32,
}

#[derive(Debug, Clone)]
pub struct PositionVersion {
    pub position_id: String,
    pub version: u32,
    pub data: PositionView,
    pub created_at: DateTime<Utc>,
    pub migration_applied: bool,
}

pub struct StateManager {
    snapshots: Vec<PositionSnapshot>,
    versions: HashMap<String, Vec<PositionVersion>>,
    current_version: u32,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            versions: HashMap::new(),
            current_version: 1,
        }
    }

    pub fn create_snapshot(&mut self, positions: Vec<PositionView>) -> String {
        let snapshot = PositionSnapshot {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            positions,
            version: self.current_version,
        };
        
        let id = snapshot.id.clone();
        self.snapshots.push(snapshot);
        id
    }

    pub fn restore_snapshot(&self, snapshot_id: &str) -> Option<Vec<PositionView>> {
        self.snapshots
            .iter()
            .find(|s| s.id == snapshot_id)
            .map(|s| s.positions.clone())
    }

    pub fn version_position(&mut self, position: PositionView) -> String {
        let position_id = format!("{}_{}", position.owner, position.symbol);
        let version = PositionVersion {
            position_id: position_id.clone(),
            version: self.current_version,
            data: position,
            created_at: Utc::now(),
            migration_applied: false,
        };

        self.versions
            .entry(position_id.clone())
            .or_insert_with(Vec::new)
            .push(version);

        position_id
    }

    pub fn get_position_history(&self, position_id: &str) -> Vec<PositionVersion> {
        self.versions
            .get(position_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn reconstruct_position(&self, position_id: &str, target_time: DateTime<Utc>) -> Option<PositionView> {
        let versions = self.versions.get(position_id)?;
        
        versions
            .iter()
            .filter(|v| v.created_at <= target_time)
            .max_by_key(|v| v.created_at)
            .map(|v| v.data.clone())
    }

    pub fn migrate_to_version(&mut self, target_version: u32) -> Vec<String> {
        let mut migrated = Vec::new();
        
        for (position_id, versions) in &mut self.versions {
            for version in versions {
                if version.version < target_version && !version.migration_applied {
                    // Apply migration logic here
                    version.migration_applied = true;
                    migrated.push(position_id.clone());
                }
            }
        }
        
        self.current_version = target_version;
        migrated
    }

    pub fn cleanup_old_versions(&mut self, keep_days: i64) {
        let cutoff = Utc::now() - chrono::Duration::days(keep_days);
        
        for versions in self.versions.values_mut() {
            versions.retain(|v| v.created_at > cutoff);
        }
        
        self.snapshots.retain(|s| s.timestamp > cutoff);
    }

    pub fn export_state(&self) -> String {
        // Simplified JSON export
        format!("{{\"version\":{},\"snapshots\":{},\"positions\":{}}}", 
                self.current_version, 
                self.snapshots.len(), 
                self.versions.len())
    }

    pub fn import_state(&mut self, _data: &str) -> Result<(), String> {
        // Simplified import - would parse JSON in real implementation
        Ok(())
    }
}