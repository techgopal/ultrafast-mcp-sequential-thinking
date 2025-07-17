#!/bin/bash

# Multi-Platform Docker Build Script for UltraFast MCP Sequential Thinking
# This script helps build Docker images for multiple platforms

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
IMAGE_NAME="ultrafast-mcp-sequential-thinking"
TAG="latest"
PLATFORMS="linux/amd64,linux/arm64,linux/arm/v7"
PUSH=false
BUILDX_BUILDER="multiplatform"

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
    echo "  -n, --name NAME     Docker image name (default: ultrafast-mcp-sequential-thinking)"
    echo "  -t, --tag TAG       Docker image tag (default: latest)"
    echo "  -p, --platforms     Comma-separated list of platforms (default: linux/amd64,linux/arm64,linux/arm/v7)"
    echo "  --push              Push images to registry after building"
    echo "  --builder NAME      Docker Buildx builder name (default: multiplatform)"
    echo "  -h, --help          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Build for all platforms, don't push"
    echo "  $0 --push                            # Build for all platforms and push"
    echo "  $0 -p linux/amd64,linux/arm64 --push # Build for specific platforms and push"
    echo "  $0 -n my-image -t v1.0.0 --push      # Build with custom name and tag"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -n|--name)
            IMAGE_NAME="$2"
            shift 2
            ;;
        -t|--tag)
            TAG="$2"
            shift 2
            ;;
        -p|--platforms)
            PLATFORMS="$2"
            shift 2
            ;;
        --push)
            PUSH=true
            shift
            ;;
        --builder)
            BUILDX_BUILDER="$2"
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

# Check if we're in the right directory
if [[ ! -f "Dockerfile" ]]; then
    print_error "Dockerfile not found. Please run this script from the project root directory."
    exit 1
fi

print_status "Starting multi-platform Docker build for $IMAGE_NAME:$TAG"
print_status "Platforms: $PLATFORMS"
print_status "Push to registry: $PUSH"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker and try again."
    exit 1
fi

# Check if buildx is available
if ! docker buildx version > /dev/null 2>&1; then
    print_error "Docker Buildx is not available. Please install Docker Buildx."
    exit 1
fi

# Create or use existing builder
print_status "Setting up Docker Buildx builder: $BUILDX_BUILDER"
if docker buildx inspect "$BUILDX_BUILDER" > /dev/null 2>&1; then
    print_status "Using existing builder: $BUILDX_BUILDER"
    docker buildx use "$BUILDX_BUILDER"
else
    print_status "Creating new builder: $BUILDX_BUILDER"
    docker buildx create --name "$BUILDX_BUILDER" --use
fi

# Build the image
print_status "Building Docker image..."
BUILD_CMD="docker buildx build --platform $PLATFORMS -t $IMAGE_NAME:$TAG"

if [[ "$PUSH" == true ]]; then
    BUILD_CMD="$BUILD_CMD --push"
    print_status "Images will be pushed to registry after building"
else
    BUILD_CMD="$BUILD_CMD --load"
    print_warning "Images will be loaded locally (only first platform will be available)"
    print_warning "Use --push to push all platforms to registry"
fi

# Add cache options for better performance
BUILD_CMD="$BUILD_CMD --cache-from type=local,src=/tmp/.buildx-cache"
BUILD_CMD="$BUILD_CMD --cache-to type=local,dest=/tmp/.buildx-cache,mode=max"

# Execute the build command
print_status "Executing: $BUILD_CMD"
if eval "$BUILD_CMD"; then
    print_success "Multi-platform build completed successfully!"
    
    if [[ "$PUSH" == true ]]; then
        print_success "Images pushed to registry: $IMAGE_NAME:$TAG"
        print_status "Available platforms: $PLATFORMS"
    else
        print_success "Image loaded locally: $IMAGE_NAME:$TAG"
        print_warning "Only the first platform in the list is available locally"
    fi
else
    print_error "Build failed!"
    exit 1
fi

# Show image information
print_status "Image information:"
if [[ "$PUSH" == true ]]; then
    echo "  Registry: $IMAGE_NAME:$TAG"
    echo "  Platforms: $PLATFORMS"
else
    docker images "$IMAGE_NAME:$TAG" 2>/dev/null || print_warning "Image not found locally (use --push to push to registry)"
fi

print_success "Build process completed!" 