use crate::db::manager::DatabaseManager;
use crate::models::operation::Operation;
use std::collections::VecDeque;

pub struct SyncEngine {
    pub db_manager: DatabaseManager,
    pub operation_queue: VecDeque<Operation>,
}

impl SyncEngine {
    pub fn new(db_manager: DatabaseManager) -> Self {
        Self {
            db_manager,
            operation_queue: VecDeque::new(),
        }
    }

    pub async fn execute_operation(&mut self, op: Operation) -> Result<(), sqlx::Error> {
        match self.log_operation(&op, "pending").await {
            Ok(_) => {
                // Log succeeded, now execute
                match self.execute_on_both(op.clone()).await {
                    Ok(_) => {
                        // Execution succeeded, log completed
                        self.log_operation(&op, "completed").await
                    }
                    Err(e) => {
                        // Execution failed, queue
                        self.handle_backup_disconnected(op).await?;
                        Err(e)
                    }
                }
            }
            Err(e) => {
                // Logging failed, queue
                self.handle_backup_disconnected(op).await?;
                Err(e)
            }
        }
    }

    async fn execute_on_both(&mut self, op: Operation) -> Result<(), sqlx::Error> {
        self.db_manager
            .execute_on_both(|pool| {
                let op = op.clone();
                Box::pin(async move {
                    match op {
                        Operation::CreateAlbum { name } => {
                            sqlx::query("INSERT INTO albums (name) VALUES (?)")
                                .bind(name)
                                .execute(pool)
                                .await?;
                        }
                        _ => {}
                    }
                    Ok(())
                })
            })
            .await
    }

    async fn handle_backup_disconnected(&mut self, op: Operation) -> Result<(), sqlx::Error> {
        self.operation_queue.push_back(op);
        // In a real implementation, we would have a mechanism to flush the queue
        // when the backup drive is reconnected.
        Ok(())
    }

    pub async fn flush_queue(&mut self) -> Result<(), sqlx::Error> {
        while let Some(op) = self.operation_queue.pop_front() {
            if let Ok(_) = self.execute_on_both(op.clone()).await {
                self.log_operation(&op, "completed").await?;
            } else {
                // If the operation fails again, we push it back to the front of the queue
                // to be retried later.
                self.operation_queue.push_front(op);
                break;
            }
        }
        Ok(())
    }

    async fn log_operation(&self, op: &Operation, status: &str) -> Result<(), sqlx::Error> {
        let op_str = serde_json::to_string(op).unwrap();
        sqlx::query("INSERT INTO sync_operations (operation, status, created_at) VALUES (?, ?, ?)")
            .bind(op_str)
            .bind(status)
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&self.db_manager.primary_db)
            .await?;
        if let Some(backup_db) = &self.db_manager.backup_db {
            sqlx::query("INSERT INTO sync_operations (operation, status, created_at) VALUES (?, ?, ?)")
                .bind(serde_json::to_string(op).unwrap())
                .bind(status)
                .bind(chrono::Utc::now().to_rfc3339())
                .execute(backup_db)
                .await?;
        }
        Ok(())
    }
}