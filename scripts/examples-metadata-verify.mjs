#!/usr/bin/env node
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));

const readRepoFile = (relativePath) => {
  return readFileSync(join(repoRoot, relativePath), "utf8");
};

const getSection = (toml, sectionName) => {
  const sectionHeader = `[${sectionName}]`;
  const lines = toml.split("\n");
  const startIndex = lines.findIndex((line) => line.trim() === sectionHeader);
  if (startIndex < 0) {
    return null;
  }

  const sectionLines = [];
  for (const line of lines.slice(startIndex + 1)) {
    if (/^\s*\[/.test(line)) {
      break;
    }
    sectionLines.push(line);
  }
  return sectionLines.join("\n");
};

const getStringField = (section, field) => {
  const match = section.match(new RegExp(`^\\s*${field}\\s*=\\s*"([^"]+)"\\s*$`, "m"));
  return match?.[1] ?? null;
};

const parseWorkspaceMembers = (rootCargo) => {
  const membersMatch = rootCargo.match(/^\s*members\s*=\s*\[([\s\S]*?)^\s*\]/m);
  if (membersMatch === null) {
    return null;
  }

  return membersMatch[1]
    .split("\n")
    .map((line) => line.trim().replace(/,$/, ""))
    .filter((line) => line.startsWith("\"") && line.endsWith("\""))
    .map((line) => line.slice(1, -1));
};

const rootCargo = readRepoFile("Cargo.toml");
const examplesReadme = readRepoFile("examples/README.md");
const packageJson = JSON.parse(readRepoFile("package.json"));
const exampleSmoke = readRepoFile("scripts/example-smoke.sh");
const webPreviewVerify = readRepoFile("scripts/web-preview-verify.mjs");
const desktopPreviewVerify = readRepoFile("scripts/desktop-preview-verify.mjs");
const mobileWebProfileVerify = readRepoFile("scripts/mobile-web-profile-verify.mjs");
const workspaceMembers = parseWorkspaceMembers(rootCargo);
const failures = [];

if (workspaceMembers === null) {
  failures.push("Cargo.toml is missing a workspace members array");
}

const exampleManifestPaths = readdirSync(join(repoRoot, "examples"), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => `examples/${entry.name}/Cargo.toml`)
  .filter((path) => existsSync(join(repoRoot, path)))
  .sort();

const examplePackages = exampleManifestPaths.map((manifestPath) => {
  const manifest = readRepoFile(manifestPath);
  const packageSection = getSection(manifest, "package");
  const name = packageSection === null ? null : getStringField(packageSection, "name");
  return {
    manifestPath,
    name,
    packageDir: dirname(manifestPath),
  };
});

for (const examplePackage of examplePackages) {
  if (examplePackage.name === null) {
    failures.push(`${examplePackage.manifestPath} is missing package.name`);
  }

  if (workspaceMembers !== null && !workspaceMembers.includes(examplePackage.packageDir)) {
    failures.push(`${examplePackage.packageDir} is missing from Cargo workspace members`);
  }
}

if (workspaceMembers !== null) {
  for (const member of workspaceMembers.filter((member) => member.startsWith("examples/"))) {
    if (!existsSync(join(repoRoot, member, "Cargo.toml"))) {
      failures.push(`workspace example member is missing Cargo.toml: ${member}`);
    }
  }
}

const requiredReadmeSnippets = [
  "## Web Demo",
  "cargo run -p dioxus-ui-web-demo",
  "dx serve --package dioxus-ui-web-demo --bin preview",
  "node scripts/web-preview-verify.mjs",
  "## Desktop Demo",
  "cargo run -p dioxus-ui-desktop-demo",
  "dx serve --package dioxus-ui-desktop-demo --bin preview --platform desktop",
  "node scripts/desktop-preview-verify.mjs",
  "## Runtime Web Verification",
  "cargo run -p dioxus-ui-runtime-web-verification",
  "cargo test -p dioxus-ui-runtime-web-verification",
  "node scripts/runtime-web-verify.mjs",
  "## Runtime Desktop Verification",
  "cargo run -p dioxus-ui-runtime-desktop-verification",
  "cargo test -p dioxus-ui-runtime-desktop-verification",
  "## CLI Init Smoke",
  "cargo run -p dioxus-ui-cli -- init --root /tmp/dxui-demo",
  "## CLI Add Smoke",
  "cargo run -p dioxus-ui-cli -- add button --root /tmp/dxui-demo",
  "## Generated Fixture Smoke",
  "scripts/generated-fixture-smoke.sh",
  "## Example Smoke",
  "scripts/example-smoke.sh",
];

for (const snippet of requiredReadmeSnippets) {
  if (!examplesReadme.includes(snippet)) {
    failures.push(`examples/README.md missing snippet: ${snippet}`);
  }
}

const scripts = packageJson.scripts ?? {};
if (scripts["verify:examples"] !== "scripts/example-smoke.sh") {
  failures.push("package.json script verify:examples must be scripts/example-smoke.sh");
}
if (scripts["verify:examples-metadata"] !== "node scripts/examples-metadata-verify.mjs") {
  failures.push("package.json script verify:examples-metadata must be node scripts/examples-metadata-verify.mjs");
}

const requiredSmokeFragments = [
  "cargo run -q -p dioxus-ui-web-demo",
  "cargo run -q -p dioxus-ui-desktop-demo",
  "dioxus-ui web demo",
  "dioxus-ui desktop demo",
];

for (const fragment of requiredSmokeFragments) {
  if (!exampleSmoke.includes(fragment)) {
    failures.push(`scripts/example-smoke.sh missing fragment: ${fragment}`);
  }
}

const requiredPreviewFragments = [
  {
    label: "web preview verifier",
    source: webPreviewVerify,
    fragments: [
      "examples/web-demo/src/bin/preview.rs",
      "examples/web-demo/assets/preview.css",
      "dioxus-ui-web-demo",
    ],
  },
  {
    label: "desktop preview verifier",
    source: desktopPreviewVerify,
    fragments: [
      "examples/desktop-demo/src/bin/preview.rs",
      "examples/desktop-demo/assets/preview.css",
      "dioxus-ui-desktop-demo",
    ],
  },
  {
    label: "mobile web profile verifier",
    source: mobileWebProfileVerify,
    fragments: [
      "examples/web-demo/src/bin/preview.rs",
      "examples/preview-states/src/lib.rs",
    ],
  },
];

for (const { label, source, fragments } of requiredPreviewFragments) {
  for (const fragment of fragments) {
    if (!source.includes(fragment)) {
      failures.push(`${label} missing fragment: ${fragment}`);
    }
  }
}

for (const examplePackage of examplePackages) {
  if (examplePackage.name === null) {
    continue;
  }

  if (!examplesReadme.includes(examplePackage.name)) {
    failures.push(`examples/README.md missing example package name: ${examplePackage.name}`);
  }
}

if (failures.length > 0) {
  console.error("examples metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log(
    `examples metadata verification passed (${examplePackages.length} example packages)`,
  );
}
