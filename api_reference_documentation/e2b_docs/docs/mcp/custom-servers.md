> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Custom servers

> Use custom MCP servers from GitHub repositories

In addition to the 200+ pre-built MCP servers from the [Docker MCP Catalog](https://hub.docker.com/mcp), you can run custom MCP servers directly from public GitHub repositories.

## How it works

When you specify a GitHub repository, E2B will:

1. Clone the repository into the sandbox
2. Run the `installCmd` (optional) to install dependencies
3. Run the `runCmd` to start the MCP server with stdio transport

The `runCmd` must start an MCP server that follows the [MCP specification](https://modelcontextprotocol.io/specification/2025-06-18) and communicates via stdio (standard input/output).

## Using a custom MCP server

<CodeGroup>
  ```typescript TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import Sandbox from 'e2b'

  const sandbox = await Sandbox.create({
      mcp: {
          'github/modelcontextprotocol/servers': {
              installCmd: 'npm install',
              runCmd: 'sudo npx -y @modelcontextprotocol/server-filesystem /root',
          },
      },
  });
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Sandbox
  import os

  sbx = Sandbox.create(
      mcp={
          "github/modelcontextprotocol/servers": {
                  "install_cmd": "npm install",
                  "run_cmd": "sudo npx -y @modelcontextprotocol/server-filesystem /root",
          },
      }
  )
  ```
</CodeGroup>

## Configuration

<ParamField path="installCmd" type="string">
  Optional command to run before starting the MCP server. Use this to install dependencies (e.g., `npm install`, `pip install -r requirements.txt`).
</ParamField>

<ParamField path="runCmd" type="string" required>
  Command to start the MCP server. Must launch a stdio-enabled MCP server.
</ParamField>

<Note>
  **Important for npx-based servers:** Always include `installCmd: 'npm install'` (or equivalent) when using `npx` in your `runCmd`. Without installing dependencies first, npx will try to use the local repository and fail.
</Note>

## Troubleshooting

If your custom MCP server doesn't work as expected:

1. Explore the sandbox either via the [dashboard](https://e2b.dev/dashboard) or by connecting to it via `e2b connect <sandbox-id>`
2. Check the gateway log file with `sudo cat /var/log/mcp-gateway/gateway.log`.
