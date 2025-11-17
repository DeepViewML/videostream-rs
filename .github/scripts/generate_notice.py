#!/usr/bin/env python3
"""
Generate NOTICE file from SBOM (CycloneDX format)
Extracts packages requiring attribution based on their licenses
"""

import json
import sys
import os
from typing import Set, List, Dict, Optional

# Licenses that require attribution in NOTICE file
ATTRIBUTION_REQUIRED_LICENSES: Set[str] = {
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "BSD-4-Clause",
    "MIT",
}


def load_license_overrides() -> Dict[str, Dict]:
    """Load manual license overrides from configuration file."""
    overrides_path = os.path.join(
        os.path.dirname(os.path.dirname(__file__)),
        "license-overrides.json"
    )

    if not os.path.exists(overrides_path):
        return {}

    try:
        with open(overrides_path, 'r') as f:
            data = json.load(f)
            # Convert to dict: "name@version" -> full entry
            return {
                f"{entry['name']}@{entry['version']}": entry
                for entry in data.get('overrides', [])
            }
    except Exception as e:
        print(f"Warning: Could not load license overrides: {e}", file=sys.stderr)
        return {}


def get_manual_license(name: str, version: str, overrides: Dict[str, Dict]) -> Optional[str]:
    """Get manually verified license for a component."""
    key = f"{name}@{version}"
    entry = overrides.get(key)
    return entry['license'] if entry else None


def extract_license_from_component(component: Dict) -> Set[str]:
    """Extract all license identifiers from a component."""
    licenses: Set[str] = set()

    if "licenses" not in component:
        return licenses

    for lic_entry in component["licenses"]:
        # Check for direct license ID
        if "license" in lic_entry and "id" in lic_entry["license"]:
            licenses.add(lic_entry["license"]["id"])

        # Check for SPDX expression
        if "expression" in lic_entry:
            expr = lic_entry["expression"]
            # Parse expression (simplified - splits on OR/AND/WITH)
            parts = expr.replace("(", "").replace(")", "")
            parts = parts.replace(" OR ", " ").replace(" AND ", " ").replace(" WITH ", " ")
            for part in parts.split():
                if part and not part.isspace():
                    licenses.add(part)

    return licenses


def requires_attribution(licenses: Set[str]) -> bool:
    """Check if any of the licenses require attribution."""
    return bool(licenses.intersection(ATTRIBUTION_REQUIRED_LICENSES))


def generate_notice(sbom_path: str) -> str:
    """Generate NOTICE content from SBOM."""

    try:
        with open(sbom_path, 'r') as f:
            sbom = json.load(f)
    except Exception as e:
        print(f"Error reading SBOM: {e}", file=sys.stderr)
        sys.exit(1)

    components = sbom.get("components", [])
    overrides = load_license_overrides()

    # Extract components requiring attribution
    attribution_components: List[tuple[str, str, Set[str], bool]] = []

    for component in components:
        name = component.get("name", "unknown")
        version = component.get("version", "unknown")
        licenses = extract_license_from_component(component)

        # Check for manual override
        manual_license = get_manual_license(name, version, overrides)
        is_manual = False
        if manual_license and not licenses:
            licenses = {manual_license}
            is_manual = True

        if licenses and requires_attribution(licenses):
            attribution_components.append((name, version, licenses, is_manual))

    # Sort by name
    attribution_components.sort(key=lambda x: x[0].lower())

    # Generate NOTICE content
    notice = []
    notice.append("videostream-rs - VideoStream Library Rust Bindings")
    notice.append("Copyright 2025 Au-Zone Technologies")
    notice.append("")
    notice.append("This product includes software developed at Au-Zone Technologies")
    notice.append("(https://au-zone.com/).")
    notice.append("")
    notice.append("This software is part of the EdgeFirst Perception Middleware, an open source")
    notice.append("initiative for edge AI and computer vision. For more information, visit:")
    notice.append("https://edgefirst.ai")
    notice.append("")
    notice.append("=" * 80)
    notice.append("")
    notice.append("This software contains components from third-party open source projects that")
    notice.append("require attribution under their respective licenses (Apache-2.0, BSD, MIT, etc.).")
    notice.append("")

    if attribution_components:
        notice.append("Third-Party Components:")
        notice.append("")

        # Separate manually verified from auto-detected
        manual_components = [(n, v, l) for n, v, l, m in attribution_components if m]
        auto_components = [(n, v, l) for n, v, l, m in attribution_components if not m]

        for name, version, licenses in auto_components:
            license_str = ", ".join(sorted(licenses))
            notice.append(f"  * {name} {version} ({license_str})")

        if manual_components:
            notice.append("")
            notice.append("The following components have been manually verified:")
            notice.append("")
            for name, version, licenses in manual_components:
                license_str = ", ".join(sorted(licenses))
                notice.append(f"  * {name} {version} ({license_str}) [manually verified]")
    else:
        notice.append("  (No third-party components requiring attribution)")

    notice.append("")
    notice.append("For a complete Software Content Register (SBOM) including all dependencies,")
    notice.append("licenses, copyrights, and version information, see the sbom.json file included")
    notice.append("in the release artifacts or generated via GitHub Actions in this repository.")
    notice.append("")
    notice.append("The SBOM provides comprehensive information about:")
    notice.append("- All direct and transitive dependencies")
    notice.append("- License information for each component")
    notice.append("- Copyright notices and attributions")
    notice.append("- Source code license scanning results")
    notice.append("")
    notice.append("To generate the SBOM locally, run:")
    notice.append("  .github/scripts/generate_sbom.sh")
    notice.append("")
    notice.append("To view license policy compliance:")
    notice.append("  python3 .github/scripts/check_license_policy.py sbom.json")
    notice.append("")

    return "\n".join(notice)


def main():
    if len(sys.argv) != 2:
        print("Usage: generate_notice.py <sbom.json>", file=sys.stderr)
        sys.exit(1)

    sbom_path = sys.argv[1]
    notice_content = generate_notice(sbom_path)
    print(notice_content)


if __name__ == "__main__":
    main()
