#!/bin/bash

# Release helper script for Konductor

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  auto-release    - Create an automatic release (increments patch version)"
    echo "  manual-release  - Create a manual release with specific version"
    echo "  check-status    - Check current version and last release"
    echo "  help           - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 auto-release"
    echo "  $0 manual-release v2.0.0"
    echo "  $0 check-status"
}

# Function to get current version
get_current_version() {
    local latest_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
    echo "$latest_tag"
}

# Function to calculate next version
calculate_next_version() {
    local current_version=$(get_current_version)
    local version_parts=(${current_version//./ })
    local major=${version_parts[0]#v}
    local minor=${version_parts[1]}
    local patch=${version_parts[2]}
    
    local new_patch=$((patch + 1))
    echo "v${major}.${minor}.${new_patch}"
}

# Function to create automatic release
auto_release() {
    print_status "Creating automatic release..."
    
    # Check if we're on main branch
    local current_branch=$(git branch --show-current)
    if [ "$current_branch" != "main" ]; then
        print_error "You must be on the main branch to create an automatic release"
        print_status "Current branch: $current_branch"
        exit 1
    fi
    
    # Check if there are uncommitted changes
    if [ -n "$(git status --porcelain)" ]; then
        print_error "You have uncommitted changes. Please commit or stash them first."
        git status --short
        exit 1
    fi
    
    local next_version=$(calculate_next_version)
    print_status "Next version will be: $next_version"
    
    # Create and push the tag
    print_status "Creating tag: $next_version"
    git tag "$next_version"
    git push origin "$next_version"
    
    print_success "Automatic release tag created: $next_version"
    print_status "The GitHub Actions workflow will now build and create the release."
}

# Function to create manual release
manual_release() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Please specify a version for manual release"
        echo "Example: $0 manual-release v2.0.0"
        exit 1
    fi
    
    # Validate version format
    if [[ ! "$version" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "Invalid version format. Use format: vX.Y.Z (e.g., v2.0.0)"
        exit 1
    fi
    
    print_status "Creating manual release: $version"
    
    # Check if tag already exists
    if git tag -l | grep -q "^$version$"; then
        print_error "Tag $version already exists"
        exit 1
    fi
    
    # Create and push the tag
    print_status "Creating tag: $version"
    git tag "$version"
    git push origin "$version"
    
    print_success "Manual release tag created: $version"
    print_status "The GitHub Actions workflow will now build and create the release."
}

# Function to check status
check_status() {
    print_status "Checking release status..."
    
    local current_version=$(get_current_version)
    local next_version=$(calculate_next_version)
    local current_branch=$(git branch --show-current)
    
    echo "Current version: $current_version"
    echo "Next auto-release version: $next_version"
    echo "Current branch: $current_branch"
    
    # Check for uncommitted changes
    if [ -n "$(git status --porcelain)" ]; then
        print_warning "You have uncommitted changes:"
        git status --short
    else
        print_success "Working directory is clean"
    fi
    
    # Check if we're on main
    if [ "$current_branch" = "main" ]; then
        print_success "You're on the main branch - ready for automatic release"
    else
        print_warning "You're not on the main branch - automatic releases only work from main"
    fi
}

# Main script logic
case "$1" in
    "auto-release")
        auto_release
        ;;
    "manual-release")
        manual_release "$2"
        ;;
    "check-status")
        check_status
        ;;
    "help"|"--help"|"-h"|"")
        show_usage
        ;;
    *)
        print_error "Unknown command: $1"
        show_usage
        exit 1
        ;;
esac
