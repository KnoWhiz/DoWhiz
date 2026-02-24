> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Docker

> Sandbox with Docker installed for running containers

## Install Docker

Use the official installation script from [get.docker.com](https://get.docker.com). The `hello-world` container run validates the installation.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template } from 'e2b'

  export const template = Template()
    .fromUbuntuImage('25.04')
    .runCmd('curl -fsSL https://get.docker.com | sudo sh')
    .runCmd('sudo docker run --rm hello-world')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template

  template = (
      Template()
      .from_ubuntu_image("25.04")
      .run_cmd("curl -fsSL https://get.docker.com | sudo sh")
      .run_cmd("sudo docker run --rm hello-world")
  )
  ```
</CodeGroup>

## Build the template

We recommend at least 2 CPUs and 2 GB of RAM for running Docker containers. **With lower RAM, your sandbox might run out of memory.**

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as dockerTemplate } from './template'

  Template.build(dockerTemplate, 'docker', {
    cpuCount: 2,
    memoryMB: 2048,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from .template import template as dockerTemplate

  Template.build(dockerTemplate, 'docker',
      cpu_count=2,
      memory_mb=2048,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>

## Run containers

Run an Alpine container that prints a hello message.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // sandbox.ts
  import { Sandbox } from 'e2b'

  const sbx = await Sandbox.create('docker')

  const result = await sbx.commands.run('sudo docker run --rm alpine echo "Hello from Alpine!"')
  console.log(result.stdout)

  await sbx.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # sandbox.py
  from e2b import Sandbox

  sbx = Sandbox.create('docker')

  result = sbx.commands.run('sudo docker run --rm alpine echo "Hello from Alpine!"')
  print(result.stdout)

  sbx.kill()
  ```
</CodeGroup>
