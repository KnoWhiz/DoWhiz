> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Computer use

> Build AI agents that see, understand, and control virtual Linux desktops using E2B Desktop sandboxes.

Computer use agents interact with graphical desktops the same way a human would — viewing the screen, clicking, typing, and scrolling. E2B provides the sandboxed desktop environment where these agents operate safely, with [VNC](https://en.wikipedia.org/wiki/Virtual_Network_Computing) streaming for real-time visual feedback.

For a complete working implementation, see [E2B Surf](https://github.com/e2b-dev/surf) — an open-source computer use agent you can try via the [live demo](https://surf.e2b.dev).

## How it works

The computer use agent loop follows this pattern:

1. **User sends a command** — e.g., "Open Firefox and search for AI news"
2. **Agent creates a desktop sandbox** — an Ubuntu 22.04 environment with [XFCE](https://xfce.org/) desktop and pre-installed applications
3. **Agent takes a screenshot** — captures the current desktop state via E2B Desktop SDK
4. **LLM analyzes the screenshot** — a vision model (e.g., [OpenAI Computer Use API](https://platform.openai.com/docs/guides/computer-use)) decides what action to take
5. **Action is executed** — click, type, scroll, or keypress via E2B Desktop SDK
6. **Repeat** — new screenshot is taken and sent back to the LLM until the task is complete

## Install the E2B Desktop SDK

The [E2B Desktop](https://github.com/e2b-dev/desktop) SDK gives your agent a full Linux desktop with mouse, keyboard, and screen capture APIs.

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  npm i @e2b/desktop
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  pip install e2b-desktop
  ```
</CodeGroup>

## Core implementation

The following snippets are adapted from [E2B Surf](https://github.com/e2b-dev/surf).

### Setting up the sandbox

Create a desktop sandbox and start VNC streaming so you can view the desktop in a browser.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/desktop'

  // Create a desktop sandbox with a 5-minute timeout
  const sandbox = await Sandbox.create({
    resolution: [1024, 720],
    dpi: 96,
    timeoutMs: 300_000,
  })

  // Start VNC streaming for browser-based viewing
  await sandbox.stream.start()
  const streamUrl = sandbox.stream.getUrl()
  console.log('View desktop at:', streamUrl)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_desktop import Sandbox

  # Create a desktop sandbox with a 5-minute timeout
  sandbox = Sandbox.create(
      resolution=(1024, 720),
      dpi=96,
      timeout=300,
  )

  # Start VNC streaming for browser-based viewing
  sandbox.stream.start()
  stream_url = sandbox.stream.get_url()
  print("View desktop at:", stream_url)
  ```
</CodeGroup>

### Executing desktop actions

The E2B Desktop SDK maps directly to mouse and keyboard actions. Here's how Surf translates LLM-returned actions into desktop interactions.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/desktop'

  const sandbox = await Sandbox.create({ timeoutMs: 300_000 })

  // Mouse actions
  await sandbox.leftClick(500, 300)
  await sandbox.rightClick(500, 300)
  await sandbox.doubleClick(500, 300)
  await sandbox.middleClick(500, 300)
  await sandbox.moveMouse(500, 300)
  await sandbox.drag([100, 200], [400, 500])

  // Keyboard actions
  await sandbox.write('Hello, world!')  // Type text
  await sandbox.press('Enter')          // Press a key

  // Scrolling
  await sandbox.scroll('down', 3)  // Scroll down 3 ticks
  await sandbox.scroll('up', 3)    // Scroll up 3 ticks

  // Screenshots
  const screenshot = await sandbox.screenshot()  // Returns Buffer

  // Run terminal commands
  await sandbox.commands.run('ls -la /home')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_desktop import Sandbox

  sandbox = Sandbox.create(timeout=300)

  # Mouse actions
  sandbox.left_click(500, 300)
  sandbox.right_click(500, 300)
  sandbox.double_click(500, 300)
  sandbox.middle_click(500, 300)
  sandbox.move_mouse(500, 300)
  sandbox.drag([100, 200], [400, 500])

  # Keyboard actions
  sandbox.write("Hello, world!")  # Type text
  sandbox.press("Enter")          # Press a key

  # Scrolling
  sandbox.scroll("down", 3)  # Scroll down 3 ticks
  sandbox.scroll("up", 3)    # Scroll up 3 ticks

  # Screenshots
  screenshot = sandbox.screenshot()  # Returns bytes

  # Run terminal commands
  sandbox.commands.run("ls -la /home")
  ```
</CodeGroup>

### Agent loop

The core loop takes screenshots, sends them to an LLM, and executes the returned actions on the desktop. This is a simplified version of how [Surf](https://github.com/e2b-dev/surf) drives the computer use cycle.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/desktop'

  const sandbox = await Sandbox.create({
    resolution: [1024, 720],
    timeoutMs: 300_000,
  })
  await sandbox.stream.start()

  while (true) {
    // 1. Capture the current desktop state
    const screenshot = await sandbox.screenshot()

    // 2. Send screenshot to your LLM and get the next action
    //    (use OpenAI Computer Use, Anthropic Claude, etc.)
    const action = await getNextActionFromLLM(screenshot)

    if (!action) break // LLM signals task is complete

    // 3. Execute the action on the desktop
    switch (action.type) {
      case 'click':
        await sandbox.leftClick(action.x, action.y)
        break
      case 'type':
        await sandbox.write(action.text)
        break
      case 'keypress':
        await sandbox.press(action.keys)
        break
      case 'scroll':
        await sandbox.scroll(
          action.scrollY < 0 ? 'up' : 'down',
          Math.abs(action.scrollY)
        )
        break
      case 'drag':
        await sandbox.drag(
          [action.startX, action.startY],
          [action.endX, action.endY]
        )
        break
    }
  }

  await sandbox.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_desktop import Sandbox

  sandbox = Sandbox.create(
      resolution=(1024, 720),
      timeout=300,
  )
  sandbox.stream.start()

  while True:
      # 1. Capture the current desktop state
      screenshot = sandbox.screenshot()

      # 2. Send screenshot to your LLM and get the next action
      #    (use OpenAI Computer Use, Anthropic Claude, etc.)
      action = get_next_action_from_llm(screenshot)

      if not action:
          break  # LLM signals task is complete

      # 3. Execute the action on the desktop
      if action.type == "click":
          sandbox.left_click(action.x, action.y)
      elif action.type == "type":
          sandbox.write(action.text)
      elif action.type == "keypress":
          sandbox.press(action.keys)
      elif action.type == "scroll":
          direction = "up" if action.scroll_y < 0 else "down"
          sandbox.scroll(direction, abs(action.scroll_y))
      elif action.type == "drag":
          sandbox.drag(
              [action.start_x, action.start_y],
              [action.end_x, action.end_y],
          )

  sandbox.kill()
  ```
</CodeGroup>

The `getNextActionFromLLM` / `get_next_action_from_llm` function is where you integrate your chosen LLM. See [Connect LLMs to E2B](/docs/quickstart/connect-llms) for integration patterns with OpenAI, Anthropic, and other providers.

## Related guides

<CardGroup cols={3}>
  <Card title="Desktop template" icon="desktop" href="/docs/template/examples/desktop">
    Build desktop sandboxes with Ubuntu, XFCE, and VNC streaming
  </Card>

  <Card title="Connect LLMs" icon="brain" href="/docs/quickstart/connect-llms">
    Integrate AI models with sandboxes using tool calling
  </Card>

  <Card title="Sandbox lifecycle" icon="rotate" href="/docs/sandbox">
    Create, manage, and control sandbox lifecycle
  </Card>
</CardGroup>
