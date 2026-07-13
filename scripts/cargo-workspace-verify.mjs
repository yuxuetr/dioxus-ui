#!/usr/bin/env node
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const rootCargoPath = join(repoRoot, "Cargo.toml");
const cratesDir = join(repoRoot, "crates");
const requiredWorkspaceFields = ["version", "edition", "license", "repository"];
const failures = [];

const readText = (path) => {
  return readFileSync(path, "utf8");
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

const hasWorkspaceInheritance = (section, field) => {
  return new RegExp(`^\\s*${field}\\.workspace\\s*=\\s*true\\s*$`, "m").test(section);
};

const normalizePath = (path) => {
  return resolve(path);
};

if (!existsSync(rootCargoPath)) {
  failures.push("root Cargo.toml is missing");
} else {
  const rootCargo = readText(rootCargoPath);
  const workspacePackage = getSection(rootCargo, "workspace.package");
  if (workspacePackage === null) {
    failures.push("root Cargo.toml is missing [workspace.package]");
  } else {
    const workspaceFields = Object.fromEntries(
      requiredWorkspaceFields.map((field) => {
        return [field, getStringField(workspacePackage, field)];
      }),
    );

    for (const field of requiredWorkspaceFields) {
      if (workspaceFields[field] === null) {
        failures.push(`root [workspace.package] is missing string field: ${field}`);
      }
    }

    if (existsSync(cratesDir)) {
      const crateManifestPaths = readdirSync(cratesDir, { withFileTypes: true })
        .filter((entry) => entry.isDirectory())
        .map((entry) => join(cratesDir, entry.name, "Cargo.toml"))
        .filter((path) => existsSync(path))
        .sort();

      for (const manifestPath of crateManifestPaths) {
        const manifest = readText(manifestPath);
        const packageSection = getSection(manifest, "package");
        const label = relative(repoRoot, manifestPath);
        if (packageSection === null) {
          failures.push(`${label} is missing [package]`);
          continue;
        }

        for (const field of requiredWorkspaceFields) {
          if (!hasWorkspaceInheritance(packageSection, field)) {
            failures.push(`${label} must inherit package.${field} with ${field}.workspace = true`);
          }
        }
      }

      const cargoMetadata = spawnSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], {
        cwd: repoRoot,
        encoding: "utf8",
      });

      if (cargoMetadata.error !== undefined) {
        failures.push(`failed to run cargo metadata: ${cargoMetadata.error.message}`);
      } else if (cargoMetadata.status !== 0) {
        failures.push("cargo metadata --no-deps --format-version 1 failed");
        if (cargoMetadata.stderr.trim().length > 0) {
          failures.push(cargoMetadata.stderr.trim());
        }
      } else {
        let metadata = null;
        try {
          metadata = JSON.parse(cargoMetadata.stdout);
        } catch (error) {
          failures.push(`failed to parse cargo metadata JSON: ${error.message}`);
        }

        if (metadata !== null) {
          const workspaceMemberIds = new Set(metadata.workspace_members ?? []);
          const crateManifestSet = new Set(crateManifestPaths.map(normalizePath));
          const seenCrateManifests = new Set();

          for (const packageMetadata of metadata.packages ?? []) {
            const manifestPath = normalizePath(packageMetadata.manifest_path);
            if (!crateManifestSet.has(manifestPath)) {
              continue;
            }
            seenCrateManifests.add(manifestPath);

            const label = relative(repoRoot, manifestPath);
            if (!workspaceMemberIds.has(packageMetadata.id)) {
              failures.push(`${label} is not a Cargo workspace member`);
            }

            for (const field of requiredWorkspaceFields) {
              const expectedValue = workspaceFields[field];
              if (expectedValue !== null && packageMetadata[field] !== expectedValue) {
                failures.push(
                  `${label} resolves ${field}=${packageMetadata[field]} but workspace ${field}=${expectedValue}`,
                );
              }
            }
          }

          for (const manifestPath of crateManifestSet) {
            if (!seenCrateManifests.has(manifestPath)) {
              failures.push(`${relative(repoRoot, manifestPath)} is missing from cargo metadata packages`);
            }
          }
        }
      }
    } else {
      failures.push("crates directory is missing");
    }
  }
}

if (failures.length > 0) {
  console.error("cargo workspace metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("cargo workspace metadata verification passed");
}
