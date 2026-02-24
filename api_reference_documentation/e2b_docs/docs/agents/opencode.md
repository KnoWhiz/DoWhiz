> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# OpenCode

> Run OpenCode in a secure E2B sandbox with full filesystem, terminal, and git access.

[OpenCode](https://opencode.ai) is an open-source coding agent that supports multiple LLM providers. E2B provides a pre-built `opencode` template with OpenCode already installed.

## CLI

Spin up a sandbox with the [E2B CLI](/docs/cli).

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
e2b sbx create opencode
```

Once inside the sandbox, start OpenCode.

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
opencode
```

## Run headless

Use `opencode run` for non-interactive mode. Pass your LLM provider's API key as an environment variable — OpenCode supports `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `GEMINI_API_KEY`, and [others](https://opencode.ai/docs/config/).

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('opencode', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
  })

  const result = await sandbox.commands.run(
    `opencode run "Create a hello world HTTP server in Go"`
  )

  console.log(result.stdout)
  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("opencode", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  })

  result = sandbox.commands.run(
      'opencode run "Create a hello world HTTP server in Go"',
  )

  print(result.stdout)
  sandbox.kill()
  ```
</CodeGroup>

### Example: work on a cloned repository

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create('opencode', {
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
    `cd /home/user/repo && opencode run "Add error handling to all API endpoints"`,
    { onStdout: (data) => process.stdout.write(data) }
  )

  const diff = await sandbox.commands.run('cd /home/user/repo && git diff')
  console.log(diff.stdout)

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  from e2b import Sandbox

  sandbox = Sandbox.create("opencode", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  }, timeout=600)

  sandbox.git.clone("https://github.com/your-org/your-repo.git",
      path="/home/user/repo",
      username="x-access-token",
      password=os.environ["GITHUB_TOKEN"],
      depth=1,
  )

  result = sandbox.commands.run(
      'cd /home/user/repo && opencode run "Add error handling to all API endpoints"',
      on_stdout=lambda data: print(data, end=""),
  )

  diff = sandbox.commands.run("cd /home/user/repo && git diff")
  print(diff.stdout)

  sandbox.kill()
  ```
</CodeGroup>

## Connect with the OpenCode SDK

OpenCode includes a [headless HTTP server](https://opencode.ai/docs/server/) that you can control programmatically using the [`@opencode-ai/sdk`](https://opencode.ai/docs/sdk/) client. Start the server inside a sandbox, get the public URL with `sandbox.getHost()`, and connect from your application.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'
  import { createOpencodeClient } from '@opencode-ai/sdk'

  const sandbox = await Sandbox.betaCreate('opencode', {
    envs: { ANTHROPIC_API_KEY: process.env.ANTHROPIC_API_KEY },
    autoPause: true,
    timeoutMs: 10 * 60 * 1000,
  })

  // Start the OpenCode server
  sandbox.commands.run('opencode serve --hostname 0.0.0.0 --port 4096', {
    background: true,
  })

  // Wait for the server to be ready
  const host = sandbox.getHost(4096)
  const baseUrl = `https://${host}`
  while (true) {
    try {
      await fetch(`${baseUrl}/global/health`)
      break
    } catch {
      await new Promise((r) => setTimeout(r, 500))
    }
  }

  // Connect to the server
  const client = createOpencodeClient({
    baseUrl,
  })

  // Create a session and send a prompt
  const { data: session } = await client.session.create({
    body: { title: 'E2B Session' },
  })
  const { data: result } = await client.session.prompt({
    path: { id: session.id },
    body: {
      parts: [{ type: 'text', text: 'Create a hello world HTTP server in Go' }],
    },
  })
  console.log(result)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  import time
  import requests
  from e2b import Sandbox

  sandbox = Sandbox.beta_create("opencode", envs={
      "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
  }, auto_pause=True, timeout=10 * 60)

  # Start the OpenCode server
  sandbox.commands.run(
      "opencode serve --hostname 0.0.0.0 --port 4096",
      background=True,
  )

  # Wait for the server to be ready
  host = sandbox.get_host(4096)
  base_url = f"https://{host}"
  while True:
      try:
          requests.get(f"{base_url}/global/health")
          break
      except requests.ConnectionError:
          time.sleep(0.5)

  # Create a session and send a prompt via the HTTP API
  session = requests.post(f"{base_url}/session").json()
  result = requests.post(
      f"{base_url}/session/{session['id']}/message",
      json={
          "parts": [{"type": "text", "text": "Create a hello world HTTP server in Go"}],
      },
  ).json()
  print(result)
  ```
</CodeGroup>

## Build a custom template

If you need to customize the environment (e.g. pre-install dependencies, add config files), build your own template on top of the pre-built `opencode` template.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template, waitForPort } from 'e2b'

  export const template = Template()
    .fromTemplate('opencode')
    .setEnvs({
      OPENCODE_SERVER_PASSWORD: 'your-password',
    })
    // Optional - start the OpenCode server on sandbox start
    .setStartCmd(
      'opencode serve --hostname 0.0.0.0 --port 4096',
      waitForPort(4096)
    )
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template, wait_for_port

  template = (
      Template()
      .from_template("opencode")
      .set_envs({
          "OPENCODE_SERVER_PASSWORD": "your-password",
      })
      # Optional - start the OpenCode server on sandbox start
      .set_start_cmd(
          "opencode serve --hostname 0.0.0.0 --port 4096",
          wait_for_port(4096)
      )
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as openCodeTemplate } from './template'

  await Template.build(openCodeTemplate, 'my-opencode', {
    cpuCount: 2,
    memoryMB: 2048,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from template import template as opencode_template

  Template.build(opencode_template, "my-opencode",
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
