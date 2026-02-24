> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# E2B Documentation

export const Quickstart = () => {
  const items = [{
    href: "/docs/quickstart",
    title: "Running your first Sandbox",
    description: "Learn how to start your first E2B Sandbox with our Python or JavaScript SDK.",
    icon: "circle-play"
  }, {
    href: "/docs/quickstart/connect-llms",
    title: "Connecting LLMs to E2B",
    description: "Connect your favorite LLM to E2B to run AI-generated code inside the Sandbox.",
    icon: "brain-circuit"
  }, {
    href: "/docs/quickstart/upload-download-files",
    title: "Uploading & downloading files",
    description: "A quick guide on how to upload and download files to and from the Sandbox.",
    icon: "cloud-arrow-up"
  }, {
    href: "/docs/quickstart/install-custom-packages",
    title: "Install custom packages",
    description: "Customize your Sandbox with third-party packages.",
    icon: "box-open-full"
  }];
  return <Columns cols={2}>
      {items.map(i => <Card title={i.title} href={i.href} icon={i.icon}>
          {i.description}
        </Card>)}
    </Columns>;
};

## What is E2B?

E2B provides isolated sandboxes that let agents safely execute code, process data, and run tools. Our SDKs make it easy to start and manage these environments.

Spin up a sandbox and run code in a few lines:

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npm i e2b
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  pip install e2b
  ```
</CodeGroup>

<CodeGroup>
  ```javascript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sandbox = await Sandbox.create() // Needs E2B_API_KEY environment variable
  const result = await sandbox.commands.run('echo "Hello from E2B Sandbox!"')
  console.log(result.stdout)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Sandbox

  sandbox = Sandbox.create()  # Needs E2B_API_KEY environment variable
  result = sandbox.commands.run('echo "Hello from E2B Sandbox!"')
  print(result.stdout)
  ```
</CodeGroup>

## E2B building blocks

A quick overview of the core building blocks you'll interact with when using E2B.

* [**Sandbox**](/docs/sandbox) — A fast, secure Linux VM created on demand for your agent

* [**Template**](/docs/template/quickstart) — Defines what environment a sandbox starts with

## How to use the docs

The documentation is split into three main sections:

* [**Quickstart**](#quickstart) — Step-by-step tutorials that walk you through spinning up your first E2B sandboxes.

* [**Examples**](#examples) — In-depth tutorials focused on specific use cases. Pick the topics that match what you're building.

* [**SDK Reference**](https://e2b.dev/docs/sdk-reference) — A complete technical reference for every SDK method, parameter, and configuration option.

## Quickstart

<Quickstart />

## Examples

<CardGroup cols={2}>
  <Card title="Computer Use" icon="desktop" href="/docs/use-cases/computer-use">
    Build AI agents that see, understand, and control virtual Linux desktops using E2B Desktop sandboxes.
  </Card>

  <Card title="GitHub Actions CI/CD" icon="gears" href="/docs/use-cases/ci-cd">
    Use E2B sandboxes in your GitHub Actions workflows to run testing, validation, and AI code reviews.
  </Card>
</CardGroup>
