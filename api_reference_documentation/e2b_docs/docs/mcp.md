> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Overview

> Connect to 200+ tools through the Model Context Protocol

E2B provides a batteries-included MCP gateway that runs inside sandboxes, giving you type-safe access to 200+ MCP tools from the [Docker MCP Catalog](https://hub.docker.com/mcp) or [custom MCPs](/docs/mcp/custom-servers) through a unified interface. This integration gives developers instant access to tools like [Browserbase](https://www.browserbase.com/), [Exa](https://exa.ai/), [Notion](https://www.notion.so/), [Stripe](https://stripe.com/), or [GitHub](https://github.com/).

The [Model Context Protocol (MCP)](https://modelcontextprotocol.io/docs/getting-started/intro) is an open standard for connecting AI models to external tools and data sources. E2B sandboxes provide an ideal environment for running MCP tools, giving AI full access to an internet-connected Linux machine where it can safely install packages, write files, run terminal commands, and AI-generated code.

<Frame>
  <img src="https://mintcdn.com/e2b/JsolbCWO7aAeiUpW/images/mcp-overview.png?fit=max&auto=format&n=JsolbCWO7aAeiUpW&q=85&s=2cc8cc672309e864c0d15aa2b2af843c" data-og-width="2156" width="2156" data-og-height="1434" height="1434" data-path="images/mcp-overview.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/JsolbCWO7aAeiUpW/images/mcp-overview.png?w=280&fit=max&auto=format&n=JsolbCWO7aAeiUpW&q=85&s=522469544face8549ed6b749e50a3149 280w, https://mintcdn.com/e2b/JsolbCWO7aAeiUpW/images/mcp-overview.png?w=560&fit=max&auto=format&n=JsolbCWO7aAeiUpW&q=85&s=442a5ad71e19e1c22718061997813572 560w, https://mintcdn.com/e2b/JsolbCWO7aAeiUpW/images/mcp-overview.png?w=840&fit=max&auto=format&n=JsolbCWO7aAeiUpW&q=85&s=b84f5a289341ec37e870ec3d1fce9dfa 840w, https://mintcdn.com/e2b/JsolbCWO7aAeiUpW/images/mcp-overview.png?w=1100&fit=max&auto=format&n=JsolbCWO7aAeiUpW&q=85&s=0243c1ede7650625cf2f74180addbe30 1100w, https://mintcdn.com/e2b/JsolbCWO7aAeiUpW/images/mcp-overview.png?w=1650&fit=max&auto=format&n=JsolbCWO7aAeiUpW&q=85&s=c2c35eba5e41ba7f5c5d4d768f7770e0 1650w, https://mintcdn.com/e2b/JsolbCWO7aAeiUpW/images/mcp-overview.png?w=2500&fit=max&auto=format&n=JsolbCWO7aAeiUpW&q=85&s=7219dd88ea643e98069149b149a873ad 2500w" />
</Frame>

<CodeGroup>
  ```typescript TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import Sandbox from 'e2b'

  const sbx = await Sandbox.create({
      mcp: {
          browserbase: {
              apiKey: process.env.BROWSERBASE_API_KEY!,
              geminiApiKey: process.env.GEMINI_API_KEY!,
              projectId: process.env.BROWSERBASE_PROJECT_ID!,
          },
          exa: {
              apiKey: process.env.EXA_API_KEY!,
          },
          airtable: {
              airtableApiKey: process.env.AIRTABLE_API_KEY!,
          },
      },
  });

  const mcpUrl = sbx.getMcpUrl();
  const mcpToken = await sbx.getMcpToken();

  // You can now connect the gateway to any MCP client, for example claude:
  // This also works for your local claude!
  await sbx.commands.run(`claude mcp add --transport http e2b-mcp-gateway ${mcpUrl} --header "Authorization: Bearer ${mcpToken}"`, { timeoutMs: 0, onStdout: console.log, onStderr: console.log });

  await sbx.commands.run(
      `echo 'Use browserbase and exa to research open positions at e2b.dev. Collect your findings in Airtable.' | claude -p --dangerously-skip-permissions`,
      { timeoutMs: 0, onStdout: console.log, onStderr: console.log }
  )

  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import asyncio
  from e2b import AsyncSandbox
  import os
  import dotenv

  dotenv.load_dotenv()

  async def main():
      sbx = await AsyncSandbox.create(mcp={
          "browserbase": {
              "apiKey": os.getenv("BROWSERBASE_API_KEY"),
              "geminiApiKey": os.getenv("GEMINI_API_KEY"),
              "projectId": os.getenv("BROWSERBASE_PROJECT_ID"),
          },
          "exa": {
              "apiKey": os.getenv("EXA_API_KEY"),
          },
          "airtable": {
              "airtableApiKey": os.getenv("AIRTABLE_API_KEY"),
          },
      })

      mcp_url = sbx.get_mcp_url()
      mcp_token = await sbx.get_mcp_token()

      # You can now connect the gateway to any MCP client, for example claude:
      # This also works for your local claude!
      await sbx.commands.run(f'claude mcp add --transport http e2b-mcp-gateway {mcp_url} --header "Authorization: Bearer {mcp_token}"', timeout=0, on_stdout=print, on_stderr=print)

      await sbx.commands.run(
          "echo 'Use browserbase and exa to research open positions at e2b.dev. Collect your findings in Airtable.' | claude -p --dangerously-skip-permissions",
          timeout=0, on_stdout=print, on_stderr=print
      )

  if __name__ == "__main__":
      asyncio.run(main())
  ```
</CodeGroup>

## Documentation

<CardGroup cols={2}>
  <Card title="Quickstart" icon="rocket" href="/docs/mcp/quickstart">
    Get started with MCP
  </Card>

  <Card title="Available servers" icon="server" href="/docs/mcp/available-servers">
    Browse 200+ pre-built MCP servers
  </Card>

  <Card title="Custom templates" icon="cube" href="/docs/mcp/custom-templates">
    Prepull MCP servers for faster runtime
  </Card>

  <Card title="Custom servers" icon="github" href="/docs/mcp/custom-servers">
    Use custom MCP servers from GitHub
  </Card>

  <Card title="Examples" icon="code" href="/docs/mcp/examples">
    See examples
  </Card>
</CardGroup>
