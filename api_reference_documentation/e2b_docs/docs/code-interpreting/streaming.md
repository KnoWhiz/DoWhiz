> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Streaming

E2B Code Interpreter SDK allows you to stream the output, and results when executing code in the sandbox.

## Stream `stdout` and `stderr`

When using the `runCode()` method in JavaScript or `run_code()` in Python you can pass `onStdout`/`on_stdout` and `onStderr`/`on_stderr` callbacks to handle the output.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={15-17} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const codeToRun = `
    import time
    import sys
    print("This goes first to stdout")
    time.sleep(3)
    print("This goes later to stderr", file=sys.stderr)
    time.sleep(3)
    print("This goes last")
  `
  const sandbox = await Sandbox.create()
  sandbox.runCode(codeToRun, {
    // Use `onError` to handle runtime code errors
    onError: error => console.error('error:', error),
    onStdout: data => console.log('stdout:', data),
    onStderr: data => console.error('stderr:', data),
  })
  ```

  ```python Python highlight={17-19} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  code_to_run = """
    import time
    import sys
    print("This goes first to stdout")
    time.sleep(3)
    print("This goes later to stderr", file=sys.stderr)
    time.sleep(3)
    print("This goes last")
  """

  sandbox = Sandbox.create()
  sandbox.run_code(
    code_to_run,
    # Use `on_error` to handle runtime code errors
    on_error=lambda error: print('error:', error),
    on_stdout=lambda data: print('stdout:', data),
    on_stderr=lambda data: print('stderr:', data),
  )
  ```
</CodeGroup>

The code above will print the following:

<CodeGroup>
  ```javascript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  stdout: {
    error: false,
    line: "This goes first to stdout\n",
    timestamp: 1729049666861000,
  }
  stderr: {
    error: true,
    line: "This goes later to stderr\n",
    timestamp: 1729049669924000,
  }
  stdout: {
    error: false,
    line: "This goes last\n",
    timestamp: 1729049672664000,
  }
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  stdout: This goes first to stdout

  stderr: This goes later to stderr

  stdout: This goes last
  ```
</CodeGroup>

## Stream `results`

When using the `runCode()` method in JavaScript or `run_code()` in Python you can pass `onResults`/`on_results` callback
to receive results from the sandbox like charts, tables, text, and more.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={20} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const codeToRun = `
  import matplotlib.pyplot as plt

  # Prepare data
  categories = ['Category A', 'Category B', 'Category C', 'Category D']
  values = [10, 20, 15, 25]

  # Create and customize the bar chart
  plt.figure(figsize=(10, 6))
  plt.bar(categories, values, color='green')
  plt.xlabel('Categories')
  plt.ylabel('Values')
  plt.title('Values by Category')

  # Display the chart
  plt.show()
  `
  const sandbox = await Sandbox.create()
  await sandbox.runCode(codeToRun, {
    onResult: result => console.log('result:', result),
  })
  ```

  ```python Python highlight={24} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  code_to_run = """
  import matplotlib.pyplot as plt

  # Prepare data
  categories = ['Category A', 'Category B', 'Category C', 'Category D']
  values = [10, 20, 15, 25]

  # Create and customize the bar chart
  plt.figure(figsize=(10, 6))
  plt.bar(categories, values, color='green')
  plt.xlabel('Categories')
  plt.ylabel('Values')
  plt.title('Values by Category')

  # Display the chart
  plt.show()
  """

  sandbox = Sandbox.create()
  sandbox.run_code(
    code_to_run,
    on_result=lambda result: print('result:', result),
  )
  ```
</CodeGroup>
