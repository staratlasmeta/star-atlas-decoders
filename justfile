# Justfile for managing decoder generation and patching

# ============================================================================
# DECODER METADATA
# ============================================================================

# List of all decoders (space-separated)
ALL_DECODERS := "sage-starbased sage-holosim atlas-staking locked-voter marketplace atlas-fee-payer"

# Program IDs
ATLAS_FEE_PAYER_PROGRAM_ID := "APR1MEny25pKupwn72oVqMH4qpDouArsX8zX4VwwfoXD"
SAGE_STARBASED_PROGRAM_ID := "SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE"
SAGE_HOLOSIM_PROGRAM_ID := "SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9"
ATLAS_STAKING_PROGRAM_ID := "ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc"
LOCKED_VOTER_PROGRAM_ID := "Lock7kBijGCQLEFAmXcengzXKA88iDNQPriQ7TbgeyG"
MARKETPLACE_PROGRAM_ID := "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg"

# Descriptions
ATLAS_FEE_PAYER_DESC := "Rust decoder for Star Atlas ATLAS fee payer program on Solana"
SAGE_STARBASED_DESC := "Rust decoder for Star Atlas SAGE Starbased program on Solana"
SAGE_HOLOSIM_DESC := "Rust decoder for Star Atlas SAGE Holosim program on Solana"
ATLAS_STAKING_DESC := "Rust decoder for Star Atlas ATLAS staking program on Solana"
LOCKED_VOTER_DESC := "Rust decoder for Star Atlas Locked Voter governance program on Solana"
MARKETPLACE_DESC := "Rust decoder for Star Atlas Galactic Marketplace program on Solana"

# IDL Sources: "mainnet" or "local"
ATLAS_FEE_PAYER_SOURCE := "mainnet"
SAGE_STARBASED_SOURCE := "mainnet"
SAGE_HOLOSIM_SOURCE := "local"
ATLAS_STAKING_SOURCE := "mainnet"
LOCKED_VOTER_SOURCE := "mainnet"
MARKETPLACE_SOURCE := "mainnet"

# Carbon-cli generated names (what carbon-cli names the directory)
SAGE_STARBASED_GENERATED_NAME := "sage-decoder"
SAGE_HOLOSIM_GENERATED_NAME := "sage-decoder"
ATLAS_STAKING_GENERATED_NAME := "atlas-staking-decoder"
LOCKED_VOTER_GENERATED_NAME := "locked-voter-decoder"
MARKETPLACE_GENERATED_NAME := "marketplace-decoder"
ATLAS_FEE_PAYER_GENERATED_NAME := "atlas-fee-payer-decoder"

# ============================================================================
# OS DETECTION FOR CROSS-PLATFORM COMPATIBILITY
# ============================================================================

# Detect OS and set appropriate sed flags
OS := `uname -s`

# Create a portable sed in-place edit function
_sed pattern file:
    #!/bin/bash
    if [ "$(uname -s)" = "Darwin" ]; then
        sed -i '' '{{pattern}}' {{file}}
    else
        sed -i '{{pattern}}' {{file}}
    fi

# ============================================================================
# METADATA HELPER RECIPES
# ============================================================================

# Get program ID for a decoder
_get-program-id decoder_name:
    #!/bin/bash
    case "{{decoder_name}}" in
        sage-starbased) echo "{{SAGE_STARBASED_PROGRAM_ID}}" ;;
        sage-holosim) echo "{{SAGE_HOLOSIM_PROGRAM_ID}}" ;;
        atlas-staking) echo "{{ATLAS_STAKING_PROGRAM_ID}}" ;;
        locked-voter) echo "{{LOCKED_VOTER_PROGRAM_ID}}" ;;
        marketplace) echo "{{MARKETPLACE_PROGRAM_ID}}" ;;
        atlas-fee-payer) echo "{{ATLAS_FEE_PAYER_PROGRAM_ID}}" ;;
        *) echo "Unknown decoder: {{decoder_name}}" >&2; exit 1 ;;
    esac

# Get description for a decoder
_get-description decoder_name:
    #!/bin/bash
    case "{{decoder_name}}" in
        sage-starbased) echo "{{SAGE_STARBASED_DESC}}" ;;
        sage-holosim) echo "{{SAGE_HOLOSIM_DESC}}" ;;
        atlas-staking) echo "{{ATLAS_STAKING_DESC}}" ;;
        locked-voter) echo "{{LOCKED_VOTER_DESC}}" ;;
        marketplace) echo "{{MARKETPLACE_DESC}}" ;;
        atlas-fee-payer) echo "{{ATLAS_FEE_PAYER_DESC}}" ;;
        *) echo "Unknown decoder: {{decoder_name}}" >&2; exit 1 ;;
    esac

