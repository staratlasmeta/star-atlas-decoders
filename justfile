# Justfile for managing decoder generation and patching

# Program IDs
SAGE_STARBASED_PROGRAM_ID := "SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE"
SAGE_HOLOSIM_PROGRAM_ID := "SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9"

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
    @echo "✅ Decoder published to ./carbon-decoders/{{decoder_name}}-decoder"
    @echo "Verifying in workspace..."
    cargo check -p {{decoder_name}}-decoder
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

# ============================================================================
# SAGE-STARBASED DECODER COMMANDS
# ============================================================================

# Generate sage-starbased decoder from mainnet IDL
generate-sage-starbased:
    #!/bin/bash
    echo "Fetching IDL from mainnet for {{SAGE_STARBASED_PROGRAM_ID}}..."
    carbon-cli parse --idl {{SAGE_STARBASED_PROGRAM_ID}} -u mainnet-beta --output ./dist --as-crate --standard anchor
    echo "✅ Decoder generated from mainnet IDL"
    echo "Renaming to sage-starbased..."
    mv ./dist/sage-decoder ./dist/sage-starbased
    if [ "$(uname -s)" = "Darwin" ]; then
        sed -i '' 's/name = "sage-decoder"/name = "sage-starbased-decoder"/' ./dist/sage-starbased/Cargo.toml
    else
        sed -i 's/name = "sage-decoder"/name = "sage-starbased-decoder"/' ./dist/sage-starbased/Cargo.toml
    fi
    echo "✅ Renamed to sage-starbased"
    just _fix-workspace-refs sage-starbased

# Build sage-starbased decoder
build-sage-starbased: (_clean "sage-starbased") generate-sage-starbased (_prepare-decoder "sage-starbased")
    @echo "✅ sage-starbased decoder generated and prepared"
    just _init-git sage-starbased

# Clean sage-starbased decoder
clean-sage-starbased: (_clean "sage-starbased")

# Apply patches for sage-starbased
apply-patches-sage-starbased: (_apply-patches "sage-starbased")

# Create patch for sage-starbased
# Usage: just create-patch-sage-starbased <patch-name>
create-patch-sage-starbased patch_name: (_create-patch "sage-starbased" patch_name)

# Publish sage-starbased decoder
publish-sage-starbased: (_publish-decoder "sage-starbased")

# Full pipeline for sage-starbased
all-sage-starbased: build-sage-starbased (_apply-patches "sage-starbased") (_publish-decoder "sage-starbased")
    @echo "✅ sage-starbased built, patched, and published"

# ============================================================================
# SAGE-HOLOSIM DECODER COMMANDS
# ============================================================================

# Generate sage-holosim decoder from local IDL
generate-sage-holosim:
    #!/bin/bash
    echo "Generating decoder from local IDL for {{SAGE_HOLOSIM_PROGRAM_ID}}..."
    carbon-cli parse --idl ./idl/{{SAGE_HOLOSIM_PROGRAM_ID}}-idl.json --output ./dist --as-crate --standard anchor
    echo "✅ Decoder generated from local IDL"
    echo "Renaming to sage-holosim..."
    mv ./dist/sage-decoder ./dist/sage-holosim
    if [ "$(uname -s)" = "Darwin" ]; then
        sed -i '' 's/name = "sage-decoder"/name = "sage-holosim-decoder"/' ./dist/sage-holosim/Cargo.toml
    else
        sed -i 's/name = "sage-decoder"/name = "sage-holosim-decoder"/' ./dist/sage-holosim/Cargo.toml
    fi
    echo "✅ Renamed to sage-holosim"
    just _fix-workspace-refs sage-holosim

# Build sage-holosim decoder
build-sage-holosim: (_clean "sage-holosim") generate-sage-holosim (_prepare-decoder "sage-holosim")
    @echo "✅ sage-holosim decoder generated and prepared"
    just _init-git sage-holosim

# Clean sage-holosim decoder
clean-sage-holosim: (_clean "sage-holosim")

# Apply patches for sage-holosim
apply-patches-sage-holosim: (_apply-patches "sage-holosim")

# Create patch for sage-holosim
# Usage: just create-patch-sage-holosim <patch-name>
create-patch-sage-holosim patch_name: (_create-patch "sage-holosim" patch_name)

# Publish sage-holosim decoder
publish-sage-holosim: (_publish-decoder "sage-holosim")

# Full pipeline for sage-holosim
all-sage-holosim: build-sage-holosim (_apply-patches "sage-holosim") (_publish-decoder "sage-holosim")
    @echo "✅ sage-holosim built, patched, and published"

# ============================================================================
# UTILITY COMMANDS
# ============================================================================

# Clean all generated decoders
clean-all: clean-sage-starbased clean-sage-holosim
    @echo "✅ All decoders cleaned"

# List available patches
list-patches:
    @echo "Available patches:"
    @ls -la patches/*.patch 2>/dev/null || echo "No patches found"

# Show OS detection info (for debugging)
show-os-info:
    @echo "Detected OS: {{OS}}"
    @echo "Using sed command: $(if [ '{{OS}}' = 'Darwin' ]; then echo 'sed -i \"\"'; else echo 'sed -i'; fi)"