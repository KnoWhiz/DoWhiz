> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Code contexts

Run code in different code execution contexts with E2B Code Interpreter SDK. This allows you to parallelize code execution by running code in different contexts at the same time.

By default the code is run in the Sandbox's default code execution context. You can change it by passing the `context` parameter to the `runCode()` method in JavaScript or `run_code()` in Python.

## Create a new Code Context

You can create a new code execution context by calling the `createCodeContext()` method in JavaScript or `create_code_context()` in Python and passing the context parameters.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()
  const context = await sandbox.createCodeContext({
    cwd: '/home/user',
    language: 'python',
    requestTimeoutMs: 60_000,
  })

  const result = await sandbox.runCode('print("Hello, world!")', { context })
  console.log(result)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()
  context = sandbox.create_code_context(
      cwd='/home/user',
      language='python',
      request_timeout=60_000,
  )
  result = sandbox.run_code('print("Hello, world!")', context=context)
  print(result)
  ```
</CodeGroup>

## List active Code Contexts

You can list active code execution contexts by calling the `listCodeContexts()` method in JavaScript or `list_code_contexts()` in Python. This will return a list of active code execution contexts.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()
  const contexts = await sandbox.listCodeContexts()
  console.log(contexts)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()
  contexts = sandbox.list_code_contexts()
  print(contexts)
  ```
</CodeGroup>

## Restart a Code Context

You can restart an active code execution context by calling the `restartCodeContext()` method in JavaScript or `restart_code_context()` in Python and passing the context object or context ID.
Restarting a context will clear its state and start a new code execution session in the same context.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()
  const context = await sandbox.createCodeContext({
    cwd: '/home/user',
    language: 'python',
    requestTimeoutMs: 60_000,
  })

  // using context object
  await sandbox.restartCodeContext(context)

  // using context ID
  await sandbox.restartCodeContext(context.contextId)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()
  context = sandbox.create_code_context(
      cwd='/home/user',
      language='python',
      request_timeout=60_000,
  )

  # using context object
  restarted_context = sandbox.restart_code_context(context)
  print(restarted_context)

  # using context ID
  restarted_context = sandbox.restart_code_context(context.contextId)
  print(restarted_context)
  ```
</CodeGroup>

## Remove a Code Context

You can remove an active code execution context by calling the `removeCodeContext()` method in JavaScript or `remove_code_context()` in Python and passing the context object or context ID.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()
  const context = await sandbox.createCodeContext({
      cwd: '/home/user',
      language: 'python',
      requestTimeoutMs: 60_000,
  })

  // using context object
  await sandbox.removeCodeContext(context)

  // using context ID
  await sandbox.removeCodeContext(context.contextId)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()
  context = sandbox.create_code_context(
      cwd='/home/user',
      language='python',
      request_timeout=60_000,
  )

  # using context object
  sandbox.remove_code_context(context)

  # using context ID
  sandbox.remove_code_context(context.contextId)
  ```
</CodeGroup>
