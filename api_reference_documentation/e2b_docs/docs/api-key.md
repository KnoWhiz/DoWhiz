> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# API key

To use the API key, you can either:

* **Set the API key as the `E2B_API_KEY` environment variable** to avoid passing it each time you create a sandbox.
* Or pass it directly to the `Sandbox` constructor as shown below:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create({ apiKey: 'YOUR_API_KEY' })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sbx = Sandbox.create(api_key="YOUR_API_KEY")
  ```
</CodeGroup>

## Where to find API key

You can get your API key at [dashboard](https://e2b.dev/dashboard?tab=keys).

<br />

# Access token

The access token is used only in the CLI and is **not needed in the SDK**. There's no need to set it when logging into the CLI using `e2b auth login`.

<Note>
  To authenticate without the browser, you can set `E2B_ACCESS_TOKEN` as an environment variable. This is useful for CI/CD pipelines.
</Note>

## Where to find access token

You can get your **Access token key** at the [dashboard](https://e2b.dev/dashboard/account).
