# System Monitor WebSocket Design

## Overview
The system monitor WebSocket API provides real-time system status updates to authorized administrators through a WebSocket connection. This design describes the components and their interactions for implementing this functionality.

## Components

### SystemStatusBroadcaster
A new broadcaster service that manages WebSocket connections and broadcasts system status updates:
```rust
pub struct SystemStatusBroadcaster {
    channel: Arc<RwLock<broadcast::Sender<SystemStatus>>>
}
```

### WebSocket Handler
A new WebSocket handler in the monitor API that:
1. Validates admin authorization
2. Establishes WebSocket connection
3. Subscribes to system status updates
4. Sends updates to connected clients

### System Status Collection Task
A background task that:
1. Periodically collects system status (every 5 seconds)
2. Broadcasts updates to all connected WebSocket clients

## Implementation Steps

1. Update `monitor.rs` service:
   - Add SystemStatusBroadcaster implementation
   - Add functions to create and manage broadcast channel
   - Implement periodic status collection task

2. Update `monitor.rs` API:
   - Add WebSocket handler endpoint
   - Implement connection handling and status broadcasting
   - Maintain admin-only access control

3. Update `AppState`:
   - Add SystemStatusBroadcaster instance
   - Initialize broadcaster and background task on startup

## Data Flow

1. Client initiates WebSocket connection with admin credentials
2. Server validates admin role and establishes connection
3. Background task collects system status every 5 seconds
4. Status updates are broadcast to all connected admin clients
5. Clients receive and process status updates in real-time

## Security Considerations

- Only administrators can connect to the WebSocket endpoint
- All connections are authenticated using existing auth middleware
- Invalid or expired credentials result in connection termination

## Error Handling

- Connection errors trigger automatic reconnection attempts
- Invalid messages are logged and ignored
- System status collection errors are handled gracefully

## Future Improvements

- Add configurable update interval
- Implement message compression for large scale deployments
- Add selective metric subscription capability