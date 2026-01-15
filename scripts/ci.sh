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

    # Check that all crates compile with the serde feature
    print_info "Checking all crates compile with serde feature..."
    if ! cargo check --workspace --features serde; then
        print_error "Compilation with serde feature failed"
        exit 1
    fi
    print_info "✅ All crates compile with serde feature"

    print_info "🎉 CI pipeline completed successfully!"
}

# Run main function
main "$@"
