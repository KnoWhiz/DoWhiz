> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Custom templates

> Use MCP servers with custom sandbox templates

You can prepull MCP server Docker images during template build time to significantly improve runtime performance.

## How it works

When you build a template with prepulled MCP servers, the Docker images for those servers are downloaded and cached during the build process. This means when you create a sandbox from that template, the MCP servers are ready to use immediately without waiting for image downloads.

<Info>
  You must use the MCP gateway enabled template (`mcp-gateway`) as your base template to use this feature.
</Info>

## Building a template with MCP servers

Use the `addMcpServer()` method (TypeScript) or `add_mcp_server()` method (Python) to prepull MCP server images during template build. You can pass a single server or an array of servers.

The server names (like `"browserbase"` and `"exa"`) correspond to the keys defined in the [Available Servers](/docs/mcp/available-servers) documentation.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import "dotenv/config";
  import { Template, defaultBuildLogger } from 'e2b';

  export const template = Template()
    .fromTemplate("mcp-gateway")
    .addMcpServer(["browserbase", "exa"]);

  await Template.build(template, 'my-mcp-gateway', {
    cpuCount: 8,
    memoryMB: 8192,
    onBuildLogs: defaultBuildLogger(),
  });
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from dotenv import load_dotenv
  from e2b import Template, default_build_logger

  load_dotenv()

  template = (
      Template()
      .from_template("mcp-gateway")
      .add_mcp_server(["browserbase", "exa"])
  )

  Template.build(
      template,
      'my-mcp-gateway',
      cpu_count=8,
      memory_mb=8192,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>

## Using the template

Once built, create sandboxes from your template. You still need to provide the configuration for each MCP server.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b';

  const sandbox = await Sandbox.create({
      template: "my-mcp-gateway",
      mcp: {
          browserbase: {
              apiKey: process.env.BROWSERBASE_API_KEY!,
              geminiApiKey: process.env.GEMINI_API_KEY!,
              projectId: process.env.BROWSERBASE_PROJECT_ID!,
          },
          exa: {
              apiKey: process.env.EXA_API_KEY!,
          },
      },
  });

  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Sandbox
  import os

  sbx = Sandbox.create(
      template="my-mcp-gateway",
      mcp={
          "browserbase": {
              "apiKey": os.getenv("BROWSERBASE_API_KEY"),
              "geminiApiKey": os.getenv("GEMINI_API_KEY"),
              "projectId": os.getenv("BROWSERBASE_PROJECT_ID"),
          },
          "exa": {
              "apiKey": os.getenv("EXA_API_KEY"),
          },
      }
  )
  ```
</CodeGroup>

## Learn more

For more information about working with templates, see:

* [Template Quickstart](/docs/template/quickstart) - Get started with custom templates
* [Defining Templates](/docs/template/defining-template) - Learn all template configuration options
* [Template Build](/docs/template/build) - Understand the build process
