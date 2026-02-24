> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# The sandbox is running but port is not open - 49999

When trying to create a custom sandbox template, you might encounter the issue when trying to call `.runCode` / `.run_code` method using the E2B Code Interpreter SDK.

The reason for this is that either the default start command is not specified on the template or that the base image is other than the Code Interpreter image thus the Code Interpreter inside the sandbox cannot be started.

**Example error**

```txt  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
{"sandboxId":"ilmqag7sxz49zjc3gex7x","message":"The sandbox is running but port is not open","port":49999,"code":502}: This error is likely due to sandbox timeout. You can modify the sandbox timeout by passing 'timeout' when starting the sandbox or calling '.set_timeout' on the sandbox with the desired timeout.
```

## Solution

### Current build system (V2)

With the current build system, you need to use the `code-interpreter` as a base template in your template definition.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template } from '@e2b/code-interpreter'

  const template = Template().fromTemplate("code-interpreter-v1")
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Template

  template = Template().from_template("code-interpreter-v1")
  ```
</CodeGroup>

<Note>
  We strongly recommend building custom template via the previous example. However, if you choose to build from docker image, do as follows.
</Note>

<CodeGroup>
  ```ts JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template, waitForURL } from '@e2b/code-interpreter'

  const template = Template()
      .fromImage("e2bdev/code-interpreter:latest")
      .setStartCmd(
          "sudo /root/.jupyter/start-up.sh", waitForURL("http://localhost:49999/health")
      )
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Template, wait_for_url

  template = (
      Template()
      .from_image("e2bdev/code-interpreter:latest")
      .set_start_cmd(
          "sudo /root/.jupyter/start-up.sh", wait_for_url("http://localhost:49999/health")
      )
  )
  ```
</CodeGroup>
