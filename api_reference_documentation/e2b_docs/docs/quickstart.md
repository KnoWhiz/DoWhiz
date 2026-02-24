> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Running your first Sandbox

> This guide will show you how to start your first E2B Sandbox.

<Steps>
  <Step title="Create E2B account">
    Every new E2B account get \$100 in credits. You can sign up [here](https://e2b.dev/auth/sign-up).
  </Step>

  <Step title="Set your environment variables">
    1. Navigate to the E2B Dashboard.
    2. Copy your [API key](https://e2b.dev/dashboard?tab=keys).
    3. Paste your E2B API key into your .env file.

    ```bash .env theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
    E2B_API_KEY=e2b_***
    ```
  </Step>

  <Step title="Install E2B SDK">
    Install the E2B SDK to your project by running the following command in your terminal.

    <CodeGroup>
      ```javascript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      npm i @e2b/code-interpreter dotenv
      ```

      ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      pip install e2b-code-interpreter python-dotenv
      ```
    </CodeGroup>
  </Step>

  <Step title="Write code for starting sandbox">
    We'll write the minimal code for starting Sandbox, executing Python inside it and listing all files inside the root directory.

    <CodeGroup>
      ```javascript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      // index.ts
      import 'dotenv/config'
      import { Sandbox } from '@e2b/code-interpreter'

      const sbx = await Sandbox.create() // By default the sandbox is alive for 5 minutes
      const execution = await sbx.runCode('print("hello world")') // Execute Python inside the sandbox
      console.log(execution.logs)

      const files = await sbx.files.list('/')
      console.log(files)
      ```

      ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      # main.py
      from dotenv import load_dotenv
      load_dotenv()
      from e2b_code_interpreter import Sandbox

      sbx = Sandbox.create() # By default the sandbox is alive for 5 minutes
      execution = sbx.run_code("print('hello world')") # Execute Python inside the sandbox
      print(execution.logs)

      files = sbx.files.list("/")
      print(files)
      ```
    </CodeGroup>
  </Step>

  <Step title="Start your first E2B sandbox">
    Run the code with the following command:

    <CodeGroup>
      ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      npx tsx ./index.ts
      ```

      ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      python main.py
      ```
    </CodeGroup>
  </Step>
</Steps>
