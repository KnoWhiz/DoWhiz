> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Expo app

> Expo web app running in the sandbox using Node.js

Basic Expo app.

<Note>
  The development server runs on port 8081 as soon as the sandbox is ready.
</Note>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { defaultBuildLogger, waitForURL } from "e2b";

  export const template = Template()
    .fromNodeImage()
    .setWorkdir("/home/user/expo-app")
    .runCmd("npx create-expo-app@latest . --yes")
    .runCmd("mv /home/user/expo-app/* /home/user/ && rm -rf /home/user/expo-app")
    .setWorkdir("/home/user")
    .setStartCmd("npx expo start", waitForURL("http://localhost:8081"));
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template, wait_for_url

  template = (
      Template()
      .from_node_image()
      .set_workdir("/home/user/expo-app")
      .run_cmd("npx create-expo-app@latest . --yes")
      .run_cmd("mv /home/user/expo-app/* /home/user/ && rm -rf /home/user/expo-app")
      .set_workdir("/home/user")
      .set_start_cmd("npx expo start", wait_for_url('http://localhost:8081'))
  )
  ```
</CodeGroup>

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as expoTemplate } from './template'

  Template.build(expoTemplate, 'expo-app', {
    cpuCount: 4,
    memoryMB: 8192,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from .template import template as expoTemplate

  Template.build(expoTemplate, 'expo-app',
      cpu_count=4,
      memory_mb=8192,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>
