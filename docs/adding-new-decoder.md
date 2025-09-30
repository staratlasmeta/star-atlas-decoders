# Adding a New Decoder to the Project

This document provides a step-by-step checklist for adding a new decoder to the star-atlas-decoders project.

## Overview

This guide covers adding decoders that fetch IDL from mainnet (like sage-starbased) or use local IDL files (like sage-holosim). The project uses a reusable justfile architecture that makes adding new decoders straightforward.

---

## Checklist

### Phase 1: Research & Preparation

- [ ] **Identify the Solana program**
  - [ ] Get the program ID (e.g., `ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc`)
  - [ ] Verify it's an Anchor program with available IDL
  - [ ] Check if source code is available for reference

- [ ] **Fetch and inspect the IDL**
  ```bash
  # Fetch from mainnet
  anchor idl fetch <PROGRAM_ID> --provider.cluster mainnet > /tmp/program.json

  # Check program name and version
  cat /tmp/program.json | jq -r '.name, .version'

  # List accounts and instructions
  cat /tmp/program.json | jq '.accounts[] | .name'
  cat /tmp/program.json | jq '.instructions[] | .name'
  ```

- [ ] **Check for existing implementations**
  - [ ] Search starcomm or other repos for existing decoders
  - [ ] Review for any custom patches or modifications needed
  - [ ] Document any complex account structures (e.g., remaining data fields)

- [ ] **Decide on decoder naming**
  - Use format: `<program-name>-decoder`
  - Examples: `atlas-staking-decoder`, `locked-voter-decoder`

---

### Phase 2: Justfile Configuration

- [ ] **Add program ID constant** (top of justfile)
  ```just
  ATLAS_STAKING_PROGRAM_ID := "ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc"
  ```

- [ ] **Add generation recipe** (choose one approach)

  **Option A: Mainnet IDL (like sage-starbased)**
  ```just
  # Generate atlas-staking decoder from mainnet
  generate-atlas-staking:
      @just _generate-from-mainnet atlas-staking {{ATLAS_STAKING_PROGRAM_ID}}
  ```

  **Option B: Local IDL (like sage-holosim)**
  ```just
  # Generate atlas-staking decoder from local IDL
  generate-atlas-staking:
      @just _generate-from-local atlas-staking {{ATLAS_STAKING_PROGRAM_ID}}
  ```

- [ ] **Add full workflow recipes** (copy and modify from existing decoder)
  ```just
  # Build atlas-staking decoder (clean + generate + prepare)
  build-atlas-staking:
      @just _build atlas-staking {{ATLAS_STAKING_PROGRAM_ID}} mainnet

  # Apply patches to atlas-staking decoder
  apply-patches-atlas-staking:
      @just _apply-patches atlas-staking

  # Create a new patch for atlas-staking
  create-patch-atlas-staking patch-name:
      @just _create-patch atlas-staking {{patch-name}}

  # Publish atlas-staking decoder to workspace
  publish-atlas-staking:
      @just _publish-decoder atlas-staking

  # Clean atlas-staking generated files
  clean-atlas-staking:
      @just _clean atlas-staking

  # Build, patch, and publish atlas-staking (full pipeline)
  all-atlas-staking:
      @just build-atlas-staking
      @just apply-patches-atlas-staking
      @just publish-atlas-staking
  ```

- [ ] **Add to clean-all recipe**
  ```just
  # Clean all generated files
  clean-all:
      just clean-sage-starbased
      just clean-sage-holosim
      just clean-atlas-staking  # Add this line
      just clean-locked-voter   # And this if adding multiple
  ```

---

### Phase 3: Optional - Save Local IDL

If you want to maintain a local copy of the IDL for reference or as a backup:

- [ ] **Save IDL file to ./idl/ directory**
  ```bash
  cp /tmp/program.json ./idl/<PROGRAM_ID>-idl.json
  ```

- [ ] **Commit the IDL file to git**
  ```bash
  git add ./idl/<PROGRAM_ID>-idl.json
  git commit -m "Add <program-name> IDL for reference"
  ```

---

### Phase 4: Generate Initial Decoder

- [ ] **Run the build command**
  ```bash
  just build-atlas-staking
  ```

- [ ] **Verify generation succeeded**
  - [ ] Check `./dist/atlas-staking/` directory was created
  - [ ] Verify expected files: `src/accounts/`, `src/instructions/`, `src/types/`, `Cargo.toml`
  - [ ] Check git was initialized in dist directory

- [ ] **Test compilation**
  ```bash
  cd ./dist/atlas-staking
  cargo check
  ```

---

### Phase 5: Identify and Create Patches (If Needed)

- [ ] **Compare with source program** (if available)
  - [ ] Check account structures for remaining data fields
  - [ ] Look for composite account types that need expansion
  - [ ] Identify any custom deserialize requirements