# Get IDL source type for a decoder
_get-source decoder_name:
    #!/bin/bash
    case "{{decoder_name}}" in
        sage-starbased) echo "{{SAGE_STARBASED_SOURCE}}" ;;
        sage-holosim) echo "{{SAGE_HOLOSIM_SOURCE}}" ;;
        atlas-staking) echo "{{ATLAS_STAKING_SOURCE}}" ;;
        locked-voter) echo "{{LOCKED_VOTER_SOURCE}}" ;;
        marketplace) echo "{{MARKETPLACE_SOURCE}}" ;;
        atlas-fee-payer) echo "{{ATLAS_FEE_PAYER_SOURCE}}" ;;
        *) echo "Unknown decoder: {{decoder_name}}" >&2; exit 1 ;;
    esac

# Get generated name from carbon-cli for a decoder
_get-generated-name decoder_name:
    #!/bin/bash
    case "{{decoder_name}}" in
        sage-starbased) echo "{{SAGE_STARBASED_GENERATED_NAME}}" ;;
        sage-holosim) echo "{{SAGE_HOLOSIM_GENERATED_NAME}}" ;;
        atlas-staking) echo "{{ATLAS_STAKING_GENERATED_NAME}}" ;;
        locked-voter) echo "{{LOCKED_VOTER_GENERATED_NAME}}" ;;
        marketplace) echo "{{MARKETPLACE_GENERATED_NAME}}" ;;
        atlas-fee-payer) echo "{{ATLAS_FEE_PAYER_GENERATED_NAME}}" ;;
        *) echo "Unknown decoder: {{decoder_name}}" >&2; exit 1 ;;
    esac

# ============================================================================
# GENERIC REUSABLE RECIPES (Private, prefixed with _)
# ============================================================================

# Fix workspace references in generated Cargo.toml
_fix-workspace-refs decoder_name:
    @echo "Fixing workspace references for {{decoder_name}}..."
    just _sed 's/edition = { workspace = true }/edition = "2024"/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/carbon-core = { workspace = true }/carbon-core = "0.10.0"/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/carbon-proc-macros = { workspace = true }/carbon-proc-macros = "0.10.0"/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/carbon-macros = { workspace = true }/carbon-macros = "0.10.0"/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/solana-account = { workspace = true }/solana-account = "2.2.1"/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/solana-instruction = { workspace = true }/solana-instruction = { version = "2.3.0", default-features = false }/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/solana-pubkey = { workspace = true }/solana-pubkey = { version = "2.4.0", features = ["borsh", "serde", "bytemuck"] }/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/serde = { workspace = true }/serde = { version = "1.0", features = ["derive"] }/' ./dist/{{decoder_name}}/Cargo.toml
    just _sed 's/serde-big-array = { workspace = true }/serde-big-array = "0.5.1"/' ./dist/{{decoder_name}}/Cargo.toml
    @echo "✅ Workspace references fixed"

# Rename package with carbon- prefix
_rename-package decoder_name old_name new_name:
    @echo "Renaming package from {{old_name}} to {{new_name}}..."
    just _sed 's/name = "{{old_name}}"/name = "{{new_name}}"/' ./dist/{{decoder_name}}/Cargo.toml
    @echo "✅ Package renamed"

# Add crates.io metadata to Cargo.toml
_add-crate-metadata decoder_name description program_id:
    #!/bin/bash
    echo "Adding crates.io metadata for {{decoder_name}}..."
    CARGO_TOML="./dist/{{decoder_name}}/Cargo.toml"

    # Create temp file with metadata
    TEMP_FILE=$(mktemp)

    # Read the file and insert metadata after edition line
    awk '
    /^edition = / {
        print
        print "description = \"{{description}}\""
        print "license = \"Apache-2.0\""
        print "repository = \"https://github.com/staratlasmeta/star-atlas-decoders\""
        print "homepage = \"https://github.com/staratlasmeta/star-atlas-decoders\""
        print "readme = \"README.md\""
        print "keywords = [\"solana\", \"star-atlas\", \"decoder\"]"
        print "categories = [\"encoding\"]"
        print "rust-version = \"1.85\""
        next
    }
    { print }
    ' "$CARGO_TOML" > "$TEMP_FILE"

    mv "$TEMP_FILE" "$CARGO_TOML"
    echo "✅ Metadata added"

