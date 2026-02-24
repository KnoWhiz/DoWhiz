> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code

> Run Claude Code in a secure E2B sandbox with full filesystem, terminal, and git access.

[Claude Code](https://docs.anthropic.com/en/docs/claude-code) is Anthropic's agentic coding tool. E2B provides a pre-built `claude` template with Claude Code already installed.

## CLI

Spin up a sandbox with the [E2B CLI](/docs/cli).

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sbx create claude
```

Once inside the sandbox, start Claude Code.

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
claude
```

## Run headless

Use `-p` for non-interactive mode and `--dangerously-skip-permissions` to auto-approve all tool calls (safe inside E2B sandboxes).

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('claude', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
  })

  const result = await sandbox.commands.run(
    `claude --dangerously-skip-permissions -p "Create a hello world HTTP server in Go"`
  )

  console.log(result.stdout)
  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("claude", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  })

  result = sandbox.commands.run(
      'claude --dangerously-skip-permissions -p "Create a hello world HTTP server in Go"',
  )

  print(result.stdout)
  sandbox.kill()
  ```
</CodeGroup>

### Example: work on a cloned repository

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('claude', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
    timeoutMs: 600_000,
  })

  await sandbox.git.clone('https://github.com/your-org/your-repo.git', {
    path: '/home/user/repo',
    username: 'x-access-token',
    password: process.env.GITHUB_TOKEN,
    depth: 1,
  })

  const result = await sandbox.commands.run(
    `cd /home/user/repo && claude --dangerously-skip-permissions -p "Add error handling to all API endpoints"`,
    { onStdout: (data) => process.stdout.write(data) }
  )

  const diff = await sandbox.commands.run('cd /home/user/repo && git diff')
  console.log(diff.stdout)

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("claude", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  }, timeout=600)

  sandbox.git.clone("https://github.com/your-org/your-repo.git",
      path="/home/user/repo",
      username="x-access-token",
      password=os.environ["GITHUB_TOKEN"],
      depth=1,
  )

  result = sandbox.commands.run(
      'cd /home/user/repo && claude --dangerously-skip-permissions -p "Add error handling to all API endpoints"',
      on_stdout=lambda data: print(data, end=""),
  )

  diff = sandbox.commands.run("cd /home/user/repo && git diff")
  print(diff.stdout)

  sandbox.kill()
  ```
</CodeGroup>

## Structured output

Use `--output-format json` to get machine-readable responses — useful for building pipelines or extracting specific results.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('claude', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
  })

  const result = await sandbox.commands.run(
    `claude --dangerously-skip-permissions --output-format json -p "Review this codebase and list all security issues as JSON"`
  )

  const response = JSON.parse(result.stdout)
  console.log(response)

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  import json
  from e2b import Sandbox

  sandbox = Sandbox.create("claude", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  })

  result = sandbox.commands.run(
      'claude --dangerously-skip-permissions --output-format json -p "Review this codebase and list all security issues as JSON"',
  )

  response = json.loads(result.stdout)
  print(response)

  sandbox.kill()
  ```
</CodeGroup>

## Streaming output

Use `--output-format stream-json` to get a real-time JSONL event stream — including tool calls, token usage, and result metadata.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('claude', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
  })

  const result = await sandbox.commands.run(
    `cd /home/user/repo && claude --dangerously-skip-permissions --output-format stream-json -p "Find and fix all TODO comments"`,
    {
      onStdout: (data) => {
        for (const line of data.split('\n').filter(Boolean)) {
          const event = JSON.parse(line)
          if (event.type === 'assistant') {
            console.log(`[assistant] tokens: ${event.message.usage?.output_tokens}`)
          } else if (event.type === 'result') {
            console.log(`[done] ${event.subtype} in ${event.duration_ms}ms`)
          }
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

  sandbox = Sandbox.create("claude", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  })

  def handle_event(data):
      for line in data.strip().split("\n"):
          if line:
              event = json.loads(line)
              if event["type"] == "assistant":
                  usage = event.get("message", {}).get("usage", {})
                  print(f"[assistant] tokens: {usage.get('output_tokens')}")
              elif event["type"] == "result":
                  print(f"[done] {event['subtype']} in {event['duration_ms']}ms")

  result = sandbox.commands.run(
      'cd /home/user/repo && claude --dangerously-skip-permissions --output-format stream-json -p "Find and fix all TODO comments"',
      on_stdout=handle_event,
  )

  sandbox.kill()
  ```
</CodeGroup>

## Resume a session

