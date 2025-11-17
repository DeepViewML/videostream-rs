# Security Policy

## Supported Versions

We actively support the following versions of videostream-rs with security updates:

| Version | Support Status          |
|---------|------------------------|
| 0.x     | ✅ Active development  |

**Note:** As this project is currently in pre-1.0 development, we provide security
fixes for the latest release on the main branch. Once we reach 1.0, we will
establish a more formal version support policy.

## Reporting a Vulnerability

Au-Zone Technologies takes security seriously across our entire EdgeFirst ecosystem,
including this VideoStream Library Rust binding.

### How to Report

**Primary Contact:**
- **Email:** support@au-zone.com
- **Subject Line:** "Security Vulnerability - videostream-rs"

**For EdgeFirst Studio Users:**
If you're using videostream-rs as part of EdgeFirst Studio or EdgeFirst Perception,
you may also report vulnerabilities through the EdgeFirst Studio security interface.

### Information to Include

Please provide as much information as possible to help us understand and address
the vulnerability:

- **Vulnerability Description:** Clear explanation of the security issue
- **Steps to Reproduce:** Detailed steps to demonstrate the vulnerability
- **Affected Versions:** Which versions of videostream-rs are impacted
- **Potential Impact:** Assessment of severity and potential consequences
- **Suggested Fixes:** If you have ideas for remediation (optional)
- **Proof of Concept:** Code, scripts, or configurations demonstrating the issue (optional)

### What to Expect

We follow a coordinated disclosure process:

1. **Acknowledgment:** Within 48 hours of your report
2. **Initial Assessment:** Within 7 business days, we will:
   - Confirm the vulnerability
   - Assess severity (using CVSS scoring)
   - Provide an initial timeline for resolution
3. **Fix Timeline:** Based on severity:
   - **Critical (CVSS 9.0-10.0):** 7 days
   - **High (CVSS 7.0-8.9):** 30 days
   - **Medium (CVSS 4.0-6.9):** Next minor release
   - **Low (CVSS 0.1-3.9):** Next major release or as time permits
4. **Coordinated Disclosure:** We will work with you to:
   - Develop and test the fix
   - Prepare security advisory
   - Coordinate public disclosure timing

### Responsible Disclosure

We kindly request that you:

- **Allow Reasonable Time:** Give us adequate time to investigate and fix the issue
- **Avoid Public Disclosure:** Do not publicly disclose the vulnerability until we've
  released a patch and published a security advisory
- **Do Not Exploit:** Do not use the vulnerability for malicious purposes or test
  on systems you don't own
- **Keep it Confidential:** Limit disclosure to those who need to know to help
  resolve the issue

### Recognition

With your permission, we will publicly credit you for responsibly disclosing the
vulnerability in:

- **Security Advisories:** GitHub Security Advisory for the vulnerability
- **Release Notes:** Changelog entry for the security fix
- **Annual Security Report:** Au-Zone Technologies security review (if applicable)
- **Hall of Fame:** Recognition on our security researchers page (coming soon)

If you prefer to remain anonymous, we will respect that choice.

## Security Update Process

When security vulnerabilities are fixed, we will:

1. **Create GitHub Security Advisory:** Using GitHub's private vulnerability reporting
2. **Release Patched Version:** Publish updated crate to crates.io
3. **Notify Users:** Through multiple channels:
   - GitHub Security Advisories (automatic for repository watchers)
   - EdgeFirst Studio notifications (for Studio users)
   - Release notes and CHANGELOG.md

## Security Best Practices for Users

When using videostream-rs in your applications:

### Dependency Management
- **Keep Updated:** Regularly update to the latest version to receive security fixes
- **Monitor Advisories:** Watch this repository for security advisories
- **Audit Dependencies:** Use `cargo audit` to check for known vulnerabilities:
  ```bash
  cargo install cargo-audit
  cargo audit
  ```

### Secure Configuration
- **Validate Inputs:** Always validate video frames and metadata from untrusted sources
- **Limit Permissions:** Run processes using videostream-rs with minimal required permissions
- **Secure Sockets:** Ensure UNIX Domain Socket paths are properly secured with filesystem permissions
- **Memory Safety:** While Rust provides memory safety guarantees, be cautious when using
  `unsafe` blocks or FFI boundaries

### Hardware Considerations
- **DMA Buffer Access:** Ensure proper access controls on `/dev/dma_heap` or similar devices
- **Shared Memory:** Validate that shared memory segments are properly isolated between processes
- **Hardware Acceleration:** When using hardware encoders/decoders, ensure firmware is up to date

## Known Security Considerations

### FFI Boundary
This crate provides Rust bindings to the native VideoStream Library (libvideostream).
Security vulnerabilities in the underlying C library could affect this Rust wrapper.
We work closely with the libvideostream maintainers to address any issues promptly.

### Privilege Requirements
Some features (camera access, DMA buffers) may require elevated privileges or specific
device permissions. Follow the principle of least privilege and run only necessary
components with elevated permissions.

### Process Isolation
The zero-copy frame sharing mechanism uses UNIX Domain Sockets and shared memory.
Ensure proper process isolation and access controls in multi-tenant environments.

## Additional Security Services

For production deployments requiring enhanced security:

- **Security Audits:** Professional code review and penetration testing
- **Compliance Certification:** Help meeting regulatory requirements (FedRAMP, HIPAA, etc.)
- **Priority Security Patches:** Expedited fixes for critical vulnerabilities
- **Custom Security Hardening:** Tailored security configurations for your deployment
- **Incident Response:** 24/7 support for security incidents

**Contact:** support@au-zone.com for enterprise security services.

## Security Resources

- **OWASP Top 10:** https://owasp.org/www-project-top-ten/
- **Rust Security Advisory Database:** https://rustsec.org/
- **CWE (Common Weakness Enumeration):** https://cwe.mitre.org/
- **CVE (Common Vulnerabilities and Exposures):** https://cve.mitre.org/

---

**Last Updated:** 2025-11-14
**Version:** 1.0
