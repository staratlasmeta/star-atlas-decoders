#!/bin/bash

# Check for required tools for Star Atlas decoder development

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_TOOLS=0
AVAILABLE_COUNT=0
MISSING_COUNT=0

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Star Atlas Decoder Tools Check${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check for Rust
echo "Checking for Rust..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
if command -v cargo >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Rust is available. Version: $(rustc --version)${NC}"
    AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
else
    echo -e "${RED}❌ Rust is not available${NC}"
    echo "  Install from: https://rustup.rs/"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check for cargo
echo "Checking for cargo..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
if command -v cargo >/dev/null 2>&1; then
    echo -e "${GREEN}✅ cargo is available. Version: $(cargo --version)${NC}"
    AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
else
    echo -e "${RED}❌ cargo is not available${NC}"
    echo "  Install Rust from: https://rustup.rs/"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check for carbon-cli
echo "Checking for carbon-cli..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
CARBON_MIN_VERSION="0.12.0"
if command -v carbon-cli >/dev/null 2>&1; then
    CARBON_VERSION=$(carbon-cli --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -n1)
    if [ -n "$CARBON_VERSION" ]; then
        if [ "$(printf '%s\n' "$CARBON_MIN_VERSION" "$CARBON_VERSION" | sort -V | head -n1)" = "$CARBON_MIN_VERSION" ]; then
            echo -e "${GREEN}✅ carbon-cli is available. Version: $CARBON_VERSION${NC}"
            AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
        else
            echo -e "${RED}❌ carbon-cli version $CARBON_VERSION is below minimum required $CARBON_MIN_VERSION${NC}"
            echo "  Update with: cargo install --git https://github.com/sevenlabs-hq/carbon.git carbon-cli"
            MISSING_COUNT=$((MISSING_COUNT + 1))
        fi
    else
        echo -e "${YELLOW}⚠️  carbon-cli is available but version could not be determined${NC}"
        AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
    fi
else
    echo -e "${RED}❌ carbon-cli is not available${NC}"
    echo "  Install with: cargo install --git https://github.com/sevenlabs-hq/carbon.git carbon-cli"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check for just
echo "Checking for just..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
if command -v just >/dev/null 2>&1; then
    echo -e "${GREEN}✅ just is available. Version: $(just --version)${NC}"
    AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
else
    echo -e "${RED}❌ just is not available${NC}"
    echo "  Install with: cargo install just"
    echo "  Or via homebrew: brew install just"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check for git
echo "Checking for git..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
if command -v git >/dev/null 2>&1; then
    echo -e "${GREEN}✅ git is available. Version: $(git --version)${NC}"
    AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
else
    echo -e "${RED}❌ git is not available${NC}"
    echo "  Install via your package manager"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check for solana CLI (optional but useful)
echo "Checking for solana CLI (optional)..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
if command -v solana >/dev/null 2>&1; then
    echo -e "${GREEN}✅ solana CLI is available. Version: $(solana --version)${NC}"
    AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
else
    echo -e "${YELLOW}⚠️  solana CLI is not available (optional)${NC}"
    echo "  Install from: https://docs.solana.com/cli/install-solana-cli-tools"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check for sed
echo "Checking for sed..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
if command -v sed >/dev/null 2>&1; then
    echo -e "${GREEN}✅ sed is available${NC}"
    AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
else
    echo -e "${RED}❌ sed is not available${NC}"
    echo "  Usually comes with your system"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check for find
echo "Checking for find..."
TOTAL_TOOLS=$((TOTAL_TOOLS + 1))
if command -v find >/dev/null 2>&1; then
    echo -e "${GREEN}✅ find is available${NC}"
    AVAILABLE_COUNT=$((AVAILABLE_COUNT + 1))
else
    echo -e "${RED}❌ find is not available${NC}"
    echo "  Usually comes with your system"
    MISSING_COUNT=$((MISSING_COUNT + 1))
fi
echo ""

# Check Rust edition
echo "Checking Rust edition support..."
if command -v rustc >/dev/null 2>&1; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    # Check if version is >= 1.85 (required for edition 2024)
    if [ "$(printf '%s\n' "1.85.0" "$RUST_VERSION" | sort -V | head -n1)" = "1.85.0" ]; then
        echo -e "${GREEN}✅ Rust version supports edition 2024${NC}"
    else
        echo -e "${YELLOW}⚠️  Rust version may not support edition 2024${NC}"
        echo "  Update with: rustup update"
    fi
else
    echo -e "${YELLOW}⚠️  Cannot check Rust edition (Rust not installed)${NC}"
fi
echo ""

# Summary
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo "Total tools checked: $TOTAL_TOOLS"
echo -e "${GREEN}Available: $AVAILABLE_COUNT${NC}"
if [ $MISSING_COUNT -eq 0 ]; then
    echo -e "${GREEN}Missing: $MISSING_COUNT${NC}"
    echo ""
    echo -e "${GREEN}🎉 All required tools are installed!${NC}"
else
    echo -e "${RED}Missing: $MISSING_COUNT${NC}"
    echo ""
    echo -e "${YELLOW}⚠️  Some tools are missing. Please install them to use all features.${NC}"
fi

# Exit with error if critical tools are missing
CRITICAL_MISSING=0
for tool in cargo carbon-cli git just sed find; do
    if ! command -v $tool >/dev/null 2>&1; then
        CRITICAL_MISSING=1
        break
    fi
done

if [ $CRITICAL_MISSING -eq 1 ]; then
    echo ""
    echo -e "${RED}❌ Critical tools are missing. Cannot proceed with decoder generation.${NC}"
    exit 1
fi

exit 0
