> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Read & write files

## Reading files

You can read files from the sandbox filesystem using the `files.read()` method.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()
  const fileContent = await sandbox.files.read('/path/to/file')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()
  file_content = sandbox.files.read('/path/to/file')
  ```
</CodeGroup>

## Writing single files

You can write single files to the sandbox filesystem using the `files.write()` method.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  await sandbox.files.write('/path/to/file', 'file content')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()

  sandbox.files.write('/path/to/file', 'file content')
  ```
</CodeGroup>

## Writing multiple files

You can also write multiple files to the sandbox.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  await sandbox.files.write([
      { path: '/path/to/a', data: 'file content' },
      { path: '/another/path/to/b', data: 'file content' }
  ])
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()

  sandbox.files.write_files([
      { "path": "/path/to/a", "data": "file content" },
      { "path": "another/path/to/b", "data": "file content" }
  ])
  ```
</CodeGroup>
