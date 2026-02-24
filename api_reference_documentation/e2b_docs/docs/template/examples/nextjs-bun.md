> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Next.js app (Bun)

> Next.js web app running in the sandbox using Bun

Basic Next.js app with Tailwind and shadcn UI using Bun.

<Note>
  The development server runs on port 3000 as soon as the sandbox is ready.
</Note>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template, waitForURL } from 'e2b'

  export const template = Template()
    .fromBunImage('1.3')
    .setWorkdir('/home/user/nextjs-app')
    .runCmd(
      'bun create next-app --app --ts --tailwind --turbopack --yes --use-bun .'
    )
    .runCmd('bunx --bun shadcn@latest init -d')
    .runCmd('bunx --bun shadcn@latest add --all')
    .runCmd(
      'mv /home/user/nextjs-app/* /home/user/ && rm -rf /home/user/nextjs-app'
    )
    .setWorkdir('/home/user')
    .setStartCmd('bun --bun run dev --turbo', waitForURL('http://localhost:3000'))
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template, wait_for_url

  template = (
      Template()
      .from_bun_image('1.3')
      .set_workdir('/home/user/nextjs-app')
      .run_cmd('bun create next-app --app --ts --tailwind --turbopack --yes --use-bun .')
      .run_cmd('bunx --bun shadcn@latest init -d')
      .run_cmd('bunx --bun shadcn@latest add --all')
      .run_cmd('mv /home/user/nextjs-app/* /home/user/ && rm -rf /home/user/nextjs-app')
      .set_workdir('/home/user')
      .set_start_cmd('bun --bun run dev --turbo', wait_for_url('http://localhost:3000'))
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as nextJSTemplate } from './template'

  Template.build(nextJSTemplate, 'nextjs-app-bun', {
    cpuCount: 4,
    memoryMB: 4096,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from .template import template as nextjsTemplate

  Template.build(nextjsTemplate, 'nextjs-app-bun',
      cpu_count=4,
      memory_mb=4096,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>
