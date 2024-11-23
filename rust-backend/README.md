# Voice Analytics Rust Backend (Rust)

## Architecture Overview

```mermaid
graph TD
    subgraph External["External Services"]
        DB[(PostgreSQL)]
        CACHE[(Redis)]
    end

    subgraph HTTP["HTTP Layer"]
        API[HTTP Server]
        CORS[CORS Middleware]
        SWAGGER[Swagger UI]
    end

    subgraph Handlers["Request Handlers"]
        HEALTH[Health Handler]
        AUDIO[Audio Handler]
        ANALYSIS[Analysis Handler]
    end

    subgraph Services["Business Logic"]
        AP[Audio Processor]
        subgraph Processing["Audio Processing"]
            VAL[Validation]
            CONV[Conversion]
            FILTER[Filtering]
        end
    end

    subgraph Models["Data Models"]
        AM[Audio Models]
        CM[Config Models]
        SM[State Models]
    end

    subgraph Utils["Utilities"]
        ERR[Error Handling]
        LOG[Logging]
    end

    %% External connections
    API --> DB
    API --> CACHE

    %% HTTP Layer connections
    API --> CORS
    API --> SWAGGER
    API --> Handlers

    %% Handler connections
    AUDIO --> AP
    ANALYSIS --> AP
    HEALTH --> DB
    HEALTH --> CACHE

    %% Service connections
    AP --> VAL
    AP --> CONV
    AP --> FILTER

    %% Model usage
    Handlers --> AM
    Handlers --> SM
    Services --> AM
    API --> CM

    %% Utility usage
    Handlers --> ERR
    Services --> ERR
    API --> LOG

    style External fill:#f9f,stroke:#333,stroke-width:2px
    style HTTP fill:#bbf,stroke:#333,stroke-width:2px
    style Handlers fill:#bfb,stroke:#333,stroke-width:2px
    style Services fill:#fbf,stroke:#333,stroke-width:2px
    style Models fill:#fbb,stroke:#333,stroke-width:2px
    style Utils fill:#bff,stroke:#333,stroke-width:2px
```

## Directory Structure

```
rust-backend/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── config/                 # Configuration management
│   │   └── mod.rs
│   ├── handlers/              # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   ├── audio.rs
│   │   └── analysis.rs
│   ├── models/               # Data structures
│   │   ├── mod.rs
│   │   └── audio.rs
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   └── audio_processor.rs
│   └── utils/              # Utility functions
│       └── mod.rs
├── Cargo.toml              # Dependencies and project metadata
└── .env                    # Environment configuration
```

## Key Components

1. **HTTP Layer**
   - Actix-web server
   - CORS middleware
   - Swagger UI documentation
   - Request routing

2. **Handlers**
   - Health check endpoints
   - Audio processing endpoints
   - Analysis endpoints

3. **Services**
   - Audio processing
   - Data validation
   - Format conversion
   - Signal filtering

4. **Models**
   - Audio data structures
   - Configuration types
   - Application state

5. **External Services**
   - PostgreSQL database
   - Redis cache

## API Documentation

Swagger UI is available at: `http://localhost:8000/swagger-ui/`

## Getting Started

1. Install dependencies:
```bash
cargo build
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Run the server:
```bash
cargo run
```

## Development

- **Testing**: `cargo test`
- **Format code**: `cargo fmt`
- **Check lints**: `cargo clippy`
- **Build documentation**: `cargo doc`

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| DATABASE_URL | PostgreSQL connection URL | postgres://user:password@localhost:5432/voice_analytics |
| REDIS_URL | Redis connection URL | redis://localhost:6379 |
| PORT | Server port | 8000 |
| RUST_LOG | Logging level | debug |

## Error Handling

The application uses a custom error type `ServiceError` that maps to appropriate HTTP responses:
- Internal Server Error (500)
- Authentication Error (401)
- Validation Error (400)
- Audio Processing Error (422)
- Database Error (500)
- Not Found (404) 