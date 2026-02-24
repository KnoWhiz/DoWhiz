> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# GitHub Actions CI/CD

> Run AI-powered code review, testing, and validation in secure E2B sandboxes from your GitHub Actions workflows.

CI/CD pipelines can use AI agents to review pull requests, generate tests, and validate code changes automatically. E2B sandboxes provide the secure, isolated execution environment where these agents can safely clone repositories, run untrusted code, and report results — all triggered by [GitHub Actions](https://docs.github.com/en/actions) on every pull request. Since each run gets a fresh sandbox, malicious or buggy PR code never touches your CI runner.

## GitHub Actions workflow

The workflow triggers on pull request events and runs a review script. `E2B_API_KEY` and the LLM API key are stored as [GitHub Actions secrets](https://docs.github.com/en/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions), while the built-in `GITHUB_TOKEN` is available automatically. The `permissions` block grants write access so the script can post PR comments.

<CodeGroup>
  ```yaml .github/workflows/ai-review.yml (JavaScript) theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  name: AI Code Review

  on:
    pull_request:
      types: [opened, synchronize]

  permissions:
    pull-requests: write

  jobs:
    ai-review:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4

        - name: Set up Node.js
          uses: actions/setup-node@v4
          with:
            node-version: "20"

        - name: Install dependencies
          run: npm install e2b openai

        - name: Run AI review
          env:
            E2B_API_KEY: ${{ secrets.E2B_API_KEY }}
            OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
            GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
            PR_REPO: ${{ github.event.pull_request.head.repo.full_name }}
            PR_BRANCH: ${{ github.event.pull_request.head.ref }}
            PR_NUMBER: ${{ github.event.pull_request.number }}
            GITHUB_REPOSITORY: ${{ github.repository }}
          run: node review.mjs
  ```

  ```yaml .github/workflows/ai-review.yml (Python) theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  name: AI Code Review

  on:
    pull_request:
      types: [opened, synchronize]

  permissions:
    pull-requests: write

  jobs:
    ai-review:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4

        - name: Set up Python
          uses: actions/setup-python@v5
          with:
            python-version: "3.12"

        - name: Install dependencies
          run: pip install e2b openai

        - name: Run AI review
          env:
            E2B_API_KEY: ${{ secrets.E2B_API_KEY }}
            OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
            GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
            PR_REPO: ${{ github.event.pull_request.head.repo.full_name }}
            PR_BRANCH: ${{ github.event.pull_request.head.ref }}
            PR_NUMBER: ${{ github.event.pull_request.number }}
            GITHUB_REPOSITORY: ${{ github.repository }}
          run: python review.py
  ```
</CodeGroup>

## Review script

The workflow calls this script on every PR. It runs five steps inside an E2B sandbox, keeping all untrusted code isolated from the CI runner.

<CodeGroup>
  ```typescript review.mjs expandable theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, CommandExitError } from 'e2b'
  import OpenAI from 'openai'

  // --- 1. Create sandbox ---
  const sandbox = await Sandbox.create({ timeoutMs: 300_000 })
  console.log('Sandbox created:', sandbox.sandboxId)

  // --- 2. Clone the PR branch ---
  const repoUrl = `https://github.com/${process.env.PR_REPO}.git`

  await sandbox.git.clone(repoUrl, {
    path: '/home/user/repo',
    branch: process.env.PR_BRANCH,
    username: 'x-access-token',
    password: process.env.GITHUB_TOKEN,
    depth: 1,
  })
  console.log('Repository cloned')

  // --- 3. Get the diff and send it to an LLM for review ---
  const diffResult = await sandbox.commands.run(
    'cd /home/user/repo && git diff origin/main...HEAD'
  )

  const openai = new OpenAI()
  const response = await openai.chat.completions.create({
    model: 'gpt-5.2-mini',
    messages: [
      {
        role: 'system',
        content:
          'You are a senior code reviewer. Analyze the following git diff and provide a concise review with actionable feedback. Focus on bugs, security issues, and code quality.',
      },
      {
        role: 'user',
        content: `Review this diff:\n\n${diffResult.stdout}`,
      },
    ],
  })

  const review = response.choices[0].message.content
  console.log('AI Review:', review)

  // --- 4. Run the test suite inside the sandbox ---
  await sandbox.commands.run('cd /home/user/repo && npm install', {
    onStdout: (data) => console.log(data),
    onStderr: (data) => console.error(data),
  })

  try {
    await sandbox.commands.run('cd /home/user/repo && npm test', {
      onStdout: (data) => console.log(data),
      onStderr: (data) => console.error(data),
    })
    console.log('All tests passed')
  } catch (err) {
    if (err instanceof CommandExitError) {
      console.error('Tests failed with exit code:', err.exitCode)
      await sandbox.kill()
      process.exit(1)
    }
    throw err
  }

  // --- 5. Post results as a PR comment ---
  const prNumber = process.env.PR_NUMBER
  const repo = process.env.GITHUB_REPOSITORY

  await fetch(
    `https://api.github.com/repos/${repo}/issues/${prNumber}/comments`,
    {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${process.env.GITHUB_TOKEN}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        body: `## AI Code Review\n\n${review}`,
      }),
    }
  )

  await sandbox.kill()
  console.log('Done')
  ```

  ```python review.py expandable theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  import sys
  import requests
  from e2b import Sandbox, CommandExitException
  from openai import OpenAI

  # --- 1. Create sandbox ---
  sandbox = Sandbox.create(timeout=300)
  print(f"Sandbox created: {sandbox.sandbox_id}")

  # --- 2. Clone the PR branch ---
  repo_url = f"https://github.com/{os.environ['PR_REPO']}.git"

  sandbox.git.clone(
      repo_url,
      path="/home/user/repo",
      branch=os.environ["PR_BRANCH"],
      username="x-access-token",
      password=os.environ["GITHUB_TOKEN"],
      depth=1,
  )
  print("Repository cloned")

  # --- 3. Get the diff and send it to an LLM for review ---
  diff_result = sandbox.commands.run(
      "cd /home/user/repo && git diff origin/main...HEAD"
  )

  client = OpenAI()
  response = client.chat.completions.create(
      model="gpt-5.2-mini",
      messages=[
          {
              "role": "system",
              "content": "You are a senior code reviewer. Analyze the following git diff and provide a concise review with actionable feedback. Focus on bugs, security issues, and code quality.",
          },
          {
              "role": "user",
              "content": f"Review this diff:\n\n{diff_result.stdout}",
          },
      ],
  )

  review = response.choices[0].message.content
  print("AI Review:", review)

  # --- 4. Run the test suite inside the sandbox ---
  sandbox.commands.run(
      "cd /home/user/repo && npm install",
      on_stdout=lambda data: print(data),
      on_stderr=lambda data: print(data, file=sys.stderr),
  )

  try:
      sandbox.commands.run(
          "cd /home/user/repo && npm test",
          on_stdout=lambda data: print(data),
          on_stderr=lambda data: print(data, file=sys.stderr),
      )
      print("All tests passed")
  except CommandExitException as err:
      print(f"Tests failed with exit code: {err.exit_code}", file=sys.stderr)
      sandbox.kill()
      sys.exit(1)

  # --- 5. Post results as a PR comment ---
  pr_number = os.environ["PR_NUMBER"]
  repo = os.environ["GITHUB_REPOSITORY"]

  requests.post(
      f"https://api.github.com/repos/{repo}/issues/{pr_number}/comments",
      headers={
          "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
          "Content-Type": "application/json",
      },
      json={
          "body": f"## AI Code Review\n\n{review}",
      },
  )

  sandbox.kill()
  print("Done")
  ```
</CodeGroup>

1. **Create sandbox** — `Sandbox.create()` spins up an isolated Linux environment with a 5-minute timeout
2. **Clone the PR** — `sandbox.git.clone()` checks out the PR branch using `x-access-token` + `GITHUB_TOKEN` for [authentication](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/authenticating-as-a-github-app-installation)
3. **AI review** — runs `git diff` inside the sandbox, sends the output to an LLM — swap the model for any provider via [Connect LLMs](/docs/quickstart/connect-llms)
4. **Run tests** — `commands.run()` streams output in real time and throws on failure (`CommandExitError` / `CommandExitException`)
5. **Post results** — comments the review on the PR via the GitHub REST API, then destroys the sandbox

## Related guides

<CardGroup cols={3}>
  <Card title="Git integration" icon="code-branch" href="/docs/sandbox/git-integration">
    Clone repos, manage branches, and push changes from sandboxes
  </Card>

  <Card title="Connect LLMs" icon="brain" href="/docs/quickstart/connect-llms">
    Integrate AI models with sandboxes using tool calling
  </Card>

  <Card title="Custom templates" icon="cube" href="/docs/template/quickstart">
    Build reproducible sandbox environments for your pipelines
  </Card>
</CardGroup>
