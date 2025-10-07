# Tax Platform

Event-driven microservices platform for tax calculation and data processing.

## Architecture

This platform follows Clean Architecture principles with event-driven communication between services.

### Services

- **file-validator** - Validates and processes incoming tax files
- **load-coordinator** - Coordinates the loading process across different data types
- **tax-loader** - Loads tax data into the system
- **snapshot-manager** - Manages data snapshots and versioning

### Libraries

- **taxonomy** - Core domain models and business logic
- **orchestrator-models** - Shared DTOs and events
- **utils** - Common utilities and helpers

### Queues

- `tax.load.queue` - Tax data loading queue
- `curr.load.queue` - Currency data loading queue
- `geo.load.queue` - Geography data loading queue
- `rbd.load.queue` - RBD data loading queue
- `load.completion.queue` - Load completion notifications
- `load.error.queue` - Error handling queue

## Getting Started

```bash
# Build all services
cargo build

# Run tests
cargo test

# Run specific service
cargo run -p file-validator
```

## Event Flow

1. **File Validator** → Validates incoming files
2. **Load Coordinator** → Coordinates loading process
3. **Tax Loader** → Loads tax data
4. **Snapshot Manager** → Manages snapshots

## Technology Stack

- **Rust** - Primary language
- **Tokio** - Async runtime
- **Axum** - Web framework
- **SQLx** - Database access
- **Kafka** - Message queuing
- **AWS S3** - File storage
- **PostgreSQL** - Database
