> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Sandbox persistence

<Note>
  Sandbox persistence is currently in public beta:

  1. Consider [some limitations](#limitations-while-in-beta).
  2. The persistence is free for all users during the beta.
</Note>

The sandbox persistence allows you to pause your sandbox and resume it later from the same state it was in when you paused it.

This includes not only state of the sandbox's filesystem but also the sandbox's memory. This means all running processes, loaded variables, data, etc.

## Sandbox state transitions

Understanding how sandboxes transition between different states is crucial for managing their lifecycle effectively. Here's a diagram showing the possible state transitions:

<Frame>
  <img src="https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/diagram.png?fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=e1704d8c49e57248d4f39d219417f45e" data-og-width="693" width="693" data-og-height="961" height="961" data-path="images/diagram.png" data-optimize="true" data-opv="3" srcset="https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/diagram.png?w=280&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=64e432d304c2d7b9f6b6bce2df534df9 280w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/diagram.png?w=560&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=130ca9605640a62b2278d10c61a122b5 560w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/diagram.png?w=840&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=64971a4c1d01138df778bcd3e0b8e077 840w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/diagram.png?w=1100&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=ad62e46b5daf517409885d24d9a427f1 1100w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/diagram.png?w=1650&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=04e0ffe5772a64cb8815ae92b0cabbed 1650w, https://mintcdn.com/e2b/bga6ifW6jAKoRCaG/images/diagram.png?w=2500&fit=max&auto=format&n=bga6ifW6jAKoRCaG&q=85&s=69fb7dcd8b091d361e9b1c6d2118cec5 2500w" />
</Frame>

### State descriptions

* **Running**: The sandbox is actively running and can execute code. This is the initial state after creation.
* **Paused**: The sandbox execution is suspended but its state is preserved.
* **Killed**: The sandbox is terminated and all resources are released. This is a terminal state.

### Changing sandbox's state

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create() // Starts in Running state

  // Pause the sandbox
  await sandbox.betaPause() // Running → Paused

  // Resume the sandbox
  await sandbox.connect() // Running/Paused → Running

  // Kill the sandbox (from any state)
  await sandbox.kill() // Running/Paused → Killed
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()  # Starts in Running state

  # Pause the sandbox
  sandbox.betaPause()  # Running → Paused

  # Resume the sandbox
  sandbox.connect()  # Running/Paused → Running

  # Kill the sandbox (from any state)
  sandbox.kill()  # Running/Paused → Killed
  ```
</CodeGroup>

## Pausing sandbox

When you pause a sandbox, both the sandbox's filesystem and memory state will be saved. This includes all the files in the sandbox's filesystem and all the running processes, loaded variables, data, etc.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={8-9} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  console.log('Sandbox created', sbx.sandboxId)

  // Pause the sandbox
  // You can save the sandbox ID in your database to resume the sandbox later
  await sbx.betaPause()
  console.log('Sandbox paused', sbx.sandboxId)
  ```

  ```python Python highlight={8-9} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()
  print('Sandbox created', sbx.sandbox_id)

  # Pause the sandbox
  # You can save the sandbox ID in your database to resume the sandbox later
  sbx.beta_pause()
  print('Sandbox paused', sbx.sandbox_id) 
  ```
</CodeGroup>

## Resuming sandbox

When you resume a sandbox, it will be in the same state it was in when you paused it.
This means that all the files in the sandbox's filesystem will be restored and all the running processes, loaded variables, data, etc. will be restored.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={12-13} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  console.log('Sandbox created', sbx.sandboxId)

  // Pause the sandbox
  // You can save the sandbox ID in your database to resume the sandbox later
  await sbx.betaPause()
  console.log('Sandbox paused', sbx.sandboxId)

  // Connect to the sandbox (it will automatically resume the sandbox, if paused)
  const sameSbx = await sbx.connect()
  console.log('Connected to the sandbox', sameSbx.sandboxId)
  ```

  ```python Python highlight={12-13} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()
  print('Sandbox created', sbx.sandbox_id)

  # Pause the sandbox
  # You can save the sandbox ID in your database to resume the sandbox later
  sbx.beta_pause()
  print('Sandbox paused', sbx.sandbox_id)

  # Connect to the sandbox (it will automatically resume the sandbox, if paused)
  same_sbx = sbx.connect()
  print('Connected to the sandbox', same_sbx.sandbox_id)
  ```
</CodeGroup>

## Listing paused sandboxes

You can list all paused sandboxes by calling the `Sandbox.list` method and supplying the `state` query parameter.
More information about using the method can be found in [List Sandboxes](/docs/sandbox/list).

<CodeGroup>
  ```js JavaScript & TypeScript highlight={4,7} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, SandboxInfo } from '@e2b/code-interpreter'

  // List all paused sandboxes
  const paginator = Sandbox.list({ query: { state: ['paused'] } })

  // Get the first page of paused sandboxes
  const sandboxes = await paginator.nextItems()

  // Get all paused sandboxes
  while (paginator.hasNext) {
    const items = await paginator.nextItems()
    sandboxes.push(...items)
  }
  ```

  ```python Python highlight={4,7} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # List all paused sandboxes
  from e2b_code_interpreter import Sandbox, SandboxQuery, SandboxState

  paginator = Sandbox.list(SandboxQuery(state=[SandboxState.PAUSED]))

  # Get the first page of paused sandboxes
  sandboxes = paginator.next_items()

  # Get all paused sandboxes
  while paginator.has_next:
    items = paginator.next_items()
    sandboxes.extend(items)
  ```
</CodeGroup>

## Removing paused sandboxes

You can remove paused sandboxes by calling the `kill` method on the Sandbox instance.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={11,14} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.create()
  console.log('Sandbox created', sbx.sandboxId)

  // Pause the sandbox
  // You can save the sandbox ID in your database to resume the sandbox later
  await sbx.betaPause()

  // Remove the sandbox
  await sbx.kill()

  // Remove sandbox by id
  await Sandbox.kill(sbx.sandboxId)
  ```

  ```python Python highlight={9,12} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create()

  # Pause the sandbox
  sbx.beta_pause()

  # Remove the sandbox
  sbx.kill()

  # Remove sandbox by id
  Sandbox.kill(sbx.sandbox_id)
  ```
