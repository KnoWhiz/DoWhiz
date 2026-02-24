> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Connect to running sandbox

If you have a running sandbox, you can connect to it using the `Sandbox.connect()` method and then start controlling it with our SDK.

This is useful if you want to, for example, reuse the same sandbox instance for the same user after a short period of inactivity.

## 1. Get the sandbox ID

To connect to a running sandbox, you first need to retrieve its ID. You can do this by calling the `Sandbox.list()` method.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()

  // Get all running sandboxes
  const paginator = await Sandbox.list({
    query: { state: ['running'] },
  })

  const runningSandboxes = await paginator.nextItems()
  if (runningSandboxes.length === 0) {
    throw new Error('No running sandboxes found')
  }

  // Get the ID of the sandbox you want to connect to
  const sandboxId = runningSandboxes[0].sandboxId
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()

  # Get all running sandboxes
  paginator = Sandbox.list()

  # Get the ID of the sandbox you want to connect to
  running_sandboxes = paginator.next_items()
  if len(running_sandboxes) == 0:
      raise Exception("No running sandboxes found")

  # Get the ID of the sandbox you want to connect to
  sandbox_id = running_sandboxes[0].sandbox_id
  ```
</CodeGroup>

## 2. Connect to the sandbox

Now that you have the sandbox ID, you can connect to the sandbox using the `Sandbox.connect()` method.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={3} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.connect(sandboxId)

  // Now you can use the sandbox as usual
  // ...
  const result = await sandbox.commands.run("whoami")
  console.log(`Running in sandbox ${sandbox.sandboxId} as "${result.stdout.trim()}"`)
  ```

  ```python Python highlight={3} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.connect(sandbox_id)

  # Now you can use the sandbox as usual
  # ...
  r = sandbox.commands.run("whoami")
  print(f"Running in sandbox {sandbox.sandbox_id} as \"{r.stdout.strip()}\"")
  ```
</CodeGroup>
