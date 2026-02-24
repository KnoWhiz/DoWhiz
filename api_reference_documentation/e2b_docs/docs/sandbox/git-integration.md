> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Git integration

> Clone repositories, manage branches, and push changes using the sandbox.git methods.

Use the `sandbox.git` methods to run common git operations inside a sandbox.

### Authentication and Identity

#### Passing credentials inline

For private repositories over HTTP(S), pass `username` and `password` (token) directly to commands that need authentication. A username is required whenever you pass a password/token.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'

  await sandbox.git.push(repoPath, {
    username: process.env.GIT_USERNAME,
    password: process.env.GIT_TOKEN,
  })

  await sandbox.git.pull(repoPath, {
    username: process.env.GIT_USERNAME,
    password: process.env.GIT_TOKEN,
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os

  repo_path = "/home/user/repo"

  sandbox.git.push(
      repo_path,
      username=os.environ.get("GIT_USERNAME"),
      password=os.environ.get("GIT_TOKEN"),
  )

  sandbox.git.pull(
      repo_path,
      username=os.environ.get("GIT_USERNAME"),
      password=os.environ.get("GIT_TOKEN"),
  )
  ```
</CodeGroup>

#### Credential helper (authenticate once)

To avoid passing credentials on each command, store them in the git credential helper inside the sandbox using `dangerouslyAuthenticate()` / `dangerously_authenticate()`.

<Warning>
  Stores credentials on disk inside the sandbox. Any process or agent with access to the sandbox can read them. Use only when you understand the risk.
</Warning>

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Default (GitHub)
  await sandbox.git.dangerouslyAuthenticate({
    username: process.env.GIT_USERNAME,
    password: process.env.GIT_TOKEN,
  })

  // Custom host (self-hosted)
  await sandbox.git.dangerouslyAuthenticate({
    username: process.env.GIT_USERNAME,
    password: process.env.GIT_TOKEN,
    host: 'git.example.com',
    protocol: 'https',
  })

  // After this, HTTPS git operations use the stored credentials
  await sandbox.git.clone('https://git.example.com/org/repo.git', { path: '/home/user/repo' })
  await sandbox.git.push('/home/user/repo')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os

  # Default (GitHub)
  sandbox.git.dangerously_authenticate(
      username=os.environ.get("GIT_USERNAME"),
      password=os.environ.get("GIT_TOKEN"),
  )

  # Custom host (self-hosted)
  sandbox.git.dangerously_authenticate(
      username=os.environ.get("GIT_USERNAME"),
      password=os.environ.get("GIT_TOKEN"),
      host="git.example.com",
      protocol="https",
  )

  # After this, HTTPS git operations use the stored credentials
  sandbox.git.clone("https://git.example.com/org/repo.git", path="/home/user/repo")
  sandbox.git.push("/home/user/repo")
  ```
</CodeGroup>

#### Keep credentials in the remote URL

By default, credentials are stripped from the remote URL after cloning. To keep credentials in the remote URL (stored in `.git/config`), set `dangerouslyStoreCredentials` / `dangerously_store_credentials`.

<Warning>
  Storing credentials in the remote URL persists them in the repo config. Any process or agent with access to the sandbox can read them. Only use this when required.
</Warning>

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // Default: credentials are stripped from the remote URL
  await sandbox.git.clone('https://git.example.com/org/repo.git', {
    path: '/home/user/repo',
    username: process.env.GIT_USERNAME,
    password: process.env.GIT_TOKEN,
  })

  // Keep credentials in the remote URL
  await sandbox.git.clone('https://git.example.com/org/repo.git', {
    path: '/home/user/repo',
    username: process.env.GIT_USERNAME,
    password: process.env.GIT_TOKEN,
    dangerouslyStoreCredentials: true,
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import os

  # Default: credentials are stripped from the remote URL
  sandbox.git.clone(
      "https://git.example.com/org/repo.git",
      path="/home/user/repo",
      username=os.environ.get("GIT_USERNAME"),
      password=os.environ.get("GIT_TOKEN"),
  )

  # Keep credentials in the remote URL
  sandbox.git.clone(
      "https://git.example.com/org/repo.git",
      path="/home/user/repo",
      username=os.environ.get("GIT_USERNAME"),
      password=os.environ.get("GIT_TOKEN"),
      dangerously_store_credentials=True,
  )
  ```
</CodeGroup>

#### Configure git identity

Set the git author name and email for commits. Configure globally or per-repository.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'

  // Global config
  await sandbox.git.configureUser('E2B Bot', 'bot@example.com')

  // Repo-local config
  await sandbox.git.configureUser('E2B Bot', 'bot@example.com', {
    scope: 'local',
    path: repoPath
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_path = "/home/user/repo"

  # Global config
  sandbox.git.configure_user("E2B Bot", "bot@example.com")

  # Repo-local config
  sandbox.git.configure_user(
      "E2B Bot",
      "bot@example.com",
      scope="local",
      path=repo_path
  )
  ```
</CodeGroup>

### Clone a repository

See [Authentication and Identity](#authentication-and-identity) for how to authenticate with private repositories.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoUrl = 'https://git.example.com/org/repo.git'
  const repoPath = '/home/user/repo'

  // Default clone
  await sandbox.git.clone(repoUrl, { path: repoPath })

  // Clone a specific branch
  await sandbox.git.clone(repoUrl, { path: repoPath, branch: 'main' })

  // Shallow clone
  await sandbox.git.clone(repoUrl, { path: repoPath, depth: 1 })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_url = "https://git.example.com/org/repo.git"
  repo_path = "/home/user/repo"

  # Default clone
  sandbox.git.clone(repo_url, path=repo_path)

  # Clone a specific branch
  sandbox.git.clone(repo_url, path=repo_path, branch="main")

  # Shallow clone
  sandbox.git.clone(repo_url, path=repo_path, depth=1)
  ```
</CodeGroup>

### Check status and branches

`status()` returns a structured object with branch, ahead/behind, and file status details. `branches()` returns the branch list and the current branch.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'

  const status = await sandbox.git.status(repoPath)
  console.log(status.currentBranch, status.ahead, status.behind)
  console.log(status.fileStatus)

  const branches = await sandbox.git.branches(repoPath)
  console.log(branches.currentBranch)
  console.log(branches.branches)
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_path = "/home/user/repo"

  status = sandbox.git.status(repo_path)
  print(status.current_branch, status.ahead, status.behind)
  print(status.file_status)

  branches = sandbox.git.branches(repo_path)
  print(branches.current_branch)
  print(branches.branches)
  ```
</CodeGroup>

### Create and manage branches

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'

  // Create and switch to a new branch
  await sandbox.git.createBranch(repoPath, 'feature/new-docs')

  // Check out an existing branch
  await sandbox.git.checkoutBranch(repoPath, 'main')

  // Delete a branch
  await sandbox.git.deleteBranch(repoPath, 'feature/old-docs')

  // Force delete a branch
  await sandbox.git.deleteBranch(repoPath, 'feature/stale-docs', { force: true })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_path = "/home/user/repo"

  # Create and switch to a new branch
  sandbox.git.create_branch(repo_path, "feature/new-docs")

  # Check out an existing branch
  sandbox.git.checkout_branch(repo_path, "main")

  # Delete a branch
  sandbox.git.delete_branch(repo_path, "feature/old-docs")

  # Force delete a branch
  sandbox.git.delete_branch(repo_path, "feature/stale-docs", force=True)
  ```
</CodeGroup>

### Stage and commit

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'

  // Default: stage all changes, commit with repo config
  await sandbox.git.add(repoPath)
  await sandbox.git.commit(repoPath, 'Initial commit')

  // Stage specific files
  await sandbox.git.add(repoPath, { files: ['README.md', 'src/index.ts'] })

  // Allow empty commit and override author
  await sandbox.git.commit(repoPath, 'Docs sync', {
    authorName: 'E2B Bot',
    authorEmail: 'bot@example.com',
    allowEmpty: true,
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_path = "/home/user/repo"

  # Default: stage all changes, commit with repo config
  sandbox.git.add(repo_path)
  sandbox.git.commit(repo_path, "Initial commit")

  # Stage specific files
  sandbox.git.add(repo_path, files=["README.md", "src/index.ts"])

  # Allow empty commit and override author
  sandbox.git.commit(
      repo_path,
      "Docs sync",
      author_name="E2B Bot",
      author_email="bot@example.com",
      allow_empty=True,
  )
  ```
</CodeGroup>

### Pull and push

See [Authentication and Identity](#authentication-and-identity) for how to authenticate with private repositories.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'

  // Default (uses upstream when set)
  await sandbox.git.push(repoPath)
  await sandbox.git.pull(repoPath)

  // Target a specific remote/branch and set upstream
  await sandbox.git.push(repoPath, {
    remote: 'origin',
    branch: 'main',
    setUpstream: true,
  })

  await sandbox.git.pull(repoPath, {
    remote: 'origin',
    branch: 'main',
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_path = "/home/user/repo"

  # Default (uses upstream when set)
  sandbox.git.push(repo_path)
  sandbox.git.pull(repo_path)

  # Target a specific remote/branch and set upstream
  sandbox.git.push(
      repo_path,
      remote="origin",
      branch="main",
      set_upstream=True,
  )

  sandbox.git.pull(
      repo_path,
      remote="origin",
      branch="main",
  )
  ```
</CodeGroup>

### Manage remotes

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'
  const repoUrl = 'https://git.example.com/org/repo.git'

  // Default
  await sandbox.git.remoteAdd(repoPath, 'origin', repoUrl)

  // Fetch after adding the remote
  await sandbox.git.remoteAdd(repoPath, 'origin', repoUrl, { fetch: true })

  // Overwrite the remote URL if it already exists
  await sandbox.git.remoteAdd(repoPath, 'origin', repoUrl, { overwrite: true })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_path = "/home/user/repo"
  repo_url = "https://git.example.com/org/repo.git"

  # Default
  sandbox.git.remote_add(repo_path, "origin", repo_url)

  # Fetch after adding the remote
  sandbox.git.remote_add(repo_path, "origin", repo_url, fetch=True)

  # Overwrite the remote URL if it already exists
  sandbox.git.remote_add(repo_path, "origin", repo_url, overwrite=True)
  ```
</CodeGroup>

### Git config

Set and get git configuration values. See [Configure git identity](#configure-git-identity) for configuring the commit author.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const repoPath = '/home/user/repo'

  // Global config
  await sandbox.git.setConfig('pull.rebase', 'false')
  const rebase = await sandbox.git.getConfig('pull.rebase')

  // Repo-local config
  await sandbox.git.setConfig('pull.rebase', 'false', { scope: 'local', path: repoPath })
  const localRebase = await sandbox.git.getConfig('pull.rebase', { scope: 'local', path: repoPath })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  repo_path = "/home/user/repo"

  # Global config
  sandbox.git.set_config("pull.rebase", "false")
  rebase = sandbox.git.get_config("pull.rebase")

  # Repo-local config
  sandbox.git.set_config("pull.rebase", "false", scope="local", path=repo_path)
  local_rebase = sandbox.git.get_config("pull.rebase", scope="local", path=repo_path)
  ```
</CodeGroup>