- [ ] **Compare with existing implementation** (if available)
  - [ ] Diff against starcomm or other implementations
  - [ ] Document any custom patches they used

- [ ] **Create patches as needed** (see `docs/patch-development-workflow.md`)
  ```bash
  # After making changes in ./dist/atlas-staking/
  just create-patch-atlas-staking 01-accounts
  just create-patch-atlas-staking 02-instructions-custom
  ```

- [ ] **Test patch application**
  ```bash
  just build-atlas-staking
  just apply-patches-atlas-staking
  cd ./dist/atlas-staking && cargo check
  ```

---

### Phase 6: Publish to Workspace

- [ ] **Publish the decoder**
  ```bash
  just publish-atlas-staking
  ```

- [ ] **Verify workspace compilation**
  ```bash
  cargo check -p atlas-staking-decoder
  ```

- [ ] **Add to workspace Cargo.toml** (if not auto-added)
  - Check `Cargo.toml` in project root
  - Ensure decoder is listed in `[workspace.members]`

---

### Phase 7: Update Documentation

- [ ] **Update CLAUDE.md**
  - [ ] Add decoder to "Project Overview" section
  - [ ] Add program ID to "Key Technical Details"
  - [ ] Update directory structure if needed

- [ ] **Update README** (if exists)
  - [ ] Add decoder to list of supported programs
  - [ ] Update build instructions if needed

- [ ] **Document any special considerations**
  - Note any complex patches or customizations
  - Document why certain patches were needed

---

### Phase 8: CI/CD Integration

- [ ] **Update CI script** (`./scripts/ci.sh`)
  ```bash
  # Add build commands for new decoder
  just all-atlas-staking
  just all-locked-voter  # if adding multiple
  ```

- [ ] **Test CI locally**
  ```bash
  ./scripts/ci.sh
  ```

- [ ] **Verify git clean state after CI**
  - CI should leave no uncommitted changes
  - All decoders should compile

---

### Phase 9: Testing & Validation

- [ ] **Run full pipeline test**
  ```bash
  just all-atlas-staking
  ```

- [ ] **Verify all commands work**
  - [ ] `just generate-atlas-staking`
  - [ ] `just build-atlas-staking`
  - [ ] `just apply-patches-atlas-staking`
  - [ ] `just publish-atlas-staking`
  - [ ] `just clean-atlas-staking`

- [ ] **Test patch creation workflow**
  - [ ] Make a change in dist directory
  - [ ] Create patch
  - [ ] Rebuild from clean
  - [ ] Verify patch applies correctly

---

### Phase 10: Commit & Document

- [ ] **Commit justfile changes**
  ```bash
  git add justfile
  git commit -m "Add atlas-staking-decoder recipes"
  ```

- [ ] **Commit patches** (if any)
  ```bash
  git add patches/atlas-staking-*.patch
  git commit -m "Add patches for atlas-staking-decoder"
  ```

- [ ] **Commit published decoder**
  ```bash
  git add carbon-decoders/atlas-staking-decoder/
  git commit -m "Add atlas-staking-decoder to workspace"
  ```

- [ ] **Commit documentation updates**
  ```bash
  git add CLAUDE.md docs/
  git commit -m "Update docs for atlas-staking-decoder"
  ```

- [ ] **Update CI if modified**
  ```bash
  git add scripts/ci.sh
  git commit -m "Add atlas-staking-decoder to CI"
  ```

---

## Example: Adding Two Decoders at Once

If adding multiple related decoders (e.g., atlas-staking and locked-voter):

1. Complete phases 1-2 for BOTH decoders before proceeding
2. Generate and test BOTH decoders in phase 4
3. Create patches for BOTH in phase 5
4. Publish BOTH in phase 6
5. Update documentation/CI for BOTH together
6. Commit all changes together in logical groups

This ensures consistency and reduces context switching.

---

## Common Issues

### Issue: "carbon-cli: command not found"
- Solution: Use `carbon-cli` instead of `carbon` in justfile
- The binary is installed as `carbon-cli`

### Issue: Patch fails to apply
- Check you committed previous patches before creating new ones
- See `docs/patch-development-workflow.md` for sequential patch creation

### Issue: Workspace references don't compile
- Ensure preparation step ran (fixes workspace references)
- Check `just _prepare-decoder` was called in build process

### Issue: Large byte arrays cause compilation errors
- Should be automatically fixed by preparation step
- Manually verify `[u8; 64]` converted to `Vec<u8>`

---

## Notes

- **Reusable recipes**: The project uses private `_*` recipes that do the actual work. Decoder-specific recipes just call these with the right parameters.

- **Git in dist/**: Each decoder's `dist/` directory has git initialized for patch tracking. Don't commit this git repo - it's only for creating patches.

- **Naming consistency**: Use kebab-case for decoder names: `atlas-staking-decoder`, not `atlas_staking_decoder`

- **Program IDs**: Always use the production/mainnet program ID, not devnet/testnet versions
