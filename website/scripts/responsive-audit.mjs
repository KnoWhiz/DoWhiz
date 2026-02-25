import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const publicDir = path.join(root, 'public');

const cssFiles = [
  path.join(root, 'src', 'index.css'),
  path.join(root, 'public', 'agent.css'),
  path.join(root, 'public', 'legal.css'),
  path.join(root, 'public', 'blog', 'blog.css')
];

function listHtmlFiles(dir) {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...listHtmlFiles(fullPath));
    } else if (entry.isFile() && entry.name.endsWith('.html')) {
      files.push(fullPath);
    }
  }

  return files;
}

function relative(filePath) {
  return path.relative(root, filePath);
}

const htmlFiles = [path.join(root, 'index.html'), ...listHtmlFiles(publicDir)];

const issues = [];
const warnings = [];

for (const htmlFile of htmlFiles) {
  const content = fs.readFileSync(htmlFile, 'utf8');

  if (!/<meta[^>]+name=["']viewport["'][^>]*>/i.test(content)) {
    issues.push(`[missing viewport] ${relative(htmlFile)}`);
  }

  const riskyInlinePattern = /style=["'][^"']*(?:min-width:\s*(\d{3,})px|width:\s*(\d{4,})px)[^"']*["']/gi;
  let match;
  while ((match = riskyInlinePattern.exec(content)) !== null) {
    const minWidth = Number(match[1] || 0);
    const width = Number(match[2] || 0);
    const riskyValue = Math.max(minWidth, width);
    if (riskyValue >= 700) {
      warnings.push(`[inline fixed width ${riskyValue}px] ${relative(htmlFile)}`);
      break;
    }
  }
}

for (const cssFile of cssFiles) {
  if (!fs.existsSync(cssFile)) {
    issues.push(`[missing css file] ${relative(cssFile)}`);
    continue;
  }

  const content = fs.readFileSync(cssFile, 'utf8');
  const hasTabletRule = /@media\s*\(max-width:\s*(?:10[0-9]{2}|9[0-9]{2})px\)/i.test(content);
  const hasMobileRule = /@media\s*\(max-width:\s*(?:7[0-9]{2}|6[0-9]{2})px\)/i.test(content);

  if (!hasTabletRule || !hasMobileRule) {
    issues.push(
      `[missing responsive breakpoints] ${relative(cssFile)} (tablet=${hasTabletRule}, mobile=${hasMobileRule})`
    );
  }
}

if (issues.length === 0) {
  console.log(`Responsive audit passed for ${htmlFiles.length} HTML files and ${cssFiles.length} CSS files.`);
} else {
  console.error('Responsive audit failed:');
  for (const issue of issues) {
    console.error(`- ${issue}`);
  }
}

if (warnings.length > 0) {
  console.log('Responsive audit warnings:');
  for (const warning of warnings) {
    console.log(`- ${warning}`);
  }
}

process.exit(issues.length > 0 ? 1 : 0);
