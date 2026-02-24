> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Error handling

> Handle errors in your template

The SDK provides specific error types:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { AuthError, BuildError, FileUploadError } from 'e2b';

  try {
    await Template.build(template, 'my-template');
  } catch (error) {
    if (error instanceof AuthError) {
      console.error("Authentication failed:", error.message);
    } else if (error instanceof FileUploadError) {
      console.error("File upload failed:", error.message);
    } else if (error instanceof BuildError) {
      console.error("Build failed:", error.message);
    }
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import AuthError, BuildError, FileUploadError

  try:
      Template.build(template, 'my-template')
  except AuthError as error:
      print(f"Authentication failed: {error}")
  except FileUploadError as error:
      print(f"File upload failed: {error}")
  except BuildError as error:
      print(f"Build failed: {error}")
  ```
</CodeGroup>
