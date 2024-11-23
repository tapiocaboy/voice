#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up Voice Analytics System...${NC}"

# Check for required tools
check_requirements() {
    echo -e "${YELLOW}Checking requirements...${NC}"
    
    command -v docker >/dev/null 2>&1 || { 
        echo -e "${RED}Docker is required but not installed. Please install Docker first.${NC}" >&2
        exit 1
    }
    
    command -v docker-compose >/dev/null 2>&1 || {
        echo -e "${RED}Docker Compose is required but not installed. Please install Docker Compose first.${NC}" >&2
        exit 1
    }
}

# Create necessary directories
create_directories() {
    echo -e "${YELLOW}Creating project directories...${NC}"
    
    mkdir -p models
    mkdir -p audio_data
    mkdir -p logs
    mkdir -p monitoring/data
}

# Generate environment files
setup_environment() {
    echo -e "${YELLOW}Setting up environment...${NC}"
    
    if [ ! -f .env ]; then
        echo "Creating .env file..."
        cat > .env << EOL
# Database
POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=voice_analytics

# JWT
JWT_SECRET=$(openssl rand -hex 32)

# AI Services
HUGGING_FACE_TOKEN=your-hf-token-here
WHISPER_MODEL=base
COMPUTE_TYPE=int8

# Monitoring
GRAFANA_PASSWORD=admin-password

# Development
RUST_LOG=debug
NODE_ENV=development

# Resource Limits
MAX_AUDIO_SIZE=25MB
PROCESSING_TIMEOUT=300
EOL
    fi
    
    # Ask for Hugging Face token
    read -p "Enter your Hugging Face token (required for AI models): " hf_token
    if [ ! -z "$hf_token" ]; then
        sed -i "s/your-hf-token-here/$hf_token/" .env
    fi
}

# Install frontend dependencies
setup_frontend() {
    echo -e "${YELLOW}Setting up frontend...${NC}"
    
    cd frontend
    
    # Create package.json if it doesn't exist
    if [ ! -f package.json ]; then
        echo "Initializing frontend package.json..."
        cat > package.json << EOL
{
  "name": "voice-analytics-frontend",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
  },
  "dependencies": {
    "next": "14.0.0",
    "react": "18.2.0",
    "react-dom": "18.2.0",
    "wavesurfer.js": "^7.0.0",
    "zustand": "^4.5.0",
    "react-query": "^5.0.0",
    "socket.io-client": "^4.7.0",
    "d3": "^7.8.0",
    "recharts": "^2.10.0",
    "tailwindcss": "^3.3.0"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@types/react": "^18.2.0",
    "typescript": "^5.0.0",
    "jest": "^29.7.0",
    "@testing-library/react": "^14.1.0",
    "cypress": "^13.6.0"
  }
}
EOL
    fi
    
    cd ..
}

# Pull required Docker images
pull_images() {
    echo -e "${YELLOW}Pulling required Docker images...${NC}"
    
    docker pull python:3.10-slim
    docker pull rust:1.70
    docker pull node:18-alpine
    docker pull postgres:15-alpine
    docker pull redis:alpine
    docker pull prom/prometheus:latest
    docker pull grafana/grafana:latest
}

# Configure AI services for CPU
configure_ai_services() {
    echo -e "${YELLOW}Configuring AI services for CPU...${NC}"
    
    # Create Python requirements
    cat > python-backend/requirements.txt << EOL
fastapi==0.104.1
uvicorn==0.24.0
python-multipart==0.0.6
torch==2.1.0+cpu
torchaudio==2.1.0+cpu
--extra-index-url https://download.pytorch.org/whl/cpu
transformers>=4.36.0
numpy>=1.24.0
scipy>=1.11.0
librosa>=0.10.1
soundfile>=0.12.1
faster-whisper==0.9.0
speechbrain==0.5.16
prometheus-client==0.19.0
python-jose==3.3.0
EOL
}

# Build and start services
start_services() {
    echo -e "${YELLOW}Building and starting services...${NC}"
    
    # Build images
    docker-compose build
    
    # Start core services first
    docker-compose up -d postgres redis
    
    # Wait for database to be ready
    echo "Waiting for database to be ready..."
    sleep 10
    
    # Start remaining services
    docker-compose up -d
}

# Check service health
check_health() {
    echo -e "${YELLOW}Checking service health...${NC}"
    
    services=("frontend" "rust-backend" "python-backend" "postgres" "redis" "prometheus" "grafana")
    
    for service in "${services[@]}"; do
        if docker-compose ps | grep -q "$service.*Up"; then
            echo -e "${GREEN}✓ $service is running${NC}"
        else
            echo -e "${RED}✗ $service is not running${NC}"
            docker-compose logs "$service"
        fi
    done
}

# Main setup flow
main() {
    check_requirements
    create_directories
    setup_environment
    setup_frontend
    configure_ai_services
    pull_images
    start_services
    check_health
    
    echo -e "${GREEN}Setup complete!${NC}"
    echo -e "${YELLOW}Access the application:${NC}"
    echo "Frontend: http://localhost:3000"
    echo "Grafana: http://localhost:3001 (admin/admin-password)"
    echo "Prometheus: http://localhost:9090"
    echo -e "\n${YELLOW}Note: Using CPU-optimized AI models. Processing may be slower but more compatible.${NC}"
}

# Run setup
main 