> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Quickstart

> Get started with MCP integration

You can connect to the MCPs running inside the sandbox both from outside and inside the sandbox.

## From outside the sandbox

To connect to the MCPs running inside the sandbox, use the `sandbox.getMcpUrl()` in JavaScript and `sandbox.get_mcp_url()` in Python.

<CodeGroup>
  ```typescript TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import Sandbox from 'e2b'
  import { Client } from '@modelcontextprotocol/sdk/client/index.js';
  import { StreamableHTTPClientTransport } from '@modelcontextprotocol/sdk/client/streamableHttp.js';

  const sandbox = await Sandbox.create({
      mcp: {
          browserbase: {
              apiKey: process.env.BROWSERBASE_API_KEY!,
              geminiApiKey: process.env.GEMINI_API_KEY!,
              projectId: process.env.BROWSERBASE_PROJECT_ID!,
          },
          exa: {
              apiKey: process.env.EXA_API_KEY!,
          },
          notion: {
              internalIntegrationToken: process.env.NOTION_API_KEY!,
          },
      },
  });

  const client = new Client({
      name: 'e2b-mcp-client',
      version: '1.0.0'
  });

  const transport = new StreamableHTTPClientTransport(
      new URL(sandbox.getMcpUrl()), 
      {
          requestInit: {
              headers: {
                  'Authorization': `Bearer ${await sandbox.getMcpToken()}`
              }
          }
      }
  );

  await client.connect(transport);

  const tools = await client.listTools();
  console.log('Available tools:', tools.tools.map(t => t.name));

  await client.close();
  await sandbox.kill();
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os
  import dotenv
  dotenv.load_dotenv()
  from e2b import AsyncSandbox
  import asyncio
  from datetime import timedelta
  from mcp.client.session import ClientSession
  from mcp.client.streamable_http import streamablehttp_client

  async def main():
      sandbox = await AsyncSandbox.create(
          mcp={            
              "browserbase": {
                  "apiKey": os.environ["BROWSERBASE_API_KEY"],
                  "geminiApiKey": os.environ["GEMINI_API_KEY"],
                  "projectId": os.environ["BROWSERBASE_PROJECT_ID"],
              },
              "exa": {
                  "apiKey": os.environ["EXA_API_KEY"],
              },
              "notion": {
                  "internalIntegrationToken": os.environ["NOTION_API_KEY"],
              },
          }
      )

      async with streamablehttp_client(
          url=sandbox.get_mcp_url(),
          headers={"Authorization": f"Bearer {await sandbox.get_mcp_token()}"},
          timeout=timedelta(seconds=600)
      ) as (read_stream, write_stream, _):
          async with ClientSession(read_stream, write_stream) as session:
              await session.initialize()
              tools = await session.list_tools()
              print(f"Available tools: {[tool.name for tool in tools.tools]}")
      await sandbox.kill()

  if __name__ == "__main__":
      asyncio.run(main())
  ```
</CodeGroup>

## From inside the sandbox

If you need to access the MCP gateway from within the sandbox itself, it's available at:

```
http://localhost:50005/mcp
```

You'll need to include the Authorization header with the MCP token when making requests from inside the sandbox. How that is added depends on the MCP client you use:

## Claude

```
claude mcp add --transport http e2b-mcp-gateway <url goes here> --header "Authorization: Bearer <token goes here>"
```

## OpenAI Agents

<CodeGroup>
  ```typescript TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { MCPServerStreamableHttp } from '@openai/agents';

  const mcp = new MCPServerStreamableHttp({
      url: mcpUrl,
      name: 'E2B MCP Gateway',
      requestInit: {
          headers: {
              'Authorization': `Bearer ${await sandbox.getMcpToken()}`
          }
      },
  });
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import asyncio
  import os
  from e2b import AsyncSandbox
  from agents.mcp import MCPServerStreamableHttp

  async def main():
      async with MCPServerStreamableHttp(
          name="e2b-mcp-client",
          params={
              "url": sandbox.get_mcp_url(),
              "headers": {"Authorization": f"Bearer {await sandbox.get_mcp_token()}"},
          },
      ) as server:
          tools = await server.list_tools()
          print("Available tools:", [t.name for t in tools])

      # Clean up
      await sandbox.kill()

  asyncio.run(main())
  ```
</CodeGroup>

## Official MCP client

<CodeGroup>
  ```typescript Typescript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Client } from '@modelcontextprotocol/sdk/client/index.js';
  import { StreamableHTTPClientTransport } from '@modelcontextprotocol/sdk/client/streamableHttp.js';

  const client = new Client({
      name: 'e2b-mcp-client',
      version: '1.0.0'
  });

  const transport = new StreamableHTTPClientTransport(
      new URL(sandbox.getMcpUrl()), 
      {
          requestInit: {
              headers: {
                  'Authorization': `Bearer ${await sandbox.getMcpToken()}`
              }
          }
      }
  );
  await client.connect(transport);
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import asyncio
  from datetime import timedelta
  from mcp.client.session import ClientSession
  from mcp.client.streamable_http import streamablehttp_client

  async def main():
      async with streamablehttp_client(
          url=sandbox.get_mcp_url(),
          headers={"Authorization": f"Bearer {await sandbox.get_mcp_token()}"},
          timeout=timedelta(seconds=600)
      ) as (read_stream, write_stream, _):
          async with ClientSession(read_stream, write_stream) as session:
              await session.initialize()
              
              tools = await session.list_tools()
              print(f"Available tools: {[tool.name for tool in tools.tools]}")
      await sandbox.kill()

  if __name__ == "__main__":
      asyncio.run(main())

  ```
</CodeGroup>

This list is not exhaustive. You can find more examples in the [E2B Cookbook](https://github.com/e2b-dev/e2b-cookbook).

## Debugging with MCP Inspector

The [MCP Inspector](https://github.com/modelcontextprotocol/inspector) is a useful tool for debugging and testing your MCP server setup. Get the command to run:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
npx @modelcontextprotocol/inspector --transport http --url <your-mcp-url> --header "Authorization: Bearer ${mcpToken}"
```

Run the command in your terminal. This will open a web interface where you can:

* Browse available tools
* Test tool calls with different parameters
* Inspect request/response payloads
* Debug connection issues
