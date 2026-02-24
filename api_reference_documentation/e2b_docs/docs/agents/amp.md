> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# AMP

> Run AMP in a secure E2B sandbox with full filesystem, terminal, and git access.

[AMP](https://ampcode.com) is Sourcegraph's coding agent with multi-model architecture and built-in code intelligence. E2B provides a pre-built `amp` template with AMP already installed.

## CLI

Spin up a sandbox with the [E2B CLI](/docs/cli).

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sbx create amp
```

Once inside the sandbox, start AMP.

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
amp
```

## Run headless

Use `-x` for non-interactive mode and `--dangerously-allow-all` to auto-approve all tool calls (safe inside E2B sandboxes). AMP uses its own API key from [ampcode.com/settings](https://ampcode.com/settings).

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('amp', {
    envs: { AMP_API_KEY: process.env.AMP_API_KEY },
  })

  const result = await sandbox.commands.run(
    `amp --dangerously-allow-all -x "Create a hello world HTTP server in Go"`
  )

  console.log(result.stdout)
  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("amp", envs={
      "AMP_API_KEY": os.environ["AMP_API_KEY"],
  })

  result = sandbox.commands.run(
      'amp --dangerously-allow-all -x "Create a hello world HTTP server in Go"',
  )

  print(result.stdout)
  sandbox.kill()
  ```
</CodeGroup>

### Example: work on a cloned repository

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('amp', {
    envs: { AMP_API_KEY: process.env.AMP_API_KEY },
    timeoutMs: 600_000,
  })

  await sandbox.git.clone('https://github.com/your-org/your-repo.git', {
    path: '/home/user/repo',
    username: 'x-access-token',
    password: process.env.GITHUB_TOKEN,
    depth: 1,
  })

  const result = await sandbox.commands.run(
    `cd /home/user/repo && amp --dangerously-allow-all -x "Add error handling to all API endpoints"`,
    { onStdout: (data) => process.stdout.write(data) }
  )

  const diff = await sandbox.commands.run('cd /home/user/repo && git diff')
  console.log(diff.stdout)

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("amp", envs={
      "AMP_API_KEY": os.environ["AMP_API_KEY"],
  }, timeout=600)

  sandbox.git.clone("https://github.com/your-org/your-repo.git",
      path="/home/user/repo",
      username="x-access-token",
      password=os.environ["GITHUB_TOKEN"],
      depth=1,
  )

  result = sandbox.commands.run(
      'cd /home/user/repo && amp --dangerously-allow-all -x "Add error handling to all API endpoints"',
      on_stdout=lambda data: print(data, end=""),
  )

  diff = sandbox.commands.run("cd /home/user/repo && git diff")
  print(diff.stdout)

  sandbox.kill()
  ```
</CodeGroup>

## Streaming JSON

Use `--stream-json` to get a real-time JSONL event stream with rich metadata — including tool calls, token usage, thinking blocks, and permission decisions.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('amp', {
    envs: { AMP_API_KEY: process.env.AMP_API_KEY },
  })

  const result = await sandbox.commands.run(
    `cd /home/user/repo && amp --dangerously-allow-all --stream-json -x "Find and fix all TODO comments"`,
    {
      onStdout: (data) => {
        for (const line of data.split('\n').filter(Boolean)) {
          const event = JSON.parse(line)
          if (event.type === 'assistant') {
            console.log(`[assistant] tokens: ${event.message.usage?.output_tokens}`)
          } else if (event.type === 'result') {
            console.log(`[done] ${event.message.subtype} in ${event.message.duration_ms}ms`)
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

  sandbox = Sandbox.create("amp", envs={
      "AMP_API_KEY": os.environ["AMP_API_KEY"],
  })

  def handle_event(data):
      for line in data.strip().split("\n"):
          if line:
              event = json.loads(line)
              if event["type"] == "assistant":
                  usage = event.get("message", {}).get("usage", {})
                  print(f"[assistant] tokens: {usage.get('output_tokens')}")
              elif event["type"] == "result":
                  msg = event["message"]
                  print(f"[done] {msg['subtype']} in {msg['duration_ms']}ms")

  result = sandbox.commands.run(
      'cd /home/user/repo && amp --dangerously-allow-all --stream-json -x "Find and fix all TODO comments"',
      on_stdout=handle_event,
  )

  sandbox.kill()
  ```
</CodeGroup>

## Thread management

AMP persists conversations as threads that can be resumed or continued with follow-up tasks.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('amp', {
    envs: { AMP_API_KEY: process.env.AMP_API_KEY },
    timeoutMs: 600_000,
  })

  // Start a new thread
  const initial = await sandbox.commands.run(
    `cd /home/user/repo && amp --dangerously-allow-all -x "Analyze the codebase and create a refactoring plan"`,
    { onStdout: (data) => process.stdout.write(data) }
  )

  // List threads and get the most recent thread ID
  const threads = await sandbox.commands.run('amp threads list --json')
  const threadId = JSON.parse(threads.stdout)[0].id

  // Continue the thread with a follow-up task
  const followUp = await sandbox.commands.run(
    `cd /home/user/repo && amp threads continue ${threadId} --dangerously-allow-all -x "Now implement step 1 of the plan"`,
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

  sandbox = Sandbox.create("amp", envs={
      "AMP_API_KEY": os.environ["AMP_API_KEY"],
  }, timeout=600)

  # Start a new thread
  initial = sandbox.commands.run(
      'cd /home/user/repo && amp --dangerously-allow-all -x "Analyze the codebase and create a refactoring plan"',
      on_stdout=lambda data: print(data, end=""),
  )

  # List threads and get the most recent thread ID
  threads = sandbox.commands.run("amp threads list --json")
  thread_id = json.loads(threads.stdout)[0]["id"]

  # Continue the thread with a follow-up task
  follow_up = sandbox.commands.run(
      f'cd /home/user/repo && amp threads continue {thread_id} --dangerously-allow-all -x "Now implement step 1 of the plan"',
      on_stdout=lambda data: print(data, end=""),
  )

  diff = sandbox.commands.run("cd /home/user/repo && git diff")
  print(diff.stdout)

  sandbox.kill()
  ```
</CodeGroup>

## Build a custom template

If you need to customize the environment (e.g. pre-install dependencies, add config files), build your own template on top of the pre-built `amp` template.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b'

  export const template = Template()
    .fromTemplate('amp')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_template("amp")
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as ampTemplate } from './template'

  await Template.build(ampTemplate, 'my-amp', {
    cpuCount: 2,
    memoryMB: 2048,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from template import template as amp_template

  Template.build(amp_template, "my-amp",
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