# Prepare generated code by fixing compilation issues
_prepare-decoder decoder_name:
    #!/bin/bash
    echo "Preparing {{decoder_name}} decoder..."
    echo "Fixing large byte arrays in types and instructions modules..."
    # Replace [u8; 64] with Vec<u8> in types and instructions
    if [ "$(uname -s)" = "Darwin" ]; then
        find ./dist/{{decoder_name}}/src/types -name "*.rs" -type f -exec sed -i '' 's/\[u8; 64\]/Vec<u8>/g' {} \;
        find ./dist/{{decoder_name}}/src/instructions -name "*.rs" -type f -exec sed -i '' 's/\[u8; 64\]/Vec<u8>/g' {} \;
        # Also fix Option<[u8; 64]> to Option<Vec<u8>> in types and instructions
        find ./dist/{{decoder_name}}/src/types -name "*.rs" -type f -exec sed -i '' 's/Option<\[u8; 64\]>/Option<Vec<u8>>/g' {} \;
        find ./dist/{{decoder_name}}/src/instructions -name "*.rs" -type f -exec sed -i '' 's/Option<\[u8; 64\]>/Option<Vec<u8>>/g' {} \;
        # Remove serde-big-array attributes since we're using Vec<u8> now
        find ./dist/{{decoder_name}}/src/types -name "*.rs" -type f -exec sed -i '' '/#\[serde(with = "serde_big_array::BigArray")\]/d' {} \;
        find ./dist/{{decoder_name}}/src/instructions -name "*.rs" -type f -exec sed -i '' '/#\[serde(with = "serde_big_array::BigArray")\]/d' {} \;
    else
        find ./dist/{{decoder_name}}/src/types -name "*.rs" -type f -exec sed -i 's/\[u8; 64\]/Vec<u8>/g' {} \;
        find ./dist/{{decoder_name}}/src/instructions -name "*.rs" -type f -exec sed -i 's/\[u8; 64\]/Vec<u8>/g' {} \;
        # Also fix Option<[u8; 64]> to Option<Vec<u8>> in types and instructions
        find ./dist/{{decoder_name}}/src/types -name "*.rs" -type f -exec sed -i 's/Option<\[u8; 64\]>/Option<Vec<u8>>/g' {} \;
        find ./dist/{{decoder_name}}/src/instructions -name "*.rs" -type f -exec sed -i 's/Option<\[u8; 64\]>/Option<Vec<u8>>/g' {} \;
        # Remove serde-big-array attributes since we're using Vec<u8> now
        find ./dist/{{decoder_name}}/src/types -name "*.rs" -type f -exec sed -i '/#\[serde(with = "serde_big_array::BigArray")\]/d' {} \;
        find ./dist/{{decoder_name}}/src/instructions -name "*.rs" -type f -exec sed -i '/#\[serde(with = "serde_big_array::BigArray")\]/d' {} \;
    fi
    echo "✅ Large byte arrays converted to Vec<u8>"
    echo "✅ Serde attributes removed"
    echo "Running cargo fmt..."
    cd ./dist/{{decoder_name}} && cargo fmt
    echo "✅ Decoder prepared"

# Apply patches for a decoder
_apply-patches decoder_name:
    #!/bin/bash
    echo "Applying patches to {{decoder_name}} decoder..."
    cd ./dist/{{decoder_name}}
    # Apply all patches using git apply
    for patch in ../../patches/{{decoder_name}}-*.patch; do
        if [ -f "$patch" ]; then
            echo "Applying $(basename $patch)..."
            git apply -p1 "$patch" && echo "✅ Applied"
        fi
    done
    echo "✅ All patches applied"

