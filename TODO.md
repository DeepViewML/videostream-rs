# TODO: Open Source Migration Plan for videostream-rs

**Repository:** videostream-rs
**Purpose:** Rust bindings for VideoStream Library - Zero-copy video frame management and distribution
**Target License:** Apache-2.0
**Part of:** EdgeFirst Perception Middleware (open source initiative)
**Context:** Part of original DeepViewRT package, used by edgefirst-camera service in production deployments
**Repository Location:** https://github.com/DeepViewML/videostream-rs (DeepViewML organization)
**Future Plan:** VideoStream Library may be open-sourced with Rust wrapper integrated into core repository
**Current Status:** ~60% complete (documentation, licensing, SBOM infrastructure done)
**Last Updated:** 2025-11-17

---

## 📊 Quick Status Overview

| Category | Status | Progress |
|----------|--------|----------|
| **Documentation** | ✅ Complete | 100% (4/4 files) |
| **Legal & Licensing** | ✅ Complete | 100% (12/12 items) |
| **SBOM Infrastructure** | ✅ Complete | 100% (7/7 items) |
| **Code Quality** | 🔄 In Progress | 60% |
| **Testing** | ⏸️ Not Started | 0% |
| **CI/CD** | 🔄 In Progress | 75% |
| **Examples** | ⏸️ Not Started | 0% |

**Overall Progress:** ~65% complete

---

## Table of Contents

