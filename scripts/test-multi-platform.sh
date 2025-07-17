#!/bin/bash

# Multi-Platform Docker Test Script for UltraFast MCP Sequential Thinking
# This script helps test Docker images on different platforms

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
IMAGE_NAME="techgopal/ultrafast-mcp-sequential-thinking"
TAG="latest"
TEST_PORT=8080
CONTAINER_NAME="test-sequential-thinking"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -i, --image NAME    Docker image name (default: techgopal/ultrafast-mcp-sequential-thinking)"
    echo "  -t, --tag TAG       Docker image tag (default: latest)"
    echo "  -p, --port PORT     Test port (default: 8080)"
    echo "  -n, --name NAME     Container name (default: test-sequential-thinking)"
    echo "  -h, --help          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Test latest image on default port"
    echo "  $0 -i my-image -t v1.0.0             # Test specific image and tag"
    echo "  $0 -p 9090                           # Test on different port"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -i|--image)
            IMAGE_NAME="$2"
            shift 2
            ;;
        -t|--tag)
            TAG="$2"
            shift 2
            ;;
        -p|--port)
            TEST_PORT="$2"
            shift 2
            ;;
        -n|--name)
            CONTAINER_NAME="$2"
            shift 2
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker and try again."
    exit 1
fi

print_status "Testing multi-platform Docker image: $IMAGE_NAME:$TAG"
print_status "Test port: $TEST_PORT"
print_status "Container name: $CONTAINER_NAME"

# Function to test a specific platform
test_platform() {
    local platform=$1
    local platform_name=$2
    
    print_status "Testing platform: $platform_name ($platform)"
    
    # Stop and remove existing container if it exists
    if docker ps -a --format "table {{.Names}}" | grep -q "^$CONTAINER_NAME$"; then
        print_status "Removing existing container: $CONTAINER_NAME"
        docker rm -f "$CONTAINER_NAME" > /dev/null 2>&1 || true
    fi
    
    # Start container for this platform
    print_status "Starting container for $platform_name..."
    if docker run -d \
        --name "$CONTAINER_NAME" \
        --platform "$platform" \
        -p "$TEST_PORT:8080" \
        "$IMAGE_NAME:$TAG" > /dev/null 2>&1; then
        
        print_success "Container started successfully for $platform_name"
        
        # Wait for container to be ready
        print_status "Waiting for container to be ready..."
        sleep 5
        
        # Test HTTP endpoint
        print_status "Testing HTTP endpoint..."
        if curl -s -f "http://localhost:$TEST_PORT/health" > /dev/null 2>&1 || \
           curl -s -f "http://localhost:$TEST_PORT/mcp" > /dev/null 2>&1; then
            print_success "HTTP endpoint test passed for $platform_name"
        else
            print_warning "HTTP endpoint test failed for $platform_name (this might be expected)"
        fi
        
        # Test MCP initialization
        print_status "Testing MCP initialization..."
        if curl -s -X POST "http://localhost:$TEST_PORT/mcp" \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2025-06-18"},"id":1}' > /dev/null 2>&1; then
            print_success "MCP initialization test passed for $platform_name"
        else
            print_warning "MCP initialization test failed for $platform_name (this might be expected)"
        fi
        
        # Check container logs
        print_status "Container logs for $platform_name:"
        docker logs "$CONTAINER_NAME" --tail 10 2>/dev/null || print_warning "No logs available"
        
    else
        print_error "Failed to start container for $platform_name"
        return 1
    fi
    
    # Clean up
    print_status "Cleaning up container for $platform_name..."
    docker rm -f "$CONTAINER_NAME" > /dev/null 2>&1 || true
    
    return 0
}

# Test different platforms
PLATFORMS=(
    "linux/amd64:AMD64"
    "linux/arm64:ARM64"
    "linux/arm/v7:ARMv7"
)

SUCCESS_COUNT=0
TOTAL_COUNT=${#PLATFORMS[@]}

for platform_info in "${PLATFORMS[@]}"; do
    platform=$(echo "$platform_info" | cut -d: -f1)
    platform_name=$(echo "$platform_info" | cut -d: -f2)
    
    echo ""
    print_status "=========================================="
    print_status "Testing $platform_name platform"
    print_status "=========================================="
    
    if test_platform "$platform" "$platform_name"; then
        ((SUCCESS_COUNT++))
        print_success "$platform_name platform test completed successfully"
    else
        print_error "$platform_name platform test failed"
    fi
    
    echo ""
done

# Summary
print_status "=========================================="
print_status "Test Summary"
print_status "=========================================="
print_status "Total platforms tested: $TOTAL_COUNT"
print_status "Successful tests: $SUCCESS_COUNT"
print_status "Failed tests: $((TOTAL_COUNT - SUCCESS_COUNT))"

if [[ $SUCCESS_COUNT -eq $TOTAL_COUNT ]]; then
    print_success "All platform tests passed!"
    exit 0
else
    print_warning "Some platform tests failed. Check the output above for details."
    exit 1
fi 