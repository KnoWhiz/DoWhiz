> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Run Java code

Use the `runCode`/`run_code` method to run Java code inside the sandbox.
You'll need to pass the `language` parameter with value `java`.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  const execution = await sbx.runCode('System.out.println("Hello, world!");', { language: 'java' })
  console.log(execution)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()
  execution = sbx.run_code('System.out.println("Hello, world!");', language="java")
  print(execution)
  ```
</CodeGroup>