1. [Completed Work](#completed-work)
2. [Documentation Requirements](#documentation-requirements)
3. [Legal and Licensing](#legal-and-licensing)
4. [SBOM Infrastructure](#sbom-infrastructure)
5. [Code Quality and Refactoring](#code-quality-and-refactoring)
6. [Testing Infrastructure](#testing-infrastructure)
7. [CI/CD and Automation](#cicd-and-automation)
8. [Examples and Tutorials](#examples-and-tutorials)
9. [Final Review and Quality Checks](#final-review-and-quality-checks)
10. [Questions for Team Discussion](#questions-for-team-discussion)
11. [Session Handoff Notes](#session-handoff-notes)

---

## ✅ Completed Work

### Session 1 (2025-11-14 to 2025-11-17)

**Documentation (100% Complete):**
- ✅ README.md - Complete user-focused rewrite (12KB)
- ✅ CONTRIBUTING.md - Developer contribution guidelines (9.8KB)
- ✅ SECURITY.md - Vulnerability reporting and security policy (6.4KB)
- ✅ NOTICE - Apache-2.0 compliance with attribution (auto-generated)
- ✅ TODO.md - Comprehensive migration plan (this file)
- ✅ CODE_OF_CONDUCT.md - Contributor Covenant (already present)
- ✅ AGENTS.md - AI assistant guidelines (already present)
- ❌ ARCHITECTURE.md - Removed (not needed for bindings-only project)

**Legal & Licensing (100% Complete):**
- ✅ LICENSE file verified (Apache-2.0 with correct copyright)
- ✅ All 11 source files have Apache-2.0 SPDX headers
- ✅ Both Cargo.toml files updated to `license = "Apache-2.0"`
- ✅ Cargo.toml metadata updated (keywords, homepage, description)
- ✅ All DeepView references replaced with EdgeFirst
- ✅ Module documentation updated (src/lib.rs)

**SBOM Infrastructure (100% Complete):**
- ✅ Python venv created with scancode-toolkit installed
- ✅ cargo-cyclonedx installed for Rust dependency SBOM
- ✅ CycloneDX CLI installed (~/.local/bin/cyclonedx)
- ✅ Three SBOM scripts created in .github/scripts/:
  - check_license_policy.py (7.3KB)
  - generate_notice.py (6.7KB)
  - generate_sbom.sh (3.1KB)
- ✅ License override system implemented (.github/license-overrides.json)
- ✅ SBOM documentation created (.github/README.md)
- ✅ .gitignore updated for SBOM artifacts
- ✅ CI workflow updated with SBOM compliance job
- ✅ SBOM tested locally (19 components, all licenses compliant)
- ✅ Manual license verification for dma-buf 0.4.0 → MIT

**CI/CD Updates (60% Complete):**
- ✅ Added SBOM compliance job to GitHub Actions
- ✅ SBOM artifacts uploaded on every push/PR
- ⏸️ Test coverage reporting not yet implemented
- ⏸️ Clippy job not yet added
- ⏸️ Security audit job not yet added

---

## 📋 Remaining Work

### 1. Documentation Requirements

#### README.md
- [x] **COMPLETED** - User-focused documentation complete
  - ✅ Installation and quick start
  - ✅ Code examples (Host, Client, Camera, Encoder)
  - ✅ Architecture diagram
  - ✅ Platform support and performance data
  - ✅ EdgeFirst integration information
  - ✅ Contributing and support sections

#### CONTRIBUTING.md
- [x] **COMPLETED** - Developer guidelines complete
  - ✅ Development setup instructions
  - ✅ Contribution workflow (fork, branch, PR)
  - ✅ Code style and testing requirements
  - ✅ License information

#### SECURITY.md
- [x] **COMPLETED** - Security policy complete
  - ✅ Vulnerability reporting process
  - ✅ Response timelines by severity
  - ✅ Coordinated disclosure guidelines

#### NOTICE
- [x] **COMPLETED** - Auto-generated from SBOM
  - ✅ Lists all attribution-required dependencies
  - ✅ Marks manually verified licenses
  - ✅ References complete SBOM

---

## 🔐 Legal and Licensing

### License Compliance
- [x] **COMPLETED** - All licensing work done
  - ✅ Apache-2.0 license verified
  - ✅ All SPDX headers added
  - ✅ Cargo.toml metadata updated
  - ✅ No remaining DeepView references in code

### Outstanding License Issues
- ✅ **RESOLVED** - dma-buf 0.4.0 manually verified as MIT
  - Documented in .github/license-overrides.json
  - Clear audit trail with repository source

---

## 📦 SBOM Infrastructure

### SBOM Generation System
- [x] **COMPLETED** - Full SBOM infrastructure implemented
  - ✅ Merged SBOM approach (cargo-cyclonedx + scancode)
  - ✅ License policy checking with Apache-2.0 compatibility
  - ✅ NOTICE file generation with attributions
  - ✅ Manual license override system
  - ✅ CI/CD integration
  - ✅ ~16 second generation time

### License Override System
- [x] **COMPLETED** - Manual verification system
  - ✅ Configuration file: .github/license-overrides.json
  - ✅ Full audit trail (date, method, source, notes)
  - ✅ Integration with policy checker and NOTICE generator
  - ✅ Clear reporting of manual verifications
  - ✅ Documentation in .github/README.md

### Current SBOM Status
- Total components: 19 (16 dependencies + 2 source + 1 project)
- License compliance: ✅ PASSING (all Apache-2.0 compatible)
- Manual verifications: 1 (dma-buf 0.4.0)
- Runtime: ~16 seconds

---

## 🔧 Code Quality and Refactoring

**Status:** 60% complete

### Completed Tasks
- [x] **Run Clippy and fix all warnings** - All 14 Clippy errors resolved:
  - Fixed needless_range_loop in frame.rs (used iterator with enumerate)
  - Fixed result_unit_err (added allow attributes for FFI functions)
  - Fixed mut_from_ref in frame.rs:mmap_mut (added safety documentation and allow)
  - Fixed unnecessary_cast in client.rs (removed redundant cast)
  - Fixed suspicious_open_options in frame.rs (added .truncate(true))
  - Fixed manual_ok in decoder.rs (used .ok() method)
  - Fixed io_other_error in decoder.rs (used io::Error::other)
  - Fixed wrong_self_convention in fourcc.rs (renamed from_u32 to to_u32)
  - Fixed mismatched_lifetime_syntaxes in camera.rs (added explicit lifetimes)
  - Fixed not_unsafe_ptr_arg_deref in encoder.rs (marked function as unsafe)
  - Removed unused c_void import from client.rs
  - Fixed const trait implementation issues in fourcc.rs (removed nightly-only features)
  - Fixed deprecated rand API (thread_rng → rng, gen → random)
- [x] **Add Clippy job to CI workflow** - Job added with -D warnings flag

### Remaining Tasks
- [ ] Verify rustfmt configuration is correct
- [ ] Run rustfmt on all source files:
  ```bash
  cargo +nightly-2023-07-01 fmt
  ```
- [ ] Audit all `unsafe` blocks - ensure safety comments present
- [ ] Review error handling - ensure proper Error implementations
- [ ] Search for TODO/FIXME comments - address or track in issues

### Code Quality Checklist
- [x] No Clippy warnings (with -D warnings)
- [ ] All code formatted with rustfmt
- [ ] All unsafe blocks documented with safety rationale
- [ ] Error messages are clear and actionable
- [ ] Public API fully documented with rustdoc
- [ ] Module-level documentation complete

### Refactoring for Testability (Future)
- [ ] Review client.rs for testability
- [ ] Review host.rs for testability
- [ ] Review frame.rs for testability
- [ ] Consider trait abstractions for FFI operations
- [ ] Enable dependency injection for testing

**Priority:** Medium (can be done incrementally)

---

## 🧪 Testing Infrastructure

**Status:** 0% complete (not started)

### Unit Tests
- [ ] Audit existing tests (if any)
- [ ] Identify untested functionality
- [ ] Implement unit tests for each module:
  - [ ] client.rs - Client initialization, subscription, reconnection
  - [ ] host.rs - Host initialization, publishing, client management
  - [ ] frame.rs - Frame lifecycle, memory management
  - [ ] encoder.rs - Encoding operations
  - [ ] decoder.rs - Decoding operations
  - [ ] camera.rs - Camera capture
  - [ ] fourcc.rs - FourCC conversions

### Mock Infrastructure
- [ ] Design mocking strategy for hardware dependencies
- [ ] Create mock traits for FFI operations
- [ ] Implement test fixtures for common scenarios
- [ ] Document mocking approach

### Test Coverage
- [ ] Install cargo-llvm-cov:
  ```bash
  cargo install cargo-llvm-cov
  ```
- [ ] Configure coverage exclusions (if needed)
- [ ] Set coverage target: 70%+ for core modules
- [ ] Generate coverage reports locally:
  ```bash
  cargo llvm-cov --html
  ```

### Integration Tests
- [ ] Create integration test suite
- [ ] Test Host-Client communication
- [ ] Test Encoder-Decoder pipelines
- [ ] Test camera capture integration
- [ ] Document hardware requirements for integration tests

**Priority:** High (needed before 1.0 release)
**Blocker:** Requires libvideostream, may need mocking

---

## ⚙️ CI/CD and Automation

**Status:** 75% complete

### GitHub Actions Workflow (.github/workflows/rust.yml)

#### Completed
- ✅ Format check job (uses nightly-2023-07-01)
- ✅ Clippy job (runs on all targets with -D warnings)
- ✅ Test job (installs libvideostream from DeepView APT)
- ✅ SBOM compliance job (generates SBOM, checks licenses, uploads artifacts)
- ✅ Deploy job (publishes to crates.io on tags)

#### Remaining Work

- [ ] Add test coverage job:
  ```yaml
  coverage:
    name: Test Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install dependencies
        run: # ... install libvideostream
      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov
      - name: Generate coverage
        run: cargo llvm-cov --all-features --lcov --output-path lcov.info
      - name: Upload to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: lcov.info
  ```

- [ ] Add security audit job:
  ```yaml
  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
  ```

- [ ] Configure Codecov (codecov.yml)
- [ ] Set up badge URLs in README.md once CI runs
- [ ] Test full CI pipeline on a branch

### Repository Settings (Post-Migration)
- [ ] Enable GitHub Discussions
- [ ] Enable GitHub Security Advisories
- [ ] Configure branch protection for main:
  - Require PR reviews
  - Require status checks (CI passing)
  - Require linear history
- [ ] Add repository topics: rust, video, zero-copy, edge-ai, computer-vision, embedded
- [ ] Create issue templates (bug report, feature request)
- [ ] Create PR template

**Priority:** Medium (needed for production use)

---

## 📚 Examples and Tutorials

**Status:** 0% complete (not started)

### Example Applications
- [ ] Create `examples/` directory
- [ ] Write example: `basic_publisher.rs`
  - Simple frame publisher using Host
  - Shows socket creation and frame publishing
- [ ] Write example: `basic_subscriber.rs`
  - Simple frame subscriber using Client
  - Shows connection and frame reception
- [ ] Write example: `camera_pipeline.rs`
  - Camera capture with V4L2
  - Publishing to subscribers
- [ ] Write example: `encoding_pipeline.rs`
  - Capture → Encode → Write to file
  - Shows hardware encoder usage
- [ ] Write example: `multi_subscriber.rs`
  - One publisher, multiple subscribers
  - Demonstrates multi-process architecture

### Documentation Examples
- [ ] Ensure all public APIs have rustdoc examples
- [ ] Test that examples compile:
  ```bash
  cargo test --doc
  ```

**Priority:** Medium (helpful for users)
**Blocker:** Requires libvideostream to run

---

## ✅ Final Review and Quality Checks

**Status:** Not started

### Pre-Release Checklist
- [ ] All CI jobs passing
- [ ] Test coverage ≥ 70%
- [ ] No Clippy warnings
- [ ] All examples compile and run
- [ ] Documentation complete and accurate
- [ ] SBOM generation successful
- [ ] License compliance verified
- [ ] CHANGELOG.md created (if not exists)
- [ ] Version number set (replace 0.0.0)

### Documentation Review
- [ ] README.md - accurate and up-to-date
- [ ] CONTRIBUTING.md - clear and actionable
- [ ] SECURITY.md - contact info correct
- [ ] API documentation - complete and helpful
- [ ] Examples - working and well-commented

### Legal Review
- [ ] All SPDX headers present
- [ ] LICENSE file correct
- [ ] NOTICE file accurate
- [ ] No license violations in dependencies
- [ ] Copyright year correct (2025)

### Release Preparation
- [ ] Tag version in git: `git tag v0.1.0`
- [ ] Update Cargo.toml version numbers
- [ ] Test crates.io publish (dry-run):
  ```bash
  cargo publish --dry-run
  ```
- [ ] Create GitHub Release with:
  - Release notes
  - SBOM (sbom.json)
  - NOTICE file
  - Source tarball

**Priority:** High (before first release)

---

## ❓ Questions for Team Discussion

### Resolved
1. ~~**License Strategy**~~ - ✅ Apache-2.0 confirmed
2. ~~**Repository Organization**~~ - ✅ DeepViewML (for now)
3. ~~**ARCHITECTURE.md needed?**~~ - ✅ No, removed

### Outstanding Questions

#### 1. External Dependency Access
**Question:** How should open source contributors access libvideostream?
- Currently installed from deepviewml.com APT repo
- Is this publicly accessible?
- Should we provide:
  - Mock library for testing without hardware?
  - Source-based build instructions?
  - Container image with libvideostream pre-installed?

**Impact:** Blocks external contributions if not resolved

#### 2. Integration Testing Strategy
**Question:** How to run hardware-dependent tests in CI?
- Tests require libvideostream runtime
- Tests may require actual hardware (cameras, encoders)

**Options:**
- Self-hosted runners with Maivin/Raivin hardware
- Mark hardware tests as `#[ignore]`, run manually
- Mock all hardware dependencies
- Hybrid: unit tests with mocks, integration tests manual

**Impact:** Affects test coverage and quality

#### 3. Documentation Hosting
**Question:** Timeline for EdgeFirst Perception documentation?
- README references https://doc.edgefirst.ai/test/perception/
- Is this URL final or placeholder?
- When will it be publicly accessible?

**Impact:** Broken links in README if not updated

#### 4. Version Numbering
**Question:** What should the first public version be?
- Currently: 0.0.0 (placeholder)
- Options: 0.1.0, 1.0.0?
- Is there existing version history to consider?

**Impact:** Release planning

#### 5. Maintenance and Ownership
**Question:** Who owns/maintains this project long-term?
- Will it migrate to VideoStream core repository?
- If so, when?
- Should we document this transition plan?

**Impact:** Contributor expectations

---

## 🔄 Session Handoff Notes

### For Next Session

**Immediate Priorities:**
1. Run Clippy and fix warnings (quick win)
2. Add Clippy job to CI
3. Decide on integration testing strategy
4. Start implementing unit tests
5. Add test coverage reporting

**What's Ready:**
- All documentation is complete and ready for use
- All licensing is compliant and verified
- SBOM infrastructure is production-ready
- CI/CD has basic jobs, needs coverage and security audit
- README has good examples, but actual example files not created yet

**What Needs Attention:**
- Testing infrastructure (0% complete - highest priority)
- Code quality checks (Clippy, rustfmt)
- Example applications
- Resolve questions about external dependency access

**Files to Review:**
- `.github/workflows/rust.yml` - CI workflow, needs expansion
- `Cargo.toml` - Version still 0.0.0, needs setting
- `src/**/*.rs` - Need unit tests added
- `.github/license-overrides.json` - May need updates as dependencies change

**Tools Installed Locally:**
- Python venv with scancode-toolkit (venv/)
- cargo-cyclonedx (global)
- cyclonedx CLI (~/.local/bin/cyclonedx)

**Useful Commands:**
```bash
# Generate SBOM
.github/scripts/generate_sbom.sh

# Check license policy
python3 .github/scripts/check_license_policy.py sbom.json

# Run Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo +nightly-2023-07-01 fmt

# Run tests
cargo test --verbose

# Generate coverage (after installing cargo-llvm-cov)
cargo llvm-cov --html
```

**Git Status:**
- Modified: 17 files (source files, configs, docs)
- New: 6 files (docs, SBOM scripts, overrides)
- All changes uncommitted (ready for review and commit)

**Next Steps Recommendation:**
1. Commit current work (documentation + SBOM infrastructure)
2. Address Clippy warnings
3. Add Clippy + coverage jobs to CI
4. Start unit test implementation
5. Create example applications

---

## 📈 Progress Tracking

### Milestone 1: Documentation & Licensing ✅ COMPLETE
- [x] README.md
- [x] CONTRIBUTING.md
- [x] SECURITY.md
- [x] LICENSE verified
- [x] SPDX headers added
- [x] Cargo.toml metadata updated

### Milestone 2: SBOM Infrastructure ✅ COMPLETE
- [x] SBOM generation scripts
- [x] License compliance checking
- [x] Manual verification system
- [x] CI integration
- [x] Documentation

### Milestone 3: Code Quality 🔄 IN PROGRESS (60%)
- [x] Clippy warnings fixed (all 14 errors resolved)
- [x] Clippy CI job added
- [ ] Code formatted
- [x] Source docs updated
- [ ] Unsafe code audited

### Milestone 4: Testing Infrastructure ⏸️ NOT STARTED (0%)
- [ ] Unit tests implemented
- [ ] Test coverage ≥ 70%
- [ ] Integration tests
- [ ] Mocking strategy

### Milestone 5: CI/CD Completion 🔄 IN PROGRESS (75%)
- [x] SBOM job
- [x] Clippy job
- [ ] Coverage job
- [ ] Security audit job

### Milestone 6: Examples & Polish ⏸️ NOT STARTED (0%)
- [ ] Example applications
- [ ] Final documentation review
- [ ] Release preparation

**Target Completion:** When all 6 milestones are ✅

---

## 📝 Change Log

**2025-11-17 (Session 2):**
- ✅ Fixed all 14 Clippy warnings and errors
  - Resolved const trait implementation issues in fourcc.rs
  - Fixed deprecated rand API (thread_rng → rng, gen → random)
  - Added safety documentation for mmap_mut function
  - Fixed lifetime inconsistencies in camera.rs
  - Marked unsafe encoder function appropriately
  - Removed unused imports
- ✅ Added Clippy job to CI workflow with -D warnings
- ✅ Updated TODO.md with code quality progress (60% complete)

**2025-11-17 (Session 1):**
- ✅ Completed SBOM infrastructure with license override system
- ✅ Updated .gitignore for SBOM artifacts
- ✅ Added SBOM compliance job to CI
- ✅ Verified dma-buf 0.4.0 as MIT license
- ✅ Updated TODO.md for session handoff

**2025-11-14:**
- ✅ Completed all documentation (README, CONTRIBUTING, SECURITY, NOTICE)
- ✅ Added Apache-2.0 SPDX headers to all 11 source files
- ✅ Updated Cargo.toml metadata (license, keywords, description)
- ✅ Removed ARCHITECTURE.md (not needed for bindings)
- ✅ Updated src/lib.rs module documentation

---

**Document Version:** 2.1
**Last Updated:** 2025-11-17
**Status:** Ready for next work session - Code quality improvements complete, testing infrastructure is next priority
**Maintained by:** Au-Zone Technologies Open Source Migration Team
