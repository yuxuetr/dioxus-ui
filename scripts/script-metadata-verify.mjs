#!/usr/bin/env node
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, extname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const scriptsDir = join(repoRoot, "scripts");
const packageJsonPath = join(repoRoot, "package.json");
const helperOnlyNodeScripts = new Set([
  "scripts/docs-catalog-builder.mjs",
  "scripts/preview-tailwind.mjs",
  "scripts/browser-check-support.mjs",
  "scripts/oklch-contrast.mjs",
]);
const failures = [];

const readFirstLine = (path) => {
  return readFileSync(path, "utf8").split(/\r?\n/, 1)[0] ?? "";
};

const isExecutable = (path) => {
  return (statSync(path).mode & 0o111) !== 0;
};

const directScriptTargets = new Set();

if (!existsSync(packageJsonPath)) {
  failures.push("package.json is missing");
} else {
  const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf8"));
  const packageScripts = packageJson.scripts ?? {};

  for (const command of Object.values(packageScripts)) {
    if (typeof command !== "string") {
      continue;
    }

    const directScriptMatches = command.matchAll(/(?:^|[\s&|;])(scripts\/[^\s&|;]+)/g);
    for (const match of directScriptMatches) {
      directScriptTargets.add(match[1]);
    }
  }
}

if (!existsSync(scriptsDir)) {
  failures.push("scripts directory is missing");
} else {
  const scriptPaths = readdirSync(scriptsDir, { withFileTypes: true })
    .filter((entry) => entry.isFile())
    .map((entry) => `scripts/${entry.name}`)
    .sort();

  for (const relativePath of scriptPaths) {
    const absolutePath = join(repoRoot, relativePath);
    const extension = extname(relativePath);
    const firstLine = readFirstLine(absolutePath);

    if (extension === ".sh") {
      if (firstLine !== "#!/usr/bin/env bash") {
        failures.push(`${relativePath} must start with #!/usr/bin/env bash`);
      }

      if (!isExecutable(absolutePath)) {
        failures.push(`${relativePath} must be executable`);
      }
    }

    if (extension === ".mjs" && !helperOnlyNodeScripts.has(relativePath)) {
      if (firstLine !== "#!/usr/bin/env node") {
        failures.push(`${relativePath} must start with #!/usr/bin/env node`);
      }
    }
  }
}

for (const relativePath of directScriptTargets) {
  const absolutePath = join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`package.json references missing script target: ${relativePath}`);
    continue;
  }

  if (extname(relativePath) === ".sh" && !isExecutable(absolutePath)) {
    failures.push(`package.json direct shell target must be executable: ${relativePath}`);
  }
}

if (failures.length > 0) {
  console.error("script metadata verification failed");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exitCode = 1;
} else {
  console.log("script metadata verification passed");
}
