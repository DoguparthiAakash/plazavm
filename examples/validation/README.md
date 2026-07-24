# Automated QA Certification Validation Example

This example demonstrates invoking PlazaVM's 16-Stage Evidence-Driven QA Certification Pipeline programmatically or via CLI.

---

## 💻 Running via CLI

```bash
cargo run -p plaza-cli -- validate
```

---

## 📊 Artifact Outputs Generated

- `REPORT.md` (Executive Markdown report)
- `REPORT.html` (Interactive dark-mode dashboard)
- `REPORT.json` (Structured telemetry)
- `logs/stage01.log` ... `stage16.log`
- `metrics/stage01_metrics.json` ... `stage16_metrics.json`
