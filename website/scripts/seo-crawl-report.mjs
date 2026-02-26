import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const sitemapPath = path.join(root, 'public', 'sitemap.xml');
const outputDir = path.join(root, 'reports');
const crawlTimestamp = new Date().toISOString();
const reportDate = crawlTimestamp.slice(0, 10);
const markdownOutputPath = path.join(outputDir, `seo-crawl-report-${reportDate}.md`);
const jsonOutputPath = path.join(outputDir, `seo-crawl-report-${reportDate}.json`);

const ENTITY_MAP = new Map([
  ['amp', '&'],
  ['lt', '<'],
  ['gt', '>'],
  ['quot', '"'],
  ['apos', "'"],
  ['nbsp', ' ']
]);

function decodeHtmlEntities(input) {
  return input.replace(/&(#x?[0-9a-f]+|[a-z]+);/gi, (_, rawEntity) => {
    const entity = rawEntity.toLowerCase();
    if (entity.startsWith('#x')) {
      const codePoint = Number.parseInt(entity.slice(2), 16);
      return Number.isFinite(codePoint) ? String.fromCodePoint(codePoint) : _;
    }
    if (entity.startsWith('#')) {
      const codePoint = Number.parseInt(entity.slice(1), 10);
      return Number.isFinite(codePoint) ? String.fromCodePoint(codePoint) : _;
    }
    return ENTITY_MAP.get(entity) ?? _;
  });
}

function cleanText(input) {
  return decodeHtmlEntities(
    input
      .replace(/<script[\s\S]*?<\/script>/gi, ' ')
      .replace(/<style[\s\S]*?<\/style>/gi, ' ')
      .replace(/<[^>]+>/g, ' ')
  )
    .replace(/\s+/g, ' ')
    .trim();
}

function normalizeKey(input) {
  return input.trim().toLowerCase().replace(/\s+/g, ' ');
}

function extractUrlsFromSitemap(xmlContent) {
  const urls = [];
  const urlPattern = /<loc>\s*([^<]+?)\s*<\/loc>/gi;
  let match;
  while ((match = urlPattern.exec(xmlContent)) !== null) {
    urls.push(match[1].trim());
  }
  return urls;
}

function extractAttribute(tag, attributeName) {
  const attrPattern = new RegExp(
    `${attributeName}\\s*=\\s*(\"([^\"]*)\"|'([^']*)'|([^\\s>]+))`,
    'i'
  );
  const match = tag.match(attrPattern);
  if (!match) {
    return '';
  }
  return match[2] ?? match[3] ?? match[4] ?? '';
}

function extractMetaContent(html, metaName) {
  const metaTags = html.match(/<meta\b[^>]*>/gi) ?? [];
  for (const tag of metaTags) {
    const name = extractAttribute(tag, 'name').toLowerCase();
    if (name === metaName.toLowerCase()) {
      return cleanText(extractAttribute(tag, 'content'));
    }
  }
  return '';
}

function extractTitle(html) {
  const match = html.match(/<title[^>]*>([\s\S]*?)<\/title>/i);
  return match ? cleanText(match[1]) : '';
}

function extractH1s(html) {
  const h1s = [];
  const pattern = /<h1\b[^>]*>([\s\S]*?)<\/h1>/gi;
  let match;
  while ((match = pattern.exec(html)) !== null) {
    const cleaned = cleanText(match[1]);
    if (cleaned) {
      h1s.push(cleaned);
    }
  }
  return h1s;
}

function statusBucket(status) {
  if (!Number.isFinite(status)) {
    return 'error';
  }
  if (status >= 200 && status < 300) {
    return '2xx';
  }
  if (status >= 300 && status < 400) {
    return '3xx';
  }
  if (status >= 400 && status < 500) {
    return '4xx';
  }
  if (status >= 500 && status < 600) {
    return '5xx';
  }
  return 'other';
}

async function fetchWithTimeout(url, timeoutMs = 20000) {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    return await fetch(url, {
      headers: {
        'user-agent': 'DoWhiz-SEO-Crawl/1.0 (+https://dowhiz.com)'
      },
      redirect: 'follow',
      signal: controller.signal
    });
  } finally {
    clearTimeout(timeout);
  }
}

