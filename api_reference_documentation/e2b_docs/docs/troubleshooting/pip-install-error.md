> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# pip install fails during template build

When building a template that installs large Python packages like PyTorch with CUDA dependencies, `pip install` can fail with one of two errors:

* **`MemoryError`** — pip tries to serialize large wheel files into memory for caching, exceeding available RAM.
* **`OSError: [Errno 28] No space left on device`** — downloaded wheels fill up the `/tmp` directory.

**Example errors**

```txt  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
File "pip/_vendor/cachecontrol/serialize.py", line 70, in dumps
    return b",".join([b"cc=4", msgpack.dumps(data, use_bin_type=True)])
MemoryError
```

```txt  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
ERROR: Could not install packages due to an OSError: [Errno 28] No space left on device
```

## Cause

The build environment mounts `/tmp` as a [tmpfs](https://www.kernel.org/doc/html/latest/filesystems/tmpfs.html) — a RAM-backed filesystem capped at \~3.9 GB. pip downloads all wheels to `/tmp/pip-*` before installing them. PyTorch with CUDA dependencies totals \~4.1 GB of downloads, which exceeds this limit.

## Solution 1: Redirect pip's temp directory to disk (recommended)

Set the `TMPDIR` environment variable to a disk-backed path so pip downloads don't go through the RAM-backed `/tmp`. Combined with `--no-cache-dir`, this avoids both the disk space and memory issues.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template()
    .runCmd('TMPDIR=/var/tmp pip install --no-cache-dir torch sentence-transformers')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = (
      Template()
      .run_cmd("TMPDIR=/var/tmp pip install --no-cache-dir torch sentence-transformers")
  )
  ```
</CodeGroup>

## Solution 2: Install CPU-only PyTorch

E2B sandboxes don't have GPUs, so there's no reason to download CUDA dependencies. Installing the CPU-only variant of PyTorch reduces the download from \~4.1 GB to \~189 MB, avoiding the `/tmp` size limit entirely.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  const template = Template()
    .runCmd('pip install --no-cache-dir torch --index-url https://download.pytorch.org/whl/cpu')
    .runCmd('echo "torch" > /tmp/constraints.txt && pip install --no-cache-dir -c /tmp/constraints.txt sentence-transformers')
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  template = (
      Template()
      .run_cmd("pip install --no-cache-dir torch --index-url https://download.pytorch.org/whl/cpu")
      .run_cmd('echo "torch" > /tmp/constraints.txt && pip install --no-cache-dir -c /tmp/constraints.txt sentence-transformers')
  )
  ```
</CodeGroup>

The constraints file in the second step prevents pip from replacing the CPU-only torch with the CUDA version when installing packages that depend on PyTorch.
