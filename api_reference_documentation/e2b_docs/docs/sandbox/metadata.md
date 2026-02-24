> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Sandbox metadata

Metadata is a way to attach arbitrary key-value pairs for a sandbox.

This is useful in various scenarios, for example:

* Associate a sandbox with a user session.
* Store custom user data for a sandbox like API keys.
* Associate a sandbox with a user ID and [connect to it later](/docs/sandbox/connect).

You specify metadata when creating a sandbox and can access it later through listing running sandboxes with `Sandbox.list()` method.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={6} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  // Create sandbox with metadata.
  const sandbox = await Sandbox.create({
    metadata: {
      userId: '123',
    },
  })

  // List running sandboxes and access metadata.
  const paginator = await Sandbox.list()
  const runningSandboxes = await paginator.nextItems()
  // Will print:
  // {
  //   'userId': '123',
  // }
  console.log(runningSandboxes[0].metadata)
  ```

  ```python Python highlight={6} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  # Create sandbox with metadata.
  sandbox = Sandbox.create(
    metadata={
      'userId': '123',
    },
  )

  # List running sandboxes and access metadata.
  paginator = Sandbox.list()
  running_sandboxes = paginator.next_items()
  # Will print:
  # {
  #   'userId': '123',
  # }
  print(running_sandboxes[0].metadata)
  ```
</CodeGroup>

## Filtering sandboxes by metadata

You can also filter sandboxes by metadata, you can find more about it [here](/docs/sandbox/list#filtering-sandboxes).
