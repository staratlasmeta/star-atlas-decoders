#!/bin/bash

# CI script for Star Atlas decoders
# Runs all decoder pipelines and ensures git repository remains clean

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to check if git repo is clean
check_git_clean() {
    if [ -n "$(git status --porcelain)" ]; then
        return 1
    fi
    return 0
}

# Main execution
main() {
    print_info "Starting CI pipeline for Star Atlas decoders"

    # Check initial git status
    if ! check_git_clean; then
        print_error "Git repository is not clean. Please commit or stash your changes first."
        git status --short
        exit 1
    fi

    print_info "Git repository is clean, proceeding with decoder generation..."

    # Run all decoder pipelines
    print_info "Building sage-starbased decoder..."
    if ! just all-sage-starbased; then
        print_error "Failed to build sage-starbased decoder"
        exit 1
    fi
    print_info "✅ sage-starbased decoder complete"

    print_info "Building sage-holosim decoder..."
    if ! just all-sage-holosim; then
        print_error "Failed to build sage-holosim decoder"
        exit 1
    fi
    print_info "✅ sage-holosim decoder complete"

    print_info "Building atlas-staking decoder..."
    if ! just all-atlas-staking; then
        print_error "Failed to build atlas-staking decoder"
        exit 1
    fi
    print_info "✅ atlas-staking decoder complete"

    print_info "Building locked-voter decoder..."
    if ! just all-locked-voter; then
        print_error "Failed to build locked-voter decoder"
        exit 1
    fi
    print_info "✅ locked-voter decoder complete"

    print_info "Building marketplace decoder..."
    if ! just all-marketplace; then
        print_error "Failed to build marketplace decoder"
        exit 1
    fi
    print_info "✅ marketplace decoder complete"

    # Verify git is still clean
    print_info "Checking final git status..."
    if ! check_git_clean; then
        print_error "Git repository is not clean after running pipelines!"
        print_error "The following files have been modified:"
        git status --short
        exit 1
    fi

    print_info "✅ All decoder pipelines completed successfully"
    print_info "✅ Git repository is clean"

    # Run cargo check to ensure everything compiles
    print_info "Running compilation check..."
    if ! cargo check --all; then
        print_error "Cargo check failed"
        exit 1
    fi

    # Run clippy to check code quality
    print_info "Running clippy checks..."
    if ! cargo clippy --all-targets --all-features -- -D warnings; then
        print_error "Clippy checks failed"
        exit 1
    fi
    print_info "✅ All clippy checks passed"

    print_info "🎉 CI pipeline completed successfully!"
}

# Run main function
main "$@"
