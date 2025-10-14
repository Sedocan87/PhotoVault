-- Create the sync_operations table for journaling
CREATE TABLE IF NOT EXISTS sync_operations (
    id TEXT PRIMARY KEY,
    operation_type TEXT NOT NULL,
    params TEXT NOT NULL, -- Storing params as JSON string
    status TEXT NOT NULL DEFAULT 'pending', -- pending, completed, failed
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT
);