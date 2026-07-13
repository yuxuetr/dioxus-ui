#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const cargoLockPath = join(repoRoot, "Cargo.lock");
const failures = [];

const getStringField = (text, field) => {
  const match = text.match(new RegExp(`^\\s*${field}\\s*=\\s*"([^"]+)"\\s*$`, "m"));
  return match?.[1] ?? null;
};

const parseCargoLockPackages = (cargoLock) => {
  return cargoLock
    .split(/\n\[\[package\]\]\n/g)
    .slice(1)
    .map((block) => {
      return {
        name: getStringField(block, "name"),
        version: getStringField(block, "version"),
        source: getStringField(block, "source"),
      };
    })
    .filter((entry) => entry.name !== null && entry.version !== null);
};

if (!existsSync(cargoLockPath)) {
  failures.push("Cargo.lock is missing");
} else {
  const cargoLock = readFileSync(cargoLockPath, "utf8");
  const lockVersionMatch = cargoLock.match(/^version\s*=\s*(\d+)\s*$/m);
  const lockVersion = lockVersionMatch?.[1] ?? null;

  if (lockVersion !== "4") {
    failures.push(`expected Cargo.lock version 4, found ${lockVersion ?? "missing"}`);
  }

  const cargoMetadata = spawnSync("cargo", ["metadata", "--locked", "--no-deps", "--format-version", "1"], {
    cwd: repoRoot,
    encoding: "utf8",
  });

  if (cargoMetadata.error !== undefined) {
    failures.push(`failed to run cargo metadata: ${cargoMetadata.error.message}`);
  } else if (cargoMetadata.status !== 0) {
    failures.push("cargo metadata --locked --no-deps --format-version 1 failed");
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
      const lockPackages = parseCargoLockPackages(cargoLock);
      const lockPackagesByName = new Map();

      for (const lockPackage of lockPackages) {
        const current = lockPackagesByName.get(lockPackage.name) ?? [];
        current.push(lockPackage);
        lockPackagesByName.set(lockPackage.name, current);
      }

      const workspaceMemberIds = new Set(metadata.workspace_members ?? []);
      const workspacePackages = (metadata.packages ?? [])
        .filter((packageMetadata) => workspaceMemberIds.has(packageMetadata.id))
        .sort((left, right) => left.name.localeCompare(right.name));

      for (const packageMetadata of workspacePackages) {
        const lockEntries = lockPackagesByName.get(packageMetadata.name) ?? [];
        if (lockEntries.length === 0) {
          failures.push(`Cargo.lock is missing workspace package: ${packageMetadata.name}`);
          continue;
        }

        const localEntries = lockEntries.filter((entry) => entry.source === null);
        if (localEntries.length !== 1) {
          failures.push(
            `Cargo.lock has ${localEntries.length} local entries for workspace package: ${packageMetadata.name}`,
          );
          continue;
        }

        const lockEntry = localEntries[0];
        if (lockEntry.version !== packageMetadata.version) {
          failures.push(
            `Cargo.lock records ${packageMetadata.name} ${lockEntry.version} but cargo metadata resolves ${packageMetadata.version}`,
          );
        }
      }
    }
  }
}

if (failures.length > 0) {
  console.error("cargo lock metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("cargo lock metadata verification passed");
}