Claude Code persists conversations that can be resumed with follow-up tasks using `--session-id`.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('claude', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
    timeoutMs: 600_000,
  })

  // Start a new session
  const initial = await sandbox.commands.run(
    `cd /home/user/repo && claude --dangerously-skip-permissions --output-format json -p "Analyze the codebase and create a refactoring plan"`
  )

  // Extract session ID from the JSON response
  const response = JSON.parse(initial.stdout)
  const sessionId = response.session_id

  // Continue with a follow-up task
  const followUp = await sandbox.commands.run(
    `cd /home/user/repo && claude --dangerously-skip-permissions --session-id ${sessionId} -p "Now implement step 1 of the plan"`,
    { onStdout: (data) => process.stdout.write(data) }
  )

  const diff = await sandbox.commands.run('cd /home/user/repo && git diff')
  console.log(diff.stdout)

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  import json
  from e2b import Sandbox

  sandbox = Sandbox.create("claude", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  }, timeout=600)

  # Start a new session
  initial = sandbox.commands.run(
      'cd /home/user/repo && claude --dangerously-skip-permissions --output-format json -p "Analyze the codebase and create a refactoring plan"',
  )

  # Extract session ID from the JSON response
  response = json.loads(initial.stdout)
  session_id = response["session_id"]

  # Continue with a follow-up task
  follow_up = sandbox.commands.run(
      f'cd /home/user/repo && claude --dangerously-skip-permissions --session-id {session_id} -p "Now implement step 1 of the plan"',
      on_stdout=lambda data: print(data, end=""),
  )

  diff = sandbox.commands.run("cd /home/user/repo && git diff")
  print(diff.stdout)

  sandbox.kill()
  ```
</CodeGroup>

## Custom system prompt

Write a `CLAUDE.md` file into the sandbox for project context or use `--system-prompt` to provide task-specific instructions.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('claude', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
  })

  // Write project context
  await sandbox.files.write('/home/user/repo/CLAUDE.md', `
  You are working on a Go microservice.
  Always use structured logging with slog.
  Follow the project's error handling conventions in pkg/errors.
  `)

  const result = await sandbox.commands.run(
    `cd /home/user/repo && claude --dangerously-skip-permissions -p "Add a /healthz endpoint"`
  )

  console.log(result.stdout)
  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("claude", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  })

  # Write project context
  sandbox.files.write("/home/user/repo/CLAUDE.md", """
  You are working on a Go microservice.
  Always use structured logging with slog.
  Follow the project's error handling conventions in pkg/errors.
  """)

  result = sandbox.commands.run(
      'cd /home/user/repo && claude --dangerously-skip-permissions -p "Add a /healthz endpoint"',
  )

  print(result.stdout)
  sandbox.kill()
  ```
</CodeGroup>

## Connect MCP tools

Claude Code has built-in support for [MCP](https://modelcontextprotocol.io/). E2B provides an [MCP gateway](/docs/mcp) that gives Claude access to 200+ tools from the [Docker MCP Catalog](https://hub.docker.com/mcp).

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('claude', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
    mcp: {
      browserbase: {
        apiKey: process.env.BROWSERBASE_API_KEY,
        projectId: process.env.BROWSERBASE_PROJECT_ID,
      },
    },
  })

  const mcpUrl = sandbox.getMcpUrl()
  const mcpToken = await sandbox.getMcpToken()

  await sandbox.commands.run(
    `claude mcp add --transport http e2b-mcp-gateway ${mcpUrl} --header "Authorization: Bearer ${mcpToken}"`
  )

  const result = await sandbox.commands.run(
    `claude --dangerously-skip-permissions -p "Use browserbase to research E2B and summarize your findings"`,
    { onStdout: console.log }
  )

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("claude", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  }, mcp={
      "browserbase": {
          "apiKey": os.environ["BROWSERBASE_API_KEY"],
          "projectId": os.environ["BROWSERBASE_PROJECT_ID"],
      },
  })

  mcp_url = sandbox.get_mcp_url()
  mcp_token = sandbox.get_mcp_token()

  sandbox.commands.run(
      f'claude mcp add --transport http e2b-mcp-gateway {mcp_url} --header "Authorization: Bearer {mcp_token}"',
  )

  result = sandbox.commands.run(
      'claude --dangerously-skip-permissions -p "Use browserbase to research E2B and summarize your findings"',
      on_stdout=lambda data: print(data, end=""),
  )

  sandbox.kill()
  ```
</CodeGroup>

## Build a custom template

If you need to customize the environment (e.g. pre-install dependencies, add config files), build your own template on top of the pre-built `claude` template.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b'

  export const template = Template()
    .fromTemplate('claude')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_template("claude")
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as claudeCodeTemplate } from './template'

  await Template.build(claudeCodeTemplate, 'my-claude', {
    cpuCount: 2,
    memoryMB: 2048,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from template import template as claude_code_template

  Template.build(claude_code_template, "my-claude",
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
  <Card title="MCP tools" icon="plug" href="/docs/mcp">
    Connect Claude Code to 200+ MCP tools
  </Card>

  <Card title="Sandbox persistence" icon="clock" href="/docs/sandbox/persistence">
    Auto-pause, resume, and manage sandbox lifecycle
  </Card>

  <Card title="Git integration" icon="code-branch" href="/docs/sandbox/git-integration">
    Clone repos, manage branches, and push changes
  </Card>
</CardGroup>
