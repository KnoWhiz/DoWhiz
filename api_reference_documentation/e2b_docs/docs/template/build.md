> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Build

> How to build the template

## Build and wait for completion

The `build` method builds the template and waits for the build to complete. It returns build information including the template ID and build ID.

<CodeGroup>
  ```typescript JavaScript & TypeScript wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const buildInfo = await Template.build(template, 'my-template', {
    cpuCount: 2, // CPU cores
    memoryMB: 2048, // Memory in MB
    skipCache: false, // Configure cache skip (except for files)
    onBuildLogs: defaultBuildLogger(), // Log callback receives LogEntry objects
    apiKey: 'your-api-key', // Override API key
    domain: 'your-domain', // Override domain
  })

  // buildInfo contains: { name, templateId, buildId }
  ```

  ```python Python wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  build_info = Template.build(
      template,
      'my-template',
      cpu_count=2,  # CPU cores
      memory_mb=2048,  # Memory in MB
      skip_cache=False,  # Configure cache skip (except for files)
      on_build_logs=default_build_logger(),  # Log callback receives LogEntry objects
      api_key="your-api-key",  # Override API key
      domain="your-domain",  # Override domain
  )

  # build_info contains: BuildInfo(name, template_id, build_id)
  ```
</CodeGroup>

## Build in background

The `buildInBackground` method starts the build process and returns immediately without waiting for completion. This is useful when you want to trigger a build and check its status later.

<CodeGroup>
  ```typescript JavaScript & TypeScript wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const buildInfo = await Template.buildInBackground(template, 'my-template', {
    cpuCount: 2,
    memoryMB: 2048,
  })

  // Returns immediately with: { name, templateId, buildId }
  ```

  ```python Python wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  build_info = Template.build_in_background(
      template,
      'my-template',
      cpu_count=2,
      memory_mb=2048,
  )

  # Returns immediately with: BuildInfo(name, template_id, build_id)
  ```
</CodeGroup>

## Check build status

Use `getBuildStatus` to check the status of a build started with `buildInBackground`.

<CodeGroup>
  ```typescript JavaScript & TypeScript wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const status = await Template.getBuildStatus(buildInfo, {
    logsOffset: 0, // Optional: offset for fetching logs
  })

  // status contains: { status: 'building' | 'ready' | 'error', logEntries: [...] }
  ```

  ```python Python wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  status = Template.get_build_status(
      build_info,
      logs_offset=0,  # Optional: offset for fetching logs
  )

  # status contains build status and logs
  ```
</CodeGroup>

## Example: Background build with status polling

<CodeGroup>
  ```typescript JavaScript & TypeScript wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Start build in background
  const buildInfo = await Template.buildInBackground(template, 'my-template', {
    cpuCount: 2,
    memoryMB: 2048,
  })

  // Poll for build status
  let logsOffset = 0
  let status = 'building'

  while (status === 'building') {
    const buildStatus = await Template.getBuildStatus(buildInfo, {
      logsOffset,
    })

    logsOffset += buildStatus.logEntries.length
    status = buildStatus.status

    buildStatus.logEntries.forEach(
      (logEntry) => console.log(logEntry.toString())
    )

    // Wait for a short period before checking the status again
    await new Promise(resolve => setTimeout(resolve, 2000))
  }

  if (status === 'ready') {
    console.log('Build completed successfully')
  } else {
    console.error('Build failed')
  }
  ```

  ```python Python wrap theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Start build in background
  build_info = Template.build_in_background(
      template,
      'my-template',
      cpu_count=2,
      memory_mb=2048,
  )

  # Poll for build status
  import time

  logs_offset = 0
  status = "building"

  while status == "building":
      build_status = Template.get_build_status(
          build_info,
          logs_offset=logs_offset,
      )

      logs_offset += len(build_status.log_entries)
      status = build_status.status.value

      for log_entry in build_status.log_entries:
          print(log_entry)

      # Wait for a short period before checking the status again
      time.sleep(2)

  if status == "ready":
      print("Build completed successfully")
  else:
      print("Build failed")
  ```
</CodeGroup>
