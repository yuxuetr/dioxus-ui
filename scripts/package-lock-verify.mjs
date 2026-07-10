#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const packageJsonPath = join(repoRoot, "package.json");
const packageLockPath = join(repoRoot, "package-lock.json");

const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf8"));
const failures = [];

if (!existsSync(packageLockPath)) {
  failures.push("package-lock.json is missing");
} else {
  const packageLock = JSON.parse(readFileSync(packageLockPath, "utf8"));
  const lockRoot = packageLock.packages?.[""];
  const packageDevDependencies = packageJson.devDependencies ?? {};
  const lockDevDependencies = lockRoot?.devDependencies ?? {};

  if (packageLock.lockfileVersion !== 3) {
    failures.push(`expected lockfileVersion 3, found ${packageLock.lockfileVersion}`);
  }

  if (packageLock.name !== packageJson.name) {
    failures.push(`top-level lockfile name mismatch: expected ${packageJson.name}, found ${packageLock.name}`);
  }

  if (packageLock.version !== packageJson.version) {
    failures.push(`top-level lockfile version mismatch: expected ${packageJson.version}, found ${packageLock.version}`);
  }

  if (lockRoot === undefined) {
    failures.push("package-lock.json is missing packages[\"\"] root metadata");
  } else {
    if (lockRoot.name !== packageJson.name) {
      failures.push(`root lockfile name mismatch: expected ${packageJson.name}, found ${lockRoot.name}`);
    }

    if (lockRoot.version !== packageJson.version) {
      failures.push(`root lockfile version mismatch: expected ${packageJson.version}, found ${lockRoot.version}`);
    }

    if (JSON.stringify(lockDevDependencies) !== JSON.stringify(packageDevDependencies)) {
      failures.push("root lockfile devDependencies do not match package.json devDependencies");
    }
  }
}

if (failures.length > 0) {
  console.error("package lock verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("package lock verification passed");
}
