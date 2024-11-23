#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Voice Analytics System...${NC}"

# Function to check if Docker is running
check_docker() {
    if ! docker info >/dev/null 2>&1; then
        echo -e "${RED}Docker is not running. Please start Docker first.${NC}"
        exit 1
    fi
}

# Function to check environment files
check_env_files() {
    if [ ! -f .env ]; then
        echo -e "${RED}.env file not found. Please run setup.sh first.${NC}"
        exit 1
    fi

    if [ ! -f frontend/.env.local ]; then
        echo -e "${RED}frontend/.env.local not found. Please run setup.sh first.${NC}"
        exit 1
    fi
}

# Function to stop running services
stop_services() {
    echo -e "${YELLOW}Stopping any running services...${NC}"
    docker-compose down
}

# Function to start services
start_services() {
    echo -e "${YELLOW}Starting services...${NC}"
    
    # Start database and cache first
    echo "Starting PostgreSQL and Redis..."
    docker-compose up -d postgres redis
    
    # Wait for database to be ready
    echo "Waiting for database to be ready..."
    sleep 10
    
    # Start Rust backend
    echo "Starting Rust backend..."
    docker-compose up -d rust-backend
    
    # Start Python backend
    echo "Starting Python backend..."
    docker-compose up -d python-backend
    
    # Start monitoring services
    echo "Starting monitoring services..."
    docker-compose up -d prometheus grafana
    
    # Finally, start frontend
    echo "Starting frontend..."
    docker-compose up -d frontend
}

# Function to check service health
check_health() {
    echo -e "${YELLOW}Checking service health...${NC}"
    
    services=(
        "frontend:3000"
        "rust-backend:8000"
        "python-backend:8001"
        "postgres:5432"
        "redis:6379"
        "prometheus:9090"
        "grafana:3001"
    )
    
    for service in "${services[@]}"; do
        name="${service%%:*}"
        port="${service##*:}"
        
        echo -n "Checking $name... "
        
        if docker-compose ps | grep -q "$name.*Up"; then
            echo -e "${GREEN}✓${NC}"
        else
            echo -e "${RED}✗${NC}"
            echo -e "${YELLOW}Logs for $name:${NC}"
            docker-compose logs --tail=50 "$name"
        fi
    done
}

# Function to show resource usage
show_resources() {
    echo -e "${YELLOW}Resource Usage:${NC}"
    docker stats --no-stream $(docker-compose ps -q)
}

# Function to display logs
show_logs() {
    if [ -z "$1" ]; then
        docker-compose logs -f
    else
        docker-compose logs -f "$1"
    fi
}

# Main run function
main() {
    check_docker
    check_env_files
    stop_services
    start_services
    check_health
    
    echo -e "\n${GREEN}System is running!${NC}"
    echo -e "${YELLOW}Access points:${NC}"
    echo "Frontend: http://localhost:3000"
    echo "Rust Backend API: http://localhost:8000"
    echo "Python AI Services: http://localhost:8001"
    echo "Grafana Monitoring: http://localhost:3001 (admin/admin-password)"
    echo "Prometheus Metrics: http://localhost:9090"
    
    echo -e "\n${YELLOW}Available commands:${NC}"
    echo "View all logs: ./run.sh logs"
    echo "View specific service logs: ./run.sh logs <service-name>"
    echo "Show resource usage: ./run.sh stats"
    echo "Stop all services: ./run.sh stop"
}

# Command handling
case "$1" in
    "logs")
        show_logs "$2"
        ;;
    "stats")
        show_resources
        ;;
    "stop")
        stop_services
        ;;
    "health")
        check_health
        ;;
    *)
        main
        ;;
esac 