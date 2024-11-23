#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}Starting development databases...${NC}"

# Check if docker-compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "docker-compose is not installed. Please install it first."
    exit 1
fi

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -i :$port > /dev/null; then
        echo -e "${RED}Port $port is already in use${NC}"
        echo "Running processes on port $port:"
        lsof -i :$port
        return 1
    fi
    return 0
}

# Function to check service health
check_service() {
    local service=$1
    local max_attempts=60  # Increased timeout
    local attempt=1

    echo -e "${YELLOW}Waiting for $service to be ready...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if docker-compose -f docker-compose.dev.yml ps | grep -q "${service}.*healthy"; then
            echo -e "${GREEN}$service is ready!${NC}"
            return 0
        fi
        echo -n "."
        sleep 1
        attempt=$((attempt + 1))
    done

    echo -e "\n${RED}$service failed to become ready in time${NC}"
    return 1
}

# Function to cleanup existing containers
cleanup() {
    echo -e "${YELLOW}Cleaning up existing containers...${NC}"
    docker-compose -f docker-compose.dev.yml down --remove-orphans
    docker network rm voice-network 2>/dev/null || true
    sleep 2
}

# Function to test database connection
test_postgres() {
    echo -e "${YELLOW}Testing PostgreSQL connection...${NC}"
    docker exec voice-postgres pg_isready -U user -d voice_analytics
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}PostgreSQL connection successful${NC}"
        return 0
    else
        echo -e "${RED}PostgreSQL connection failed${NC}"
        return 1
    fi
}

# Function to test Redis connection
test_redis() {
    echo -e "${YELLOW}Testing Redis connection...${NC}"
    docker exec voice-redis redis-cli ping
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Redis connection successful${NC}"
        return 0
    else
        echo -e "${RED}Redis connection failed${NC}"
        return 1
    fi
}

case "$1" in
    "start")
        # Check ports before starting
        check_port 5432 || exit 1
        check_port 6379 || exit 1

        # Cleanup existing containers
        cleanup

        # Create network if it doesn't exist
        docker network create voice-network 2>/dev/null || true

        # Start services
        docker-compose -f docker-compose.dev.yml up -d

        # Wait for services to be ready
        check_service postgres || exit 1
        check_service redis || exit 1

        # Test connections
        sleep 5
        test_postgres || exit 1
        test_redis || exit 1

        echo -e "${GREEN}Development databases are ready!${NC}"
        echo -e "PostgreSQL is available at localhost:5432"
        echo -e "Redis is available at localhost:6379"
        
        # Print connection strings
        echo -e "\n${YELLOW}Connection strings:${NC}"
        echo "DATABASE_URL=postgres://user:password@localhost:5432/voice_analytics"
        echo "REDIS_URL=redis://localhost:6379"

        # Create .env file for rust-backend
        echo -e "\n${YELLOW}Creating .env file for rust-backend...${NC}"
        cat > rust-backend/.env << EOL
DATABASE_URL=postgres://user:password@localhost:5432/voice_analytics
REDIS_URL=redis://localhost:6379
PORT=8000
JWT_SECRET=your-secret-key-here
RUST_LOG=debug
EOL
        ;;
    "stop")
        echo -e "${YELLOW}Stopping development databases...${NC}"
        docker-compose -f docker-compose.dev.yml down
        echo -e "${GREEN}Development databases stopped${NC}"
        ;;
    "logs")
        docker-compose -f docker-compose.dev.yml logs -f
        ;;
    "clean")
        echo -e "${YELLOW}Cleaning up development databases...${NC}"
        docker-compose -f docker-compose.dev.yml down -v --remove-orphans
        docker network rm voice-network 2>/dev/null || true
        echo -e "${GREEN}Development databases cleaned${NC}"
        ;;
    "restart")
        $0 stop
        sleep 2
        $0 start
        ;;
    *)
        echo "Usage: $0 {start|stop|logs|clean|restart}"
        echo "  start   - Start development databases"
        echo "  stop    - Stop development databases"
        echo "  logs    - Show database logs"
        echo "  clean   - Stop databases and remove volumes"
        echo "  restart - Restart all services"
        exit 1
        ;;
esac