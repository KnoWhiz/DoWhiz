> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Run Python code

Use the `runCode`/`run_code` method to run Python code inside the sandbox.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  const execution = await sbx.runCode('print("Hello, world!")')
  console.log(execution)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()
  execution = sbx.run_code('print("Hello, world!")')
  print(execution)
  ```
</CodeGroup>
