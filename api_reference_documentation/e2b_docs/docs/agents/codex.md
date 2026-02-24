> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Codex

> Run OpenAI Codex in a secure E2B sandbox with full filesystem, terminal, and git access.

[Codex](https://github.com/openai/codex) is OpenAI's open-source coding agent. E2B provides a pre-built `codex` template with Codex already installed.

## CLI

Spin up a sandbox with the [E2B CLI](/docs/cli).

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sbx create codex
```

Once inside the sandbox, start Codex.

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
codex
```

## Run headless

Use `codex exec` for non-interactive mode and `--full-auto` to auto-approve tool calls (safe inside E2B sandboxes). Pass `--skip-git-repo-check` to bypass git directory ownership checks inside the sandbox. Pass `CODEX_API_KEY` as an environment variable.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('codex', {
    envs: { CODEX_API_KEY: process.env.CODEX_API_KEY },
  })

  const result = await sandbox.commands.run(
    `codex exec --full-auto --skip-git-repo-check "Create a hello world HTTP server in Go"`
  )

  console.log(result.stdout)
  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("codex", envs={
      "CODEX_API_KEY": os.environ["CODEX_API_KEY"],
  })

  result = sandbox.commands.run(
      'codex exec --full-auto --skip-git-repo-check "Create a hello world HTTP server in Go"',
  )

  print(result.stdout)
  sandbox.kill()
  ```
</CodeGroup>

### Example: work on a cloned repository

Use `-C` to set Codex's working directory to a cloned repo.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('codex', {
    envs: { CODEX_API_KEY: process.env.CODEX_API_KEY },
    timeoutMs: 600_000,
  })

  await sandbox.git.clone('https://github.com/your-org/your-repo.git', {
    path: '/home/user/repo',
    username: 'x-access-token',
    password: process.env.GITHUB_TOKEN,
    depth: 1,
  })

  const result = await sandbox.commands.run(
    `codex exec --full-auto --skip-git-repo-check -C /home/user/repo "Add error handling to all API endpoints"`,
    { onStdout: (data) => process.stdout.write(data) }
  )

  const diff = await sandbox.commands.run('cd /home/user/repo && git diff')
  console.log(diff.stdout)

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("codex", envs={
      "CODEX_API_KEY": os.environ["CODEX_API_KEY"],
  }, timeout=600)

  sandbox.git.clone("https://github.com/your-org/your-repo.git",
      path="/home/user/repo",
      username="x-access-token",
      password=os.environ["GITHUB_TOKEN"],
      depth=1,
  )

  result = sandbox.commands.run(
      'codex exec --full-auto --skip-git-repo-check -C /home/user/repo "Add error handling to all API endpoints"',
      on_stdout=lambda data: print(data, end=""),
  )

  diff = sandbox.commands.run("cd /home/user/repo && git diff")
  print(diff.stdout)

  sandbox.kill()
  ```
</CodeGroup>

## Schema-validated output

Use `--output-schema` to constrain the agent's final response to a JSON Schema. This ensures the output conforms to a specific structure — useful for building reliable pipelines.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('codex', {
    envs: { CODEX_API_KEY: process.env.CODEX_API_KEY },
  })

  await sandbox.files.write('/home/user/schema.json', JSON.stringify({
    type: 'object',
    properties: {
      issues: {
        type: 'array',
        items: {
          type: 'object',
          properties: {
            file: { type: 'string' },
            line: { type: 'number' },
            severity: { type: 'string', enum: ['low', 'medium', 'high', 'critical'] },
            description: { type: 'string' },
          },
          required: ['file', 'severity', 'description'],
        },
      },
    },
    required: ['issues'],
  }))

  const result = await sandbox.commands.run(
    `codex exec --full-auto --skip-git-repo-check --output-schema /home/user/schema.json -C /home/user/repo "Review this codebase for security issues"`
  )

  const response = JSON.parse(result.stdout)
  console.log(response.issues)

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  import json
  from e2b import Sandbox

  sandbox = Sandbox.create("codex", envs={
      "CODEX_API_KEY": os.environ["CODEX_API_KEY"],
  })

  sandbox.files.write("/home/user/schema.json", json.dumps({
      "type": "object",
      "properties": {
          "issues": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                      "file": {"type": "string"},
                      "line": {"type": "number"},
                      "severity": {"type": "string", "enum": ["low", "medium", "high", "critical"]},
                      "description": {"type": "string"},
                  },
                  "required": ["file", "severity", "description"],
              },
          },
      },
      "required": ["issues"],
  }))

  result = sandbox.commands.run(
      'codex exec --full-auto --skip-git-repo-check --output-schema /home/user/schema.json -C /home/user/repo "Review this codebase for security issues"',
  )

  response = json.loads(result.stdout)
  print(response["issues"])

  sandbox.kill()
  ```
</CodeGroup>

## Streaming events

Use `--json` to get a JSONL event stream. Each line is a JSON object representing an agent event (tool calls, file changes, messages). Progress goes to stderr; events go to stdout.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('codex', {
    envs: { CODEX_API_KEY: process.env.CODEX_API_KEY },
  })

  const result = await sandbox.commands.run(
    `codex exec --full-auto --skip-git-repo-check --json -C /home/user/repo "Refactor the utils module into separate files"`,
    {
      onStdout: (data) => {
        for (const line of data.split('\n').filter(Boolean)) {
          const event = JSON.parse(line)
          console.log(`[${event.type}]`, event)
        }
      },
    }
  )

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  import json
  from e2b import Sandbox

  sandbox = Sandbox.create("codex", envs={
      "CODEX_API_KEY": os.environ["CODEX_API_KEY"],
  })

  def handle_event(data):
      for line in data.strip().split("\n"):
          if line:
              event = json.loads(line)
              print(f"[{event['type']}]", event)

  result = sandbox.commands.run(
      'codex exec --full-auto --skip-git-repo-check --json -C /home/user/repo "Refactor the utils module into separate files"',
      on_stdout=handle_event,
  )

  sandbox.kill()
  ```
