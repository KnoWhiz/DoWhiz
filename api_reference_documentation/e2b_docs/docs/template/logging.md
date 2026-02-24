> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Logging

> How to view logs from the template build

You can retrieve the build logs using the SDK.

## Default logger

We provide a default logger that you can use to filter logs by level:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Template, defaultBuildLogger } from 'e2b';

  await Template.build(template, 'my-template', {
    onBuildLogs: defaultBuildLogger({
      minLevel: "info", // Minimum log level to show
    }),
  });
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b import Template, default_build_logger

  Template.build(
      template,
      'my-template',
      on_build_logs=default_build_logger(
          min_level="info",  # Minimum log level to show
      )
  )
  ```
</CodeGroup>

## Custom logger

You can customize how logs are handled:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Simple logging
  onBuildLogs: (logEntry) => console.log(logEntry.toString());

  // Custom formatting
  onBuildLogs: (logEntry) => {
    const time = logEntry.timestamp.toISOString();
    console.log(`[${time}] ${logEntry.level.toUpperCase()}: ${logEntry.message}`);
  };

  // Filter by log level
  onBuildLogs: (logEntry) => {
    if (logEntry.level === "error" || logEntry.level === "warn") {
      console.error(logEntry.toString());
    }
  };
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Simple logging
  on_build_logs=lambda log_entry: print(log_entry)

  # Custom formatting
  def custom_logger(log_entry):
      time = log_entry.timestamp.isoformat()
      print(f"[{time}] {log_entry.level.upper()}: {log_entry.message}")

  Template.build(template, 'my-template', on_build_logs=custom_logger)

  # Filter by log level
  def error_logger(log_entry):
      if log_entry.level in ["error", "warn"]:
          print(f"ERROR/WARNING: {log_entry}", file=sys.stderr)

  Template.build(template, 'my-template', on_build_logs=error_logger)
  ```
</CodeGroup>

The `onBuildLogs`/`on_build_logs` callback receives structured `LogEntry` objects with the following properties:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  type LogEntryLevel = 'debug' | 'info' | 'warn' | 'error'

  class LogEntry {
    constructor(
      public readonly timestamp: Date,
      public readonly level: LogEntryLevel,
      public readonly message: string
    )

    toString() // Returns formatted log string
  }

  // Indicates the start of the build process
  class LogEntryStart extends LogEntry {
    constructor(timestamp: Date, message: string) {
      super(timestamp, 'debug', message)
    }
  }

  // Indicates the end of the build process
  class LogEntryEnd extends LogEntry {
    constructor(timestamp: Date, message: string) {
      super(timestamp, 'debug', message)
    }
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  LogEntryLevel = Literal["debug", "info", "warn", "error"]

  @dataclass
  class LogEntry:
      timestamp: datetime
      level: LogEntryLevel
      message: str

      def __str__(self) -> str:  # Returns formatted log string


  # Indicates the start of the build process
  @dataclass
  class LogEntryStart(LogEntry):
      level: LogEntryLevel = field(default="debug", init=False)

  # Indicates the end of the build process
  @dataclass
  class LogEntryEnd(LogEntry):
      level: LogEntryLevel = field(default="debug", init=False)
  ```
</CodeGroup>

Together with the `LogEntry` type, there are also `LogEntryStart` and `LogEntryEnd` types that indicate the start and end of the build process. Their default log level is `debug` and you can use them to like this:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  if (logEntry instanceof LogEntryStart) {
    // Build started
    return
  }

  if (logEntry instanceof LogEntryEnd) {
    // Build ended
    return
  }
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  if isinstance(log_entry, LogEntryStart):
      # Build started
      return

  if isinstance(log_entry, LogEntryEnd):
      # Build ended
      return

  ```
</CodeGroup>
