---
name: bright-data-social
description: Use when a task needs Bright Data-powered social media extraction, profile lookup, post fetches, or auth verification from a DoWhiz workspace or container. Trigger whenever the user wants structured LinkedIn, X/Twitter, or Bright Data-backed Xiaohongshu/RedNote data through BRIGHT_DATA_API_KEY without changing runtime code.
---

# Bright Data Social Scraping

Use this skill when the task should stay inside the existing DoWhiz skill-loading flow and fetch social media data through Bright Data.

Shared skill path inside a task workspace:

```bash
.agents/skills/bright-data-social
```

Helper script:

```bash
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py --help
```

## Credential Rules

The helper reads:

- `BRIGHT_DATA_API_KEY`: required runtime source of truth
- `BRIGHTDATA_API_KEY`: optional Bright Data CLI alias; the helper auto-bridges it when only `BRIGHT_DATA_API_KEY` is present

If the variable is present in a nearby `.env` or `DoWhiz_service/.env` but not exported by the shell,
the helper will load that file automatically before failing.

For Xiaohongshu / RedNote custom scrapers, Bright Data usually needs a pre-provisioned Scraper Studio deployment. Configure one of:

- `BRIGHT_DATA_XIAOHONGSHU_COLLECTOR`
- `BRIGHT_DATA_XIAOHONGSHU_TRIGGER_URL`

If neither Xiaohongshu variable exists, do not pretend the account can scrape Xiaohongshu directly. Report that Bright Data-side provisioning is still missing.

## Supported Commands

Validate auth first:

```bash
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py status
```

This health check validates both:

- whether Bright Data recognizes the key
- whether dataset APIs are actually reachable for this account

That matters because Bright Data can report proxy-zone limitations in `/status` while dataset scraping still works.

List available Bright Data datasets:

```bash
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py datasets --filter linkedin
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py datasets --filter 'x|twitter'
```

LinkedIn:

```bash
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py linkedin-person \
  --url https://www.linkedin.com/in/satyanadella

python3 .agents/skills/bright-data-social/scripts/bright_data_social.py linkedin-company \
  --url https://www.linkedin.com/company/microsoft

python3 .agents/skills/bright-data-social/scripts/bright_data_social.py linkedin-post \
  --url https://www.linkedin.com/posts/satyanadella_congrats-shantanu-on-a-legendary-run-at-activity-7437966653801320448-HL0x
```

X / Twitter:

```bash
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py x-profile \
  --url https://x.com/OpenAI

python3 .agents/skills/bright-data-social/scripts/bright_data_social.py x-post \
  --url https://x.com/OpenAI/status/1901336014392379444
```

Important:

- `x-profile` expects a profile URL.
- `x-post` expects a single status/post URL, not a profile URL.

Xiaohongshu / RedNote via a prebuilt Bright Data custom scraper:

```bash
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py xiaohongshu \
  --url https://www.xiaohongshu.com/explore/EXAMPLE_NOTE_ID
```

If the custom scraper needs richer input, pass JSON explicitly:

```bash
python3 .agents/skills/bright-data-social/scripts/bright_data_social.py xiaohongshu \
  --payload-json '{"url":"https://www.xiaohongshu.com/explore/EXAMPLE_NOTE_ID","keyword":"咖啡"}'
```

## Workflow

1. Run `status` before the first Bright Data call in a task so you can prove the credential is loaded.
2. Prefer the helper script over ad hoc curl so the same env handling works in local workspaces and containers.
3. For LinkedIn and X, use the built-in subcommands backed by verified Bright Data dataset IDs.
4. For Xiaohongshu, verify that `BRIGHT_DATA_XIAOHONGSHU_COLLECTOR` or `BRIGHT_DATA_XIAOHONGSHU_TRIGGER_URL` exists before you promise a result.
5. When results are large, save them to a workspace file with `--output` and summarize from that file instead of flooding stdout.

## Optional CLI Fallback

If you want Bright Data CLI behavior inside the same task, bridge the env var first:

```bash
export BRIGHTDATA_API_KEY="${BRIGHTDATA_API_KEY:-$BRIGHT_DATA_API_KEY}"
npx --yes @brightdata/cli pipelines list
```

Use CLI only when you specifically need a Bright Data pipeline workflow. For normal DoWhiz tasks, prefer the helper script because it already matches the repo's env naming and skill-loading model.
