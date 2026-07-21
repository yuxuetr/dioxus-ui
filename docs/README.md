# Documentation

This directory defines the architecture and execution plan for `dioxus-ui`.

Read in this order:

1. [Design Overview](design.md)
2. [Roadmap](roadmap.md)
3. [Workspace Specification](workspace.md)
4. [Component API Specification](component-api.md)
5. [Release and Package Strategy](release.md)
6. [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)
7. [Release Gate Failure Triage Runbook](release-gate-failure-triage-runbook.md)
8. [Quality Gates](quality-gates.md)
9. [Component Catalog](components/README.md)
10. [Documentation Site Plan](site.md)
11. [CI Browser Smoke Guide](ci-browser-smoke.md)
12. [CI Browser Workflow Template](ci-browser-workflow-template.md)
13. [CI Browser Workflow Metadata](ci-browser-workflow-metadata.md)
14. [Browser Artifact Policy Metadata](browser-artifact-policy-metadata.md)
15. [Release Warning Inventory Metadata](release-warning-inventory-metadata.md)
16. [Cargo Publish Metadata](cargo-publish-metadata.md)
17. [Publish Readiness Blockers](publish-readiness-blockers.md)
18. [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
19. [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)
20. [Changelog Metadata](changelog-metadata.md)
21. [Release Notes Readiness Metadata](release-notes-readiness-metadata.md)
22. [License Readiness Metadata](license-readiness-metadata.md)
23. [Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md)
24. [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
25. [Public API Surface Inventory](public-api-surface-inventory.md)
26. [CLI Template Packaging Readiness Metadata](cli-template-packaging-readiness-metadata.md)
27. [Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md)
28. [Publish Readiness Coverage Metadata](publish-readiness-coverage-metadata.md)
29. [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
30. [Publish Order Metadata](publish-order-metadata.md)
31. [Workspace Dependency Publish Readiness Metadata](workspace-dependency-publish-readiness-metadata.md)
32. [Runtime Adapter Plan](components/runtime-adapters.md)
33. [Runtime Renderer Verification Matrix](components/runtime-renderer-verification.md)
34. [Web Runtime Verification Harness](components/runtime-web-verification-harness.md)
35. [Desktop and Mobile Runtime Verification Strategy](components/runtime-desktop-mobile-verification.md)
36. [Runtime Implementation Milestone Seeds](components/runtime-implementation-milestones.md)
37. [Web Runtime Adapter Module Boundaries](components/runtime-web-adapter-boundaries.md)
38. [Rendered Component Verification](components/rendered-component-verification.md)
39. [Browser DOM Component Verification](components/browser-dom-component-verification.md)
40. [Runtime Interaction Verification](components/runtime-interaction-verification.md)
41. [Web Preview Screenshot Smoke](components/web-preview-screenshot-smoke.md)
42. [Browser Smoke Aggregate](components/browser-smoke-aggregate.md)
43. [Release Screenshot Review Checklist](components/release-screenshot-review-checklist.md)
44. [Screenshot Artifact Retention](components/screenshot-artifact-retention.md)
45. [Release Screenshot Review Notes Template](components/release-screenshot-review-notes-template.md)
46. [Release Candidate Browser Review Runbook](components/release-candidate-browser-review-runbook.md)
47. [RFC 0001: Project Architecture](rfcs/0001-project-architecture.md)
48. [RFC 0002: CLI Registry and Code Generation](rfcs/0002-cli-registry-and-code-generation.md)
49. [RFC 0003: Tailwind Styling Contract](rfcs/0003-tailwind-styling-contract.md)
50. [RFC 0004: Benchmark and CSS Output Strategy](rfcs/0004-benchmark-and-css-output.md)
51. [RFC 0005: Modules and Platform Profiles](rfcs/0005-modules-and-platform-profiles.md)
52. [RFC 0006: Focus and Portal Primitives](rfcs/0006-focus-and-portal-primitives.md)
53. [RFC 0007: Keyboard Navigation Primitives](rfcs/0007-keyboard-navigation-primitives.md)
54. [RFC 0008: Overlay Positioning and Portals](rfcs/0008-overlay-positioning-and-portals.md)
55. [RFC 0009: CI Browser Workflow Activation](rfcs/0009-ci-browser-workflow-activation.md)
56. [TODO Plan](../TODOs.md)

## Project Principles

- Documentation before implementation.
- Source-copy workflow before packaged crate workflow.
- Headless primitives before complex styled components.
- Complete Tailwind class tokens in source files.
- Accessibility requirements are part of component behavior, not optional polish.
- Rendered preview targets should be deterministic metadata before browser
  screenshot claims.
