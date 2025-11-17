# SBOM and License Management

This directory contains configuration and scripts for SBOM (Software Bill of Materials) generation and license compliance checking.

## Files

### `license-overrides.json`

Manual license verifications for packages with missing or incorrect license metadata in their package manifests.

**Purpose:**
- Document manually verified licenses for packages that lack proper metadata
- Provide audit trail for license verification process
- Ensure accurate license reporting in SBOM and NOTICE files

**Format:**
```json
{
  "overrides": [
    {
      "name": "package-name",
      "version": "1.2.3",
      "license": "MIT",
      "verification": {
        "date": "2025-11-14",
        "method": "Repository inspection",
        "source": "https://github.com/owner/repo",
        "notes": "Additional context about the verification"
      }
    }
  ]
}
```

**When to add an entry:**

1. Package shows "no license information" in SBOM but repository clearly shows license
2. Package has incorrect license in Cargo.toml metadata
3. Package license needs manual verification due to dual-licensing or complex licensing

**How the overrides are used:**

- `check_license_policy.py`: Uses overrides to fill in missing license data before policy check
- `generate_notice.py`: Includes manually verified components with `[manually verified]` marker
- Both scripts display clearly which components were manually verified

**Example output:**
```
MANUALLY VERIFIED LICENSES:
--------------------------------------------------------------------------------
  ✓ dma-buf 0.4.0 - Manually verified: MIT
```

**Maintenance:**
- Review overrides when upgrading dependencies (newer versions may have fixed metadata)
- Always include verification source and date for audit trail
- Document reasoning in "notes" field

### `scripts/` Directory

Contains SBOM generation and license checking scripts:

- **`generate_sbom.sh`**: Main script to generate complete SBOM
- **`check_license_policy.py`**: License policy compliance checker
- **`generate_notice.py`**: NOTICE file generator with attribution

See [../CONTRIBUTING.md](../CONTRIBUTING.md) for usage instructions.

## SBOM Generation Process

The SBOM generation follows a merged approach:

1. **cargo-cyclonedx**: Generate dependency SBOM from Cargo.lock (~1 second)
2. **scancode-toolkit**: Scan source code for licenses/copyrights (~15 seconds)
3. **cyclonedx-cli**: Merge both SBOMs into complete SBOM
4. **check_license_policy.py**: Validate license compliance (using overrides)
5. **generate_notice.py**: Create NOTICE file with attributions (using overrides)

**Total runtime:** ~16 seconds

## License Policy

This project uses **Apache-2.0** license and requires all dependencies to use compatible licenses.

**Allowed licenses:**
- MIT, Apache-2.0, BSD variants, ISC, Unlicense, etc.
- See `scripts/check_license_policy.py` for complete list

**Disallowed licenses:**
- GPL, AGPL (any version)
- Non-commercial licenses (CC-BY-NC)
- SSPL

**Review required:**
- MPL-2.0 (Mozilla Public License)
- LGPL variants

## Troubleshooting

### New dependency shows "no license information"

1. Check the dependency's repository for license file
2. If clearly licensed, add entry to `license-overrides.json`
3. Re-run SBOM generation: `.github/scripts/generate_sbom.sh`

### License policy check fails

1. Review output to identify problematic dependency
2. Options:
   - Find alternative dependency with compatible license
   - If legitimately compatible, add to ALLOWED_LICENSES in `check_license_policy.py`
   - If needs manual review, add to REVIEW_REQUIRED_LICENSES

### Updating dependencies

After `cargo update`:
1. Re-run SBOM generation
2. Check if any overrides can be removed (if metadata fixed in new version)
3. Add new overrides if new dependencies lack license metadata

## CI/CD Integration

The GitHub Actions workflow automatically:
- Generates SBOM on every push/PR
- Checks license policy compliance
- Uploads SBOM artifacts (sbom.json, NOTICE)
- Fails build on license violations

See `.github/workflows/rust.yml` for details.
