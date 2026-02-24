> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Caching

> How the caching process works

The caching concept is similar to [Docker's layer caching](https://docs.docker.com/build/cache/). For each layer command (`.copy()`, `.runCmd()`, `.setEnvs()`, etc.), we create a new layer on top of the existing.
Each layer is cached based on the command and its inputs (e.g., files copied, command executed, environment variables set).
If a layer command is unchanged and its inputs are the same as in any previous build, we reuse the cached layer instead of rebuilding it.

This significantly speeds up the build process, especially for large templates with many layers.
The cache is scoped to the team, so even if you have multiple templates, they can share the same cache if they have identical layers.

## Invalidation

You can invalidate the caches only partially, or for the whole template.

### Partial

Force rebuild from the next instruction, use the method:

<CodeGroup>
  ```typescript JavaScript & TypeScript highlight={3} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template()
    .fromBaseImage()
    .skipCache()
    .runCmd("echo 'Hello, World!'")
  ```

  ```python Python highlight={4} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = (
      Template()
      .from_base_image()
      .skip_cache()
      .run_cmd("echo 'Hello, World!'")
  )
  ```
</CodeGroup>

This will force rebuild from the next instruction, invalidating the cache for all subsequent instructions in the template.

### Whole template

To force rebuild the whole template, you can use also `skipCache`/`skip_cache` parameter in the `Template.build` method:

<CodeGroup>
  ```typescript JavaScript & TypeScript wrap highlight={2} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Template.build(template, 'my-template', {
    skipCache: true, // Configure cache skip (except for files)
  })
  ```

  ```python Python wrap highlight={4} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Template.build(
      template,
      'my-template',
      skip_cache=True,  # Configure cache skip (except for files)
  )
  ```
</CodeGroup>

This will skip the cache usage for the whole template build.

## Files caching

When using the `.copy()` command, we cache the files based on their content. If the files haven't changed since the last build, we reuse them from the files cache.

We differ from Docker here. Because we build the template on our infrastructure, we use improved caching on files level.
Even if you invalidate the layer before `.copy()` (e.g., by changing ENV variables), we'll reuse the already uploaded files.
The `copy()` command will still be re-executed, but the files for the layer will be reused from the files cache, no need to upload them from your computer again.

To invalidate the cache for all subsequent instructions in the template **AND** the layer files cache, use the `forceUpload`/`force_upload` parameter.

<CodeGroup>
  ```typescript JavaScript & TypeScript highlight={3} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template()
    .fromBaseImage()
    .copy("config.json", "/app/config.json", { forceUpload: true })
  ```

  ```python Python highlight={4} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = (
      Template()
      .from_base_image()
      .copy("config.json", "/app/config.json", force_upload=True)
  )
  ```
</CodeGroup>

## Use case for caching

You can leverage caching to create templates with multiple variants (e.g., different RAM or CPU) while reusing the common layers.
When building the template, just change the template name to a specific RAM/CPU configuration (e.g., `my-template-2cpu-2gb`, `my-template-1cpu-4gb`), keep the rest of the template definition the same, and the build process will reuse the cached layers.

## Optimize build times

To optimize build times, place frequently changing commands (e.g., copying source code) towards the end of your template definition. This way, earlier layers can be cached and reused more often.