# Publish decoder to carbon-decoders workspace
_publish-decoder decoder_name:
    @echo "Publishing {{decoder_name}} decoder..."
    @echo "Checking if decoder compiles first..."
    cd ./dist/{{decoder_name}} && cargo fmt && cargo check && cargo clean
    @echo "✅ Decoder compiles"
    @echo "Moving decoder to carbon-decoders..."
    rm -rf ./carbon-decoders/{{decoder_name}}-decoder
    rm -rf ./dist/{{decoder_name}}/.git
    rm -f ./dist/{{decoder_name}}/.gitignore
    rm -f ./dist/{{decoder_name}}/Cargo.lock
    mv ./dist/{{decoder_name}} ./carbon-decoders/{{decoder_name}}-decoder
    @echo "Copying README.md..."
    cp ./docs/readmes/{{decoder_name}}-README.md ./carbon-decoders/{{decoder_name}}-decoder/README.md
    @echo "✅ Decoder published to ./carbon-decoders/{{decoder_name}}-decoder"
    @echo "Verifying in workspace..."
    cargo check -p carbon-{{decoder_name}}-decoder
    @echo "✅ Decoder works in main workspace"

# Clean generated decoder files
_clean decoder_name:
    @echo "Cleaning {{decoder_name}} decoder..."
    rm -rf ./dist/{{decoder_name}}
    @echo "✅ Cleaned"

# Initialize git repo for patch management
_init-git decoder_name:
    @echo "target/" > ./dist/{{decoder_name}}/.gitignore
    @echo "Cargo.lock" >> ./dist/{{decoder_name}}/.gitignore
    @echo ".DS_Store" >> ./dist/{{decoder_name}}/.gitignore
    cd ./dist/{{decoder_name}} && git init -q && git add . && git commit -q -m "Initial generated state"

# Create a patch from current changes
_create-patch decoder_name patch_name:
    @echo "Creating patch from changes in {{decoder_name}}..."
    cd ./dist/{{decoder_name}} && git diff > ../../patches/{{decoder_name}}-{{patch_name}}.patch
    @echo "✅ Patch saved to patches/{{decoder_name}}-{{patch_name}}.patch"
    @echo "Patch size: $(wc -l < patches/{{decoder_name}}-{{patch_name}}.patch) lines"

# Universal decoder generation (handles both mainnet and local IDL sources)
_generate-decoder decoder_name:
    #!/bin/bash
    set -euo pipefail

    PROGRAM_ID=$(just _get-program-id {{decoder_name}})
    SOURCE=$(just _get-source {{decoder_name}})
    GENERATED_NAME=$(just _get-generated-name {{decoder_name}})

    if [ "$SOURCE" = "mainnet" ]; then
        echo "Fetching IDL from mainnet for $PROGRAM_ID..."
        carbon-cli parse --idl "$PROGRAM_ID" -u mainnet-beta --output ./dist --as-crate --standard anchor
    elif [ "$SOURCE" = "local" ]; then
        echo "Generating decoder from local IDL for $PROGRAM_ID..."
        carbon-cli parse --idl ./idl/${PROGRAM_ID}-idl.json --output ./dist --as-crate --standard anchor
    else
        echo "Error: Unknown source type '$SOURCE'" >&2
        exit 1
    fi

    echo "✅ Decoder generated from $SOURCE IDL"
    echo "Renaming to {{decoder_name}}..."
    mv ./dist/$GENERATED_NAME ./dist/{{decoder_name}}
    echo "✅ Renamed to {{decoder_name}}"

    # Rename package to include carbon- prefix
    just _rename-package {{decoder_name}} $GENERATED_NAME carbon-{{decoder_name}}-decoder

    # Fix workspace references
    just _fix-workspace-refs {{decoder_name}}

# ============================================================================
# UNIVERSAL DECODER PIPELINE COMMANDS
# ============================================================================

# Generate a decoder from its IDL source
generate decoder: (_generate-decoder decoder)

# Build a decoder (clean + generate + prepare + add metadata)
build decoder: (_clean decoder) (_generate-decoder decoder) (_prepare-decoder decoder)
    #!/bin/bash
    DESCRIPTION=$(just _get-description {{decoder}})
    PROGRAM_ID=$(just _get-program-id {{decoder}})
    just _add-crate-metadata {{decoder}} "$DESCRIPTION" "$PROGRAM_ID"
    echo "✅ {{decoder}} decoder generated and prepared"
    just _init-git {{decoder}}

# Clean a decoder
clean decoder: (_clean decoder)

# Apply patches to a decoder
apply-patches decoder: (_apply-patches decoder)

# Create a patch for a decoder
# Usage: just create-patch <decoder-name> <patch-name>
create-patch decoder patch_name: (_create-patch decoder patch_name)

# Publish a decoder to the workspace
publish decoder: (_publish-decoder decoder)

