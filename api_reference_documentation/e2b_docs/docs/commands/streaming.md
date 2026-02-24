> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Streaming command output

To stream command output as it is being executed, pass the `onStdout`, `onStderr` callbacks to the `commands.run()` method in JavaScript
or the `on_stdout`, `on_stderr` callbacks to the `commands.run()` method in Python.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  const result = await sandbox.commands.run('echo hello; sleep 1; echo world', {
    onStdout: (data) => {
      console.log(data)
    },
    onStderr: (data) => {
      console.log(data)
    },
  })
  console.log(result)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()

  result = sandbox.commands.run('echo hello; sleep 1; echo world', on_stdout=lambda data: print(data), on_stderr=lambda data: print(data))
  print(result)
  ```
</CodeGroup>
