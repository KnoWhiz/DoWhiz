> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Next.js app

> Next.js web app running in the sandbox using Node.js

Basic Next.js app with Tailwind and shadcn UI

<Note>
  The development server runs on port 3000 as soon as the sandbox is ready.
</Note>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template, waitForURL } from 'e2b'

  export const template = Template()
    .fromNodeImage('21-slim')
    .setWorkdir('/home/user/nextjs-app')
    .runCmd(
      'npx create-next-app@14.2.30 . --ts --tailwind --no-eslint --import-alias "@/*" --use-npm --no-app --no-src-dir'
    )
    .runCmd('npx shadcn@2.1.7 init -d')
    .runCmd('npx shadcn@2.1.7 add --all')
    .runCmd(
      'mv /home/user/nextjs-app/* /home/user/ && rm -rf /home/user/nextjs-app'
    )
    .setWorkdir('/home/user')
    .setStartCmd('npx next --turbo', waitForURL('http://localhost:3000'))
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template, wait_for_url

  template = (
      Template()
      .from_node_image("21-slim")
      .set_workdir("/home/user/nextjs-app")
      .run_cmd(
          'npx create-next-app@14.2.30 . --ts --tailwind --no-eslint --import-alias "@/*" --use-npm --no-app --no-src-dir'
      )
      .run_cmd("npx shadcn@2.1.7 init -d")
      .run_cmd("npx shadcn@2.1.7 add --all")
      .run_cmd("mv /home/user/nextjs-app/* /home/user/ && rm -rf /home/user/nextjs-app")
      .set_workdir("/home/user")
      .set_start_cmd("npx next --turbo", wait_for_url('http://localhost:3000'))
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as nextJSTemplate } from './template'

  Template.build(nextJSTemplate, 'nextjs-app', {
    cpuCount: 4,
    memoryMB: 4096,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from .template import template as nextjsTemplate

  Template.build(nextjsTemplate, 'nextjs-app',
      cpu_count=4,
      memory_mb=4096,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>
