> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Secured access

Secure access authenticates communication between SDK and sandbox controller.

Sandbox controller runs in sandbox itself and exposes APIs for work with file system, run commands, and generally control the sandbox via our SDK.
Without secure access, anyone with a sandbox ID can access the controller APIs and control the sandbox from inside.

<Note>
  SDKs version `v2.0.0` and above are using secure access by default when creating sandbox. This may not be compatible with older custom templates and you may need to rebuild them.
</Note>

## Migration path

When using custom templates created before envd `v0.2.0`, you need to rebuild the templates to enable secure access.
Temporarily, you can disable secure access by setting `secure` to `false` during sandbox creation, but this is not recommended for production use because it increases security risks.

You can check the template envd version using the `e2b template list` command or by viewing the templates list on the dashboard.

## Supported versions

All sandboxes based on templates with envd version at least `v0.2.0` already support secure access without any additional changes.

The secure access flag was introduced in `1.5.0` for JavaScript and Python SDKs to be used optionally.
Starting with SDK version `v2.0.0`, sandboxes are created with secure access enabled by default.

## Access sandbox API directly

In some cases, you might want to access sandbox controller APIs directly through its URL, such as when you are not using SDKs.
When secure access is enabled, you must provide an authentication token that was returned during sandbox creation.

Each call to the sandbox controller must include an additional header `X-Access-Token` with the access token value returned during sandbox creation.

For sandbox [upload](/docs/filesystem/upload#upload-with-pre-signed-url) and [download](/docs/filesystem/download#download-with-pre-signed-url) URLs, you need to generate pre-signed URLs. We are advising to use SDK for generating presigned URLs.

## Disable secure access

Disabling secured access is discouraged because it creates security vulnerabilities.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create({ secure: false }) // Explicitly disable
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create(secure=False)  # Explicitly disable
  ```
</CodeGroup>
