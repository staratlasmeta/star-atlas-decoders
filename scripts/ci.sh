#!/bin/bash

# CI script for Star Atlas decoders
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

# Main execution
main() {
    print_info "Starting CI pipeline for Star Atlas decoders"

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
