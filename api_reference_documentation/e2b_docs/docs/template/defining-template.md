> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Defining template

> How to create your own template

### Method chaining

All template methods return the template instance, allowing for fluent API usage:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template()
    .fromUbuntuImage("22.04")
    .aptInstall(["curl"]) // necessary for waitForPort
    .setWorkdir('/app')
    .copy("package.json", "/app/package.json")
    .runCmd("npm install")
    .setStartCmd("npm start", waitForPort(3000));
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = (
      Template()
      .from_ubuntu_image("22.04")
      .set_workdir("/app")
      .copy("package.json", "/app/package.json")
      .run_cmd("npm install")
      .set_start_cmd("npm start", wait_for_timeout(10_000)))
  ```
</CodeGroup>

### User and workdir

Set the working directory and user for the template:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Set working directory
  template.setWorkdir("/app");

  // Set user (runs subsequent commands as this user)
  template.setUser('node')
  template.setUser("1000:1000"); // User ID and group ID
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Set working directory
  template.set_workdir("/app")

  # Set user (runs subsequent commands as this user)
  template.set_user("node")
  template.set_user("1000:1000")  # User ID and group ID
  ```
</CodeGroup>

### Copying files

Copy files from your local filesystem to the template:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Copy a single file
  template.copy("package.json", "/app/package.json");

  // Copy multiple files to same destination
  template.copy(["file1", "file2"], "/app/file")

  // Multiple copy operations using copyItems
  template.copyItems([
    { src: "src/", dest: "/app/src/" },
    { src: "package.json", dest: "/app/package.json" },
  ]);

  // Copy with user and mode options
  template.copy("config.json", "/app/config.json", {
    user: "appuser",
    mode: 0o644,
  });
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Copy a single file
  template.copy("package.json", "/app/package.json")

  # Copy multiple files to the same destination
  template.copy(["file1", "file2"], "/app/file")

  # Multiple copy operations using copy_items
  template.copy_items([
      {"src": "src/", "dest": "/app/src/"},
      {"src": "package.json", "dest": "/app/package.json"},
  ])

  # Copy with user and mode options
  template.copy("config.json", "/app/config.json", user="appuser", mode=0o644)
  ```
</CodeGroup>

### File operations

Perform various file operations during template build:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Remove files or directories
  template.remove("/tmp/temp-file.txt");
  template.remove("/old-directory", { recursive: true });
  template.remove("/file.txt", { force: true });

  // Rename files or directories
  template.rename("/old-name.txt", "/new-name.txt");
  template.rename("/old-dir", "/new-dir", { force: true });

  // Create directories
  template.makeDir("/app/logs");
  template.makeDir("/app/data", { mode: 0o755 });

  // Create symbolic links
  template.makeSymlink("/app/data", "/app/logs/data");
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Remove files or directories
  template.remove("/tmp/old-file")
  template.remove("/tmp/old-dir", recursive=True)
  template.remove("/tmp/file", force=True)  # Force removal

  # Rename files or directories
  template.rename("/old/path", "/new/path")
  template.rename("/old/path", "/new/path", force=True)  # Force rename

  # Create directories
  template.make_dir("/app/data")
  template.make_dir("/app/data", mode=0o755)  # Set permissions

  # Create symbolic links
  template.make_symlink("/path/to/target", "/path/to/link")
  ```
</CodeGroup>

### Installing packages

Install packages using package managers:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Install Python packages
  template.pipInstall(['requests', 'pandas', 'numpy'])

  // Install Python packages (user)
  template.pipInstall(['requests', 'pandas', 'numpy'], { g: false })

  // Install Node.js packages
  template.npmInstall(['express', 'lodash'])

  // Install Node.js packages (global)
  template.npmInstall(['express', 'lodash'], { g: true })

  // Install Bun packages
  template.bunInstall(['express', 'lodash'])

  // Install Bun packages (global)
  template.bunInstall(['express', 'lodash'], { g: true })

  // Install system packages (automatically runs apt update)
  template.aptInstall(['curl', 'wget', 'git'])
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Install Python packages
  template.pip_install(["requests", "pandas", "numpy"])

  # Install Python packages (user)
  template.pip_install(["requests", "pandas", "numpy"], g=False)

  # Install Node.js packages
  template.npm_install(["express", "lodash"])

  # Install Node.js packages (global)
  template.npm_install(["express", "lodash"], g=True)

  # Install Bun packages
  template.bun_install(["express", "lodash"])

  # Install Bun packages (global)
  template.bun_install(["express", "lodash"], g=True)

  # Install system packages (Ubuntu/Debian)
  template.apt_install(["curl", "wget", "git"])
  ```
</CodeGroup>

### Git operations

Clone Git repositories during template build (requires `git` to be installed):

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Clone a repository
  template.gitClone('https://github.com/user/repo.git')

  // Clone a repository to a specific path
  template.gitClone('https://github.com/user/repo.git', '/app/repo')

  // Clone a specific branch
  template.gitClone('https://github.com/user/repo.git', '/app/repo', {
    branch: 'main',
  })

  // Shallow clone with depth limit
  template.gitClone('https://github.com/user/repo.git', '/app/repo', {
    depth: 1,
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Clone a repository
  template.git_clone("https://github.com/user/repo.git")

  # Clone a repository to a specific path
  template.git_clone("https://github.com/user/repo.git", "/app/repo")

  # Clone a specific branch
  template.git_clone("https://github.com/user/repo.git", "/app/repo", branch="main")

  # Shallow clone with depth limit
  template.git_clone("https://github.com/user/repo.git", "/app/repo", depth=1)

  ```
</CodeGroup>

### Environment variables

<Warning>
  Environment variables set in template definition are only available during template build.
  [How to setup environment variables in sandbox?](/docs/sandbox/environment-variables)
</Warning>

Set environment variables in the template:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.setEnvs({
    NODE_ENV: 'production',
    API_KEY: 'your-api-key',
    DEBUG: 'true',
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template.set_envs({
      "NODE_ENV": "production",
      "API_KEY": "your-api-key",
      "DEBUG": "true",
  })
  ```
</CodeGroup>

### Running commands

Execute shell commands during template build:

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Run a single command
  template.runCmd('apt-get update && apt-get install -y curl')

  // Run multiple commands
  template.runCmd(['apt-get update', 'apt-get install -y curl', 'curl --version'])

  // Run commands as a specific user
  template.runCmd('npm install', { user: 'node' })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # Run a single command
  template.run_cmd("apt-get update && apt-get install -y curl")

  # Run multiple commands
  template.run_cmd(["apt-get update", "apt-get install -y curl", "curl --version"])

  # Run command as specific user
  template.run_cmd("npm install", user="node")
  ```
</CodeGroup>