async function crawlUrl(url) {
  const startedAt = Date.now();
  try {
    const response = await fetchWithTimeout(url);
    const status = response.status;
    const statusClass = statusBucket(status);
    let html = '';

    if (status >= 200 && status < 400) {
      html = await response.text();
    }

    const title = html ? extractTitle(html) : '';
    const metaDescription = html ? extractMetaContent(html, 'description') : '';
    const robots = html ? extractMetaContent(html, 'robots') : '';
    const h1s = html ? extractH1s(html) : [];
    const noindex = robots.toLowerCase().includes('noindex');

    return {
      url,
      finalUrl: response.url || url,
      status,
      statusClass,
      durationMs: Date.now() - startedAt,
      indexable: status >= 200 && status < 300 && !noindex,
      title,
      titleLength: title.length,
      metaDescription,
      metaDescriptionLength: metaDescription.length,
      robots,
      noindex,
      h1Count: h1s.length,
      h1s,
      primaryH1: h1s[0] ?? '',
      error: ''
    };
  } catch (error) {
    return {
      url,
      finalUrl: url,
      status: NaN,
      statusClass: 'error',
      durationMs: Date.now() - startedAt,
      indexable: false,
      title: '',
      titleLength: 0,
      metaDescription: '',
      metaDescriptionLength: 0,
      robots: '',
      noindex: false,
      h1Count: 0,
      h1s: [],
      primaryH1: '',
      error: error instanceof Error ? error.message : String(error)
    };
  }
}

function buildDuplicateGroups(rows, pickValue) {
  const map = new Map();
  for (const row of rows) {
    const value = pickValue(row);
    if (!value) {
      continue;
    }
    const key = normalizeKey(value);
    if (!map.has(key)) {
      map.set(key, { value, urls: [] });
    }
    map.get(key).urls.push(row.url);
  }
  return Array.from(map.values()).filter((entry) => entry.urls.length > 1);
}

function toMarkdownList(items) {
  if (items.length === 0) {
    return '- (none)';
  }
  return items.map((item) => `- ${item}`).join('\n');
}