</CodeGroup>

## Sandbox's timeout

When you connect to a sandbox, the sandbox's timeout is reset to the default timeout of an E2B sandbox - 5 minutes.

You can pass a custom timeout to the `Sandbox.connect()`/`Sandbox.connect()` method like this:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sbx = await Sandbox.connect(sandboxId, { timeoutMs: 60 * 1000 }) // 60 seconds
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.connect(sandbox_id, timeout=60) # 60 seconds
  ```
</CodeGroup>

### Auto-pause (beta)

**Note: Auto-pause is currently in beta and available through `Sandbox.betaCreate()`/`Sandbox.beta_create()` method.**

Sandboxes can now automatically pause after they time out. When a sandbox is paused, it stops consuming compute but preserves its state. The default inactivity timeout is 10 minutes. You can change the timeout by passing the `timeoutMs`/`timeout` parameter to the `Sandbox.connect()`/`Sandbox.connect()` method.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  // Create sandbox with auto-pause enabled
  const sandbox = await Sandbox.betaCreate({
      autoPause: true,
      timeoutMs: 10 * 60 * 1000 // Optional: change the default timeout (10 minutes)
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  # Create sandbox with auto-pause enabled (Beta)
  sandbox = Sandbox.beta_create(
      auto_pause=True,  # Auto-pause after the sandbox times out
      timeout=10 * 60, # Optional: change the default timeout (10 minutes)
  )
  ```
</CodeGroup>

The auto-pause is persistent, meaning that if your sandbox resumes and it times out, it will be automatically paused again.

If you `.kill()` the sandbox, it will be permanently deleted and you won't be able to resume it.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  // Create sandbox with auto-pause enabled (Beta)
  const sandbox = await Sandbox.betaCreate({
      autoPause: true  // Auto-pause after the sandbox times out
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  # Create sandbox with auto-pause enabled (Beta)
  sandbox = Sandbox.beta_create(
      auto_pause=True  # Auto-pause after the sandbox times out
  )
  ```
</CodeGroup>

## Network

If you have a service (for example a server) running inside your sandbox and you pause the sandbox, the service won't be accessible from the outside and all the clients will be disconnected.
If you resume the sandbox, the service will be accessible again but you need to connect clients again.

## Limitations while in beta

### Pause and resume performance

* Pausing a sandbox takes approximately **4 seconds per 1 GiB of RAM**
* Resuming a sandbox takes approximately **1 second**

### Continuous runtime limits

* A sandbox can remain running (without being paused) for:
  * **24 hours** on the **Pro tier**
  * **1 hour** on the **Base tier**
* After a sandbox is paused and resumed, the continuous runtime limit is **reset**
