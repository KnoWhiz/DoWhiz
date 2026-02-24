> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code

> Claude Code Agent available in a sandbox

<Info>
  For a complete guide on running Claude Code in E2B sandboxes — including working with repositories, streaming output, and connecting MCP tools — see the [Agents in Sandbox: Claude Code](/docs/agents/claude-code) guide.
</Info>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b'

  export const template = Template()
    .fromNodeImage('24')
    .aptInstall(['curl', 'git', 'ripgrep'])
    // Claude Code will be available globally as "claude"
    .npmInstall('@anthropic-ai/claude-code@latest', { g: true })

  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_node_image("24")
      .apt_install(["curl", "git", "ripgrep"])
      # Claude Code will be available globally as "claude"
      .npm_install("@anthropic-ai/claude-code@latest", g=True)
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as claudeCodeTemplate } from './template'

  Template.build(claudeCodeTemplate, 'claude-code', {
    cpuCount: 1,
    memoryMB: 1024,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from .template import template as claudeCodeTemplate

  Template.build(claudeCodeTemplate, 'claude-code',
      cpu_count=1,
      memory_mb=1024,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // sandbox.ts
  import { Sandbox } from 'e2b'

  const sbx = await Sandbox.create('claude-code', {
    envs: {
      ANTHROPIC_API_KEY: '<your api key>',
    },
  })

  console.log('Sandbox created', sbx.sandboxId)

  // Print help for Claude Code
  // const result = await sbx.commands.run('claude --help')
  // console.log(result.stdout)

  // Run a prompt with Claude Code
  const result = await sbx.commands.run(
    `claude --dangerously-skip-permissions -p 'Create a hello world index.html'`,
    { timeoutMs: 0 }
  )

  console.log(result.stdout)

  sbx.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # sandbox.py
  from e2b import Sandbox

  sbx = Sandbox(
      'claude-code',
      envs={
          'ANTHROPIC_API_KEY': '<your api key>',
      },
  )
  print("Sandbox created", sbx.sandbox_id)

  # Print help for Claude Code
  # result = sbx.commands.run('claude --help')
  # print(result.stdout)

  # Run a prompt with Claude Code
  result = sbx.commands.run(
      "claude --dangerously-skip-permissions -p 'Create a hello world index.html'",
      timeout=0,
  )
  print(result.stdout)

  sbx.kill()
  ```
</CodeGroup>