# Full pipeline for a decoder (build + patch + publish)
all decoder: (build decoder) (_apply-patches decoder) (_publish-decoder decoder)
    @echo "✅ {{decoder}} built, patched, and published"

# ============================================================================
# CONVENIENCE ALIASES (Keep existing command names)
# ============================================================================

# SAGE Starbased
generate-sage-starbased: (generate "sage-starbased")
build-sage-starbased: (build "sage-starbased")
clean-sage-starbased: (clean "sage-starbased")
apply-patches-sage-starbased: (apply-patches "sage-starbased")
create-patch-sage-starbased patch_name: (create-patch "sage-starbased" patch_name)
publish-sage-starbased: (publish "sage-starbased")
all-sage-starbased: (all "sage-starbased")

# SAGE Holosim
generate-sage-holosim: (generate "sage-holosim")
build-sage-holosim: (build "sage-holosim")
clean-sage-holosim: (clean "sage-holosim")
apply-patches-sage-holosim: (apply-patches "sage-holosim")
create-patch-sage-holosim patch_name: (create-patch "sage-holosim" patch_name)
publish-sage-holosim: (publish "sage-holosim")
all-sage-holosim: (all "sage-holosim")

# ATLAS Staking
generate-atlas-staking: (generate "atlas-staking")
build-atlas-staking: (build "atlas-staking")
clean-atlas-staking: (clean "atlas-staking")
apply-patches-atlas-staking: (apply-patches "atlas-staking")
create-patch-atlas-staking patch_name: (create-patch "atlas-staking" patch_name)
publish-atlas-staking: (publish "atlas-staking")
all-atlas-staking: (all "atlas-staking")

# Locked Voter
generate-locked-voter: (generate "locked-voter")
build-locked-voter: (build "locked-voter")
clean-locked-voter: (clean "locked-voter")
apply-patches-locked-voter: (apply-patches "locked-voter")
create-patch-locked-voter patch_name: (create-patch "locked-voter" patch_name)
publish-locked-voter: (publish "locked-voter")
all-locked-voter: (all "locked-voter")

# Marketplace
generate-marketplace: (generate "marketplace")
build-marketplace: (build "marketplace")
clean-marketplace: (clean "marketplace")
apply-patches-marketplace: (apply-patches "marketplace")
create-patch-marketplace patch_name: (create-patch "marketplace" patch_name)
publish-marketplace: (publish "marketplace")
all-marketplace: (all "marketplace")

# ATLAS Fee Payer
generate-atlas-fee-payer: (generate "atlas-fee-payer")
build-atlas-fee-payer: (build "atlas-fee-payer")
clean-atlas-fee-payer: (clean "atlas-fee-payer")
apply-patches-atlas-fee-payer: (apply-patches "atlas-fee-payer")
create-patch-atlas-fee-payer patch_name: (create-patch "atlas-fee-payer" patch_name)
publish-atlas-fee-payer: (publish "atlas-fee-payer")
all-atlas-fee-payer: (all "atlas-fee-payer")

# ============================================================================
# UTILITY COMMANDS
# ============================================================================

# Clean all generated decoders
clean-all:
    #!/bin/bash
    for decoder in {{ALL_DECODERS}}; do
        echo "Cleaning $decoder..."
        just clean $decoder
    done
    echo "✅ All decoders cleaned"

# Build all decoders
build-all:
    #!/bin/bash
    for decoder in {{ALL_DECODERS}}; do
        echo "Building $decoder..."
        just build $decoder
    done
    echo "✅ All decoders built"

# Run full pipeline for all decoders
all-all:
    #!/bin/bash
    for decoder in {{ALL_DECODERS}}; do
        echo "Running full pipeline for $decoder..."
        just all $decoder
    done
    echo "✅ All decoders completed"

# List all available decoders
list-decoders:
    #!/bin/bash
    echo "Available decoders:"
    for decoder in {{ALL_DECODERS}}; do
        echo "  - $decoder"
    done

# List available patches
list-patches:
    @echo "Available patches:"
    @ls -la patches/*.patch 2>/dev/null || echo "No patches found"

# Show OS detection info (for debugging)
show-os-info:
    @echo "Detected OS: {{OS}}"
    @echo "Using sed command: $(if [ '{{OS}}' = 'Darwin' ]; then echo 'sed -i \"\"'; else echo 'sed -i'; fi)"
