> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Quickstart

> Start with custom sandbox templates

E2B templates allow you to define custom sandboxes.
You can define the base image, environment variables, files to copy, commands to run, and
a start command that will be already running when you spawn the sandbox.
This way, you can have fully configured sandboxes with running processes ready to use with zero wait time for your users.

There are two ways how you can start creating a new template:

* using the CLI
* manually using the SDK

## CLI

You can use the E2B CLI to create a new template.

<Steps>
  <Step title="Install the E2B CLI">
    Install the latest version of the [E2B CLI](https://e2b.dev/docs/cli)
  </Step>

  <Step title="Initialize a new template">
    ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
    e2b template init
    ```
  </Step>

  <Step title="Follow the prompts">
    Follow the prompts to create a new template.
  </Step>

  <Step title="Done">
    Check the generated `README.md` file to see how to build and use your new template.
  </Step>
</Steps>

## Manual

### Install the packages

<Info>
  Requires the E2B SDK version at least 2.3.0
</Info>

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npm install e2b dotenv
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  pip install e2b dotenv
  ```
</CodeGroup>

Create the `.env` file

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
E2B_API_KEY=e2b_***
```

### Create a new template file

Create a template file with the following name and content

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template, waitForTimeout } from 'e2b';

  export const template = Template()
    .fromBaseImage()
    .setEnvs({
      HELLO: "Hello, World!",
    })
    .setStartCmd("echo $HELLO", waitForTimeout(5_000));
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template, wait_for_timeout

  template = (
      Template()
      .from_base_image()
      .set_envs(
          {
              "HELLO": "Hello, World!",
          }
      )
      .set_start_cmd("echo $HELLO", wait_for_timeout(5_000)))
  ```
</CodeGroup>

### Create a development build script

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.dev.ts
  import 'dotenv/config';
  import { Template, defaultBuildLogger } from 'e2b';
  import { template } from './template';

  async function main() {
    await Template.build(template, 'template-tag-dev', {
      cpuCount: 1,
      memoryMB: 1024,
      onBuildLogs: defaultBuildLogger(),
    });
  }

  main().catch(console.error);
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build_dev.py
  from dotenv import load_dotenv
  from e2b import Template, default_build_logger
  from template import template

  load_dotenv()

  if __name__ == '__main__':
      Template.build(
          template,
          'template-tag-dev',
          cpu_count=1,
          memory_mb=1024,
          on_build_logs=default_build_logger(),
      )
  ```
</CodeGroup>

### Create a production build script

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.prod.ts
  import 'dotenv/config';
  import { Template, defaultBuildLogger } from 'e2b';
  import { template } from './template';

  async function main() {
    await Template.build(template, 'template-tag', {
      cpuCount: 1,
      memoryMB: 1024,
      onBuildLogs: defaultBuildLogger(),
    });
  }

  main().catch(console.error);
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build_prod.py
  from dotenv import load_dotenv
  from e2b import Template, default_build_logger
  from template import template

  load_dotenv()

  if __name__ == '__main__':
      Template.build(
          template,
          'template-tag',
          cpu_count=1,
          memory_mb=1024,
          on_build_logs=default_build_logger(),
      )
  ```
</CodeGroup>

### Build the template

Build the development template

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npx tsx build.dev.ts
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  python build_dev.py
  ```
</CodeGroup>

Build the production template

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npx tsx build.prod.ts
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  python build_prod.py
  ```
</CodeGroup>

## Create a new Sandbox from the Template

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import 'dotenv/config';
  import { Sandbox } from 'e2b';

  // Create a Sandbox from development template
  const sandbox = await Sandbox.create("template-tag-dev");

  // Create a Sandbox from production template
  const sandbox = await Sandbox.create("template-tag");
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from dotenv import load_dotenv
  from e2b import Sandbox

  load_dotenv()

  # Create a new Sandbox from the development template
  sbx = Sandbox(template="template-tag-dev")

  # Create a new Sandbox from the production template
  sbx = Sandbox(template="template-tag")
  ```
</CodeGroup>

<Note>
  The template name is the identifier that can be used to create a new Sandbox.
</Note>