</CodeGroup>

## Image input

Pass screenshots or design mockups with `--image` to give Codex visual context alongside the prompt.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import fs from 'fs'
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('codex', {
    envs: { CODEX_API_KEY: process.env.CODEX_API_KEY },
    timeoutMs: 600_000,
  })

  // Upload a design mockup to the sandbox
  await sandbox.files.write(
    '/home/user/mockup.png',
    fs.readFileSync('./mockup.png')
  )

  const result = await sandbox.commands.run(
    `codex exec --full-auto --skip-git-repo-check --image /home/user/mockup.png -C /home/user/repo "Implement this UI design as a React component"`,
    { onStdout: (data) => process.stdout.write(data) }
  )

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("codex", envs={
      "CODEX_API_KEY": os.environ["CODEX_API_KEY"],
  }, timeout=600)

  # Upload a design mockup to the sandbox
  with open("./mockup.png", "rb") as f:
      sandbox.files.write("/home/user/mockup.png", f)

  result = sandbox.commands.run(
      'codex exec --full-auto --skip-git-repo-check --image /home/user/mockup.png -C /home/user/repo "Implement this UI design as a React component"',
      on_stdout=lambda data: print(data, end=""),
  )

  sandbox.kill()
  ```
</CodeGroup>

## Build a custom template

If you need to customize the environment (e.g. pre-install dependencies, add config files), build your own template on top of the pre-built `codex` template.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b'

  export const template = Template()
    .fromTemplate('codex')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_template("codex")
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as codexTemplate } from './template'

  await Template.build(codexTemplate, 'my-codex', {
    cpuCount: 2,
    memoryMB: 2048,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from template import template as codex_template

  Template.build(codex_template, "my-codex",
      cpu_count=2,
      memory_mb=2048,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>

Run the build script to create the template.

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npx tsx build.ts
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  python build.py
  ```
</CodeGroup>

## Related guides

<CardGroup cols={3}>
  <Card title="Sandbox persistence" icon="clock" href="/docs/sandbox/persistence">
    Auto-pause, resume, and manage sandbox lifecycle
  </Card>

  <Card title="Git integration" icon="code-branch" href="/docs/sandbox/git-integration">
    Clone repos, manage branches, and push changes
  </Card>

  <Card title="SSH access" icon="terminal" href="/docs/sandbox/ssh-access">
    Connect to the sandbox via SSH for interactive sessions
  </Card>
</CardGroup>
