> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Running commands in sandbox

You can run terminal commands inside the sandbox using the `commands.run()` method.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()
  const result = await sandbox.commands.run('ls -l')
  console.log(result)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()
  result = sandbox.commands.run('ls -l')
  print(result)
  ```
</CodeGroup>