async function main() {
  if (!fs.existsSync(sitemapPath)) {
    throw new Error(`Sitemap not found at ${sitemapPath}`);
  }

  const sitemapXml = fs.readFileSync(sitemapPath, 'utf8');
  const urls = extractUrlsFromSitemap(sitemapXml);
  if (urls.length === 0) {
    throw new Error(`No URLs found in sitemap: ${sitemapPath}`);
  }

  const crawlResults = [];
  for (const url of urls) {
    crawlResults.push(await crawlUrl(url));
  }

  const indexablePages = crawlResults.filter((row) => row.indexable);
  const status4xx = crawlResults.filter((row) => row.statusClass === '4xx');
  const status5xx = crawlResults.filter((row) => row.statusClass === '5xx');
  const fetchErrors = crawlResults.filter((row) => row.statusClass === 'error');

  const duplicateTitleGroups = buildDuplicateGroups(indexablePages, (row) => row.title);
  const duplicateMetaGroups = buildDuplicateGroups(indexablePages, (row) => row.metaDescription);
  const duplicateH1Groups = buildDuplicateGroups(indexablePages, (row) => row.primaryH1);

  const duplicateTitleUrls = new Set(duplicateTitleGroups.flatMap((entry) => entry.urls));
  const duplicateMetaUrls = new Set(duplicateMetaGroups.flatMap((entry) => entry.urls));

  const missingTitlePages = indexablePages.filter((row) => !row.title);
  const missingMetaPages = indexablePages.filter((row) => !row.metaDescription);

  const titleIssuePages = new Set([
    ...missingTitlePages.map((row) => row.url),
    ...Array.from(duplicateTitleUrls)
  ]);
  const metaIssuePages = new Set([
    ...missingMetaPages.map((row) => row.url),
    ...Array.from(duplicateMetaUrls)
  ]);

  const summary = {
    generated_at: crawlTimestamp,
    source_sitemap: 'website/public/sitemap.xml',
    crawl_target_count: urls.length,
    indexable_pages: indexablePages.length,
    title_issue_pages: titleIssuePages.size,
    meta_description_issue_pages: metaIssuePages.size,
    duplicate_h1_groups: duplicateH1Groups.length,
    duplicate_h1_pages: duplicateH1Groups.flatMap((entry) => entry.urls).length,
    status_4xx: status4xx.length,
    status_5xx: status5xx.length,
    fetch_errors: fetchErrors.length
  };

  const markdown = [
    '# SEO Crawl Report',
    '',
    `Generated at: ${crawlTimestamp}`,
    `Run command: \`npm run seo:crawl\``,
    `Source sitemap: \`website/public/sitemap.xml\``,
    '',
    '## Summary Counts',
    '',
    '| Metric | Count |',
    '| --- | ---: |',
    `| URLs crawled | ${summary.crawl_target_count} |`,
    `| Indexable pages (2xx and not noindex) | ${summary.indexable_pages} |`,
    `| Title issue pages (missing/duplicate) | ${summary.title_issue_pages} |`,
    `| Meta description issue pages (missing/duplicate) | ${summary.meta_description_issue_pages} |`,
    `| Duplicate H1 groups | ${summary.duplicate_h1_groups} |`,
    `| Pages in duplicate H1 groups | ${summary.duplicate_h1_pages} |`,
    `| 4xx responses | ${summary.status_4xx} |`,
    `| 5xx responses | ${summary.status_5xx} |`,
    `| Fetch errors | ${summary.fetch_errors} |`,
    '',
    '## Title Issue Details',
    '',
    `Missing title pages: ${missingTitlePages.length}`,
    toMarkdownList(missingTitlePages.map((row) => row.url)),
    '',
    `Duplicate title groups: ${duplicateTitleGroups.length}`,
    toMarkdownList(
      duplicateTitleGroups.map(
        (group) => `"${group.value}" (${group.urls.length} pages): ${group.urls.join(', ')}`
      )
    ),
    '',
    '## Meta Description Issue Details',
    '',
    `Missing meta description pages: ${missingMetaPages.length}`,
    toMarkdownList(missingMetaPages.map((row) => row.url)),
    '',
    `Duplicate meta description groups: ${duplicateMetaGroups.length}`,
    toMarkdownList(
      duplicateMetaGroups.map(
        (group) => `"${group.value}" (${group.urls.length} pages): ${group.urls.join(', ')}`
      )
    ),
    '',
    '## Duplicate H1 Details',
    '',
    toMarkdownList(
      duplicateH1Groups.map(
        (group) => `"${group.value}" (${group.urls.length} pages): ${group.urls.join(', ')}`
      )
    ),
    '',
    '## 4xx / 5xx / Fetch Error URLs',
    '',
    `4xx URLs: ${status4xx.length}`,
    toMarkdownList(status4xx.map((row) => `${row.url} (status ${row.status})`)),
    '',
    `5xx URLs: ${status5xx.length}`,
    toMarkdownList(status5xx.map((row) => `${row.url} (status ${row.status})`)),
    '',
    `Fetch errors: ${fetchErrors.length}`,
    toMarkdownList(fetchErrors.map((row) => `${row.url} (${row.error})`)),
    '',
    '## Crawl Matrix',
    '',
    '| URL | Status | Indexable | Title chars | Meta chars | H1 count |',
    '| --- | ---: | :---: | ---: | ---: | ---: |',
    ...crawlResults.map((row) => {
      const status = Number.isFinite(row.status) ? String(row.status) : 'ERR';
      const indexable = row.indexable ? 'yes' : 'no';
      return `| ${row.url} | ${status} | ${indexable} | ${row.titleLength} | ${row.metaDescriptionLength} | ${row.h1Count} |`;
    }),
    ''
  ].join('\n');

  fs.mkdirSync(outputDir, { recursive: true });
  fs.writeFileSync(markdownOutputPath, markdown, 'utf8');
  fs.writeFileSync(
    jsonOutputPath,
    JSON.stringify(
      {
        summary,
        crawl_results: crawlResults
      },
      null,
      2
    ),
    'utf8'
  );

  console.log(`SEO crawl report written: ${markdownOutputPath}`);
  console.log(`SEO crawl JSON written: ${jsonOutputPath}`);
  console.log(JSON.stringify(summary, null, 2));
}

main().catch((error) => {
  console.error(error instanceof Error ? error.stack : String(error));
  process.exit(1);
});
