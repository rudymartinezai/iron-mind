# 🧩 Versioning and Release Guide

This document defines the versioning and release workflow for **Iron Mind**.

---

## Semantic Versioning (SemVer)

Iron Mind follows [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH
```

### Definitions

- **MAJOR**: Incompatible API changes or major rewrites (e.g., `1.0.0` → `2.0.0`).
- **MINOR**: Backward-compatible feature additions (e.g., `0.1.0` → `0.2.0`).
- **PATCH**: Bug fixes, refactors, or documentation updates (e.g., `0.1.0` → `0.1.1`).

---

## Current Version

| Component | Version | Description |
|------------|----------|--------------|
| Sandbox Firewall | `0.1.0` | Local prototype, config-based packet filtering |
| Production Build | `0.2.0` *(planned)* | Blockchain-ready scaffolding |

---

## Release Workflow

1. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat(sandbox): improved packet filtering logic"
   ```

2. **Tag the Release**
   ```bash
   git tag -a v0.1.0 -m "Initial sandbox firewall prototype"
   git push origin main --tags
   ```

3. **Update Version Numbers**
   Edit `Cargo.toml` for the crate you’re releasing:
   ```toml
   version = "0.1.0"
   ```

4. **Document the Release**
   Add a section to `CHANGELOG.md` (create if missing):
   ```markdown
   ## [0.1.0] – 2025-10-25
   - Implemented initial Rust firewall prototype
   - Added config-based deny patterns
   - Introduced packet simulation and CLI startup
   ```

---

## Branching Strategy

| Branch | Purpose |
|:--------|:---------|
| `main` | Stable branch for working releases |
| `dev` | Active development branch for new features |
| `prod` | Blockchain integration and production-ready code |

Feature branches follow this pattern:
```
feature/<description>
```
Example:
```
feature/blockchain-logger
```

---

## Best Practices

- Keep commits atomic: one logical change per commit.
- Write clear commit messages using [Conventional Commits](https://www.conventionalcommits.org/).
- Always bump version numbers *after* merging new features.
- Tag everything—tags are your insurance policy against chaos.

---

## Example Release Sequence

```bash
git checkout main
git merge dev
cargo check && cargo test
git commit -m "chore: prepare for release v0.1.0"
git tag -a v0.1.0 -m "Initial prototype release"
git push origin main --tags
```

---

## Future Plans

Once production reaches `v0.2.0`, releases will include:
- Automated CI checks (GitHub Actions)
- Binary builds for macOS & Linux
- Release notes auto-generated from commits

---
