> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Install custom packages

There are two ways to install custom packages in the E2B Sandbox.

1. [Create custom sandbox with preinstalled packages](#create-a-custom-sandbox).
2. [Install packages during the sandbox runtime](#install-packages-during-the-sandbox-runtime).

***

## Create a custom sandbox

Use this option if you know beforehand what packages you need in the sandbox.

<Note>
  Sandbox templates allow you to define custom sandboxes with preinstalled packages and configurations.
</Note>

### 1. Install the E2B SDK

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npm install e2b dotenv
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  pip install e2b python-dotenv
  ```
</CodeGroup>

Create a `.env` file with your API key:

```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
E2B_API_KEY=e2b_***
```

### 2. Create a template file

Define your custom template with the packages you need.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b';

  export const template = Template()
    .fromTemplate("code-interpreter-v1")
    .pipInstall(['cowsay'])  // Install Python packages
    .npmInstall(['cowsay']);  // Install Node.js packages
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_template("code-interpreter-v1")
      .pip_install(['cowsay'])  # Install Python packages
      .npm_install(['cowsay'])  # Install Node.js packages
  )
  ```
</CodeGroup>

### 3. Create a build script

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.prod.ts
  import "dotenv/config";
  import { Template, defaultBuildLogger } from 'e2b';
  import { template } from './template';

  async function main() {
    await Template.build(template, 'custom-packages', {
      cpuCount: 2,
      memoryMB: 2048,
      onBuildLogs: defaultBuildLogger(),
    });
  }

  main().catch(console.error);
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.prod.py
  from dotenv import load_dotenv
  from e2b import Template, default_build_logger
  from template import template

  load_dotenv()

  if __name__ == '__main__':
      Template.build(
          template,
          'custom-packages',
          cpu_count=2,
          memory_mb=2048,
          on_build_logs=default_build_logger(),
      )
  ```
</CodeGroup>

### 4. Build the template

Run the build script to create your custom template:

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npx tsx build.prod.ts
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  python build.prod.py
  ```
</CodeGroup>

This will build your template and you'll see build logs in the console.

### 5. Use your custom sandbox

Now you can create sandboxes from your custom template:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from 'e2b'

  const sbx = await Sandbox.create("custom-packages")

  // Your packages are already installed and ready to use
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Sandbox

  sbx = Sandbox.create("custom-packages")

  # Your packages are already installed and ready to use
  ```
</CodeGroup>

***

## Install packages during the sandbox runtime

Use this option if don't know beforehand what packages you need in the sandbox. You can install packages with the package manager of your choice.

<Note>
  The packages installed during the runtime are available only in the running sandbox instance.
  When you start a new sandbox instance, the packages are not be available.
</Note>

### 1. Install Python packages with PIP

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  sbx.commands.run('pip install cowsay') // This will install the cowsay package
  sbx.runCode(`
    import cowsay
    cowsay.cow("Hello, world!")
  `)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()
  sbx.commands.run("pip install cowsay") # This will install the cowsay package
  sbx.run_code("""
    import cowsay
    cowsay.cow("Hello, world!")
  """)
  ```
</CodeGroup>

### 2. Install Node.js packages with NPM

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  sbx.commands.run('npm install cowsay') // This will install the cowsay package
  sbx.runCode(`
    const cowsay = require('cowsay')
    console.log(cowsay.say({ text: 'Hello, world!' }))
  `, { language: 'javascript' })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()
  sbx.commands.run("npm install cowsay") # This will install the cowsay package
  sbx.run_code("""
    import { say } from 'cowsay'
    console.log(say('Hello, world!'))
  """, language="javascript")
  ```
</CodeGroup>

### 3. Install packages with package manager of your choice

Since E2B Sandboxes are Debian based machines, you can use any package manager supported by Debian.
You just need to make sure that the package manager is already installed in the sandbox.

For example, to install `curl` and `git`, you can use the following commands:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  await sbx.commands.run('apt-get update && apt-get install -y curl git')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()
  sbx.commands.run("apt-get update && apt-get install -y curl git")
  ```
</CodeGroup>
