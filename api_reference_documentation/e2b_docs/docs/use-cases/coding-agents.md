> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Coding Agents

> Run AI coding agents like Claude Code, Codex, and AMP in secure E2B sandboxes with full terminal, filesystem, and git access.

Coding agents like [Claude Code](https://docs.anthropic.com/en/docs/agents-and-tools/claude-code/overview), [Codex](https://github.com/openai/codex), and [AMP](https://ampcode.com/) can write, debug, and refactor code autonomously. E2B sandboxes give each agent a full Linux environment with terminal, filesystem, and git — completely isolated from your infrastructure. Pre-built templates mean you can go from zero to a running agent in a single API call.

## Why Use a Sandbox

Running coding agents directly on your machine or servers means giving AI-generated code unrestricted access to your environment. E2B sandboxes solve this:

1. **Isolation** — agent-generated code runs in a secure sandbox, never touching your production systems or local machine
2. **Full dev environment** — terminal, filesystem, git, and package managers are all available out of the box, so agents work like a developer would
3. **Pre-built templates** — ready-made templates for popular agents get you started fast, and you can [build your own](/docs/template/quickstart) for any agent
4. **Scalability** — spin up many sandboxes in parallel, each running its own agent on a separate task

## How It Works

1. **Create a sandbox** — use a pre-built template or [build your own](/docs/template/quickstart) with any agent installed
2. **Agent gets a full environment** — terminal, filesystem, git access, and any tools installed in the template
3. **Agent works autonomously** — it reads the codebase, writes code, runs tests, and iterates until the task is done
4. **Extract results** — pull out the git diff, structured output, or modified files via the SDK
5. **Sandbox is cleaned up** — once the work is done, the sandbox is destroyed automatically. No lingering state or cleanup needed

## Agent Examples

Since each sandbox is a full Linux environment, you can run any coding agent — just install it in a [custom template](/docs/template/quickstart). E2B also provides pre-built templates for popular agents to get you started quickly.

<CardGroup cols={2}>
  <Card title="Claude Code" icon="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/claude-code.svg?fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=fe8bde030df28db002845135fc456500" href="/docs/agents/claude-code" data-og-width="248" width="248" data-og-height="248" height="248" data-path="images/icons/claude-code.svg" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/claude-code.svg?w=280&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=eff93a1d1ef718ae7b48a17886b8eec3 280w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/claude-code.svg?w=560&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=899efb4fcdab1198f6c1b4f96656b704 560w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/claude-code.svg?w=840&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=6ffc7ae6d3741e6e5b692510eb60dc6b 840w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/claude-code.svg?w=1100&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=2f8394f5f2f230733ab1e3ac439fb544 1100w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/claude-code.svg?w=1650&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=b40eedf0aebf50881827f6003806e938 1650w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/claude-code.svg?w=2500&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=a5db3dad91b5c89f56846f7a47dbbb8e 2500w">
    Anthropic's autonomous coding agent with structured output and MCP tool support
  </Card>

  <Card title="Codex" icon="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/codex.svg?fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=4d75b0e791c6bdd4b910a416ca9671c0" href="/docs/agents/codex" data-og-width="512" width="512" data-og-height="512" height="512" data-path="images/icons/codex.svg" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/codex.svg?w=280&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=ed0adc78c03d3a25f07270f6dcb5dec2 280w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/codex.svg?w=560&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=da2e0fa322f8216f76b82cc2839c2a96 560w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/codex.svg?w=840&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=57f617d3f4b173f005586d342d183c0e 840w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/codex.svg?w=1100&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=d942c94f1a141699224880959aabf308 1100w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/codex.svg?w=1650&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=59f1abad459b298cd6b17292adeea7ac 1650w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/codex.svg?w=2500&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=802ad210172fade233c172827b022a62 2500w">
    OpenAI's coding agent with schema-validated output and image input
  </Card>

  <Card title="AMP" icon="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/amp.svg?fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=f67931c58e282faa7b8b218cc187b5bb" href="/docs/agents/amp" data-og-width="21" width="21" data-og-height="21" height="21" data-path="images/icons/amp.svg" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/amp.svg?w=280&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=9b12a6c4e32f5c5de2e1f580c0b1211e 280w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/amp.svg?w=560&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=aa0b21653b20aebc4c73da21283681c1 560w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/amp.svg?w=840&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=c122e0539c73b567233339bf79862b4d 840w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/amp.svg?w=1100&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=8cd856cf616f0e3f952012a5359d7987 1100w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/amp.svg?w=1650&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=71c234549340adbe5e09e16c542d322c 1650w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/amp.svg?w=2500&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=75724e825eee5b478d995333630f2755 2500w">
    Sourcegraph's coding agent with streaming JSON and thread management
  </Card>

  <Card title="OpenCode" icon="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/opencode.svg?fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=d58064bcc6e8390e8124bafcf1e8c60e" href="/docs/agents/opencode" data-og-width="240" width="240" data-og-height="300" height="300" data-path="images/icons/opencode.svg" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/opencode.svg?w=280&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=9e55823fd8511c75bd8c8ebc3e7ad834 280w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/opencode.svg?w=560&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=04fd532c1871f7914e6bc67a185a2d86 560w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/opencode.svg?w=840&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=bb55d162cc688f64a06822b23dc203b0 840w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/opencode.svg?w=1100&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=479f3bc533df06d791dee3c322567fa4 1100w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/opencode.svg?w=1650&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=378530749ba2a6388d84b4ba6b8fa561 1650w, https://mintcdn.com/e2b/PQElRi7ukfkEUITb/images/icons/opencode.svg?w=2500&fit=max&auto=format&n=PQElRi7ukfkEUITb&q=85&s=24dca43f0fbc860054a2903ff7223c7c 2500w">
    Open-source multi-provider agent with a built-in web UI
  </Card>
</CardGroup>

## Related Guides

<CardGroup cols={3}>
  <Card title="Git Integration" icon="code-branch" href="/docs/sandbox/git-integration">
    Clone repos, manage branches, and push changes from sandboxes
  </Card>

  <Card title="Sandbox Persistence" icon="floppy-disk" href="/docs/sandbox/persistence">
    Pause and resume sandboxes to preserve state across sessions
  </Card>

  <Card title="Custom Templates" icon="cube" href="/docs/template/quickstart">
    Build your own sandbox templates with custom tools and dependencies
  </Card>
</CardGroup>
