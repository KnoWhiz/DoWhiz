> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Internet access

Every sandbox has access to the internet and can be reached by a public URL.

## Controlling internet access

You can control whether a sandbox has access to the internet by using the `allowInternetAccess` parameter when creating a sandbox. By default, internet access is enabled (`true`), but you can disable it for security-sensitive workloads.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  // Create sandbox with internet access enabled (default)
  const sandbox = await Sandbox.create({ allowInternetAccess: true })

  // Create sandbox without internet access
  const isolatedSandbox = await Sandbox.create({ allowInternetAccess: false })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  # Create sandbox with internet access enabled (default)
  sandbox = Sandbox.create(allow_internet_access=True)

  # Create sandbox without internet access
  isolated_sandbox = Sandbox.create(allow_internet_access=False)
  ```
</CodeGroup>

When internet access is disabled, the sandbox cannot make outbound network connections, which provides an additional layer of security for sensitive code execution.

<Note>
  Setting `allowInternetAccess` to `false` is equivalent to setting `network.denyOut` to `['0.0.0.0/0']` (denying all traffic).
</Note>

## Fine-grained network control

For more granular control over network access, you can use the `network` configuration option to specify allow and deny lists for outbound traffic.

### Allow and deny lists

You can specify IP addresses, CIDR blocks, or domain names that the sandbox is allowed to use:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, ALL_TRAFFIC } from '@e2b/code-interpreter'

  // Deny all traffic except specific IPs
  const sandbox = await Sandbox.create({
    network: {
      denyOut: [ALL_TRAFFIC],
      allowOut: ['1.1.1.1', '8.8.8.0/24']
    }
  })

  // Deny specific IPs only
  const restrictedSandbox = await Sandbox.create({
    network: {
      denyOut: ['8.8.8.8']
    }
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox, ALL_TRAFFIC

  # Deny all traffic except specific IPs
  sandbox = Sandbox.create(
      network={
          "deny_out": [ALL_TRAFFIC],
          "allow_out": ["1.1.1.1", "8.8.8.0/24"]
      }
  )

  # Deny specific IPs only
  restricted_sandbox = Sandbox.create(
      network={
          "deny_out": ["8.8.8.8"]
      }
  )
  ```
</CodeGroup>

### Domain-based filtering

You can allow traffic to specific domains by specifying hostnames in `allow out`. When using domain-based filtering, you must include `ALL_TRAFFIC` in `deny out` to block all other traffic. Domains are not supported in the `deny out` list.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, ALL_TRAFFIC } from '@e2b/code-interpreter'

  // Allow only traffic to google.com
  const sandbox = await Sandbox.create({
    network: {
      allowOut: ['google.com'],
      denyOut: [ALL_TRAFFIC]
    }
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox, ALL_TRAFFIC

  # Allow only traffic to google.com
  sandbox = Sandbox.create(
      network={
          "allow_out": ["google.com"],
          "deny_out": [ALL_TRAFFIC]
      }
  )
  ```
</CodeGroup>

<Note>
  When any domain is used, the default nameserver `8.8.8.8` is automatically allowed to ensure proper DNS resolution.
</Note>

You can also use wildcards to allow all subdomains of a domain:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, ALL_TRAFFIC } from '@e2b/code-interpreter'

  // Allow traffic to any subdomain of mydomain.com
  const sandbox = await Sandbox.create({
    network: {
      allowOut: ['*.mydomain.com'],
      denyOut: [ALL_TRAFFIC]
    }
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox, ALL_TRAFFIC

  # Allow traffic to any subdomain of mydomain.com
  sandbox = Sandbox.create(
      network={
          "allow_out": ["*.mydomain.com"],
          "deny_out": [ALL_TRAFFIC]
      }
  )
  ```
</CodeGroup>

You can combine domain names with IP addresses and CIDR blocks:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, ALL_TRAFFIC } from '@e2b/code-interpreter'

  // Allow traffic to specific domains and IPs
  const sandbox = await Sandbox.create({
    network: {
      allowOut: ['api.example.com', '*.github.com', '8.8.8.8'],
      denyOut: [ALL_TRAFFIC]
    }
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox, ALL_TRAFFIC

  # Allow traffic to specific domains and IPs
  sandbox = Sandbox.create(
      network={
          "allow_out": ["api.example.com", "*.github.com", "8.8.8.8"],
          "deny_out": [ALL_TRAFFIC]
      }
  )
  ```
</CodeGroup>

<Note>
  Domain-based filtering only works for HTTP traffic on port 80 (via Host header inspection) and TLS traffic on port 443 (via SNI inspection). Traffic on other ports uses CIDR-based filtering only. UDP-based protocols like QUIC/HTTP3 are not supported for domain filtering.
</Note>

### Priority rules

When both `allow out` and `deny out` are specified, **allow rules always take precedence** over deny rules. This means if an IP address is in both lists, it will be allowed.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, ALL_TRAFFIC } from '@e2b/code-interpreter'

  // Even though ALL_TRAFFIC is denied, 1.1.1.1 and 8.8.8.8 are explicitly allowed
  const sandbox = await Sandbox.create({
    network: {
      denyOut: [ALL_TRAFFIC],
      allowOut: ['1.1.1.1', '8.8.8.8']
    }
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox, ALL_TRAFFIC

  # Even though ALL_TRAFFIC is denied, 1.1.1.1 and 8.8.8.8 are explicitly allowed
  sandbox = Sandbox.create(
      network={
          "deny_out": [ALL_TRAFFIC],
          "allow_out": ["1.1.1.1", "8.8.8.8"]
      }
  )
  ```
</CodeGroup>

### ALL\_TRAFFIC helper

The `ALL_TRAFFIC` constant represents the CIDR range `0.0.0.0/0`, which matches all IP addresses. Use it to easily deny or allow all network traffic:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox, ALL_TRAFFIC } from '@e2b/code-interpreter'

  // Deny all outbound traffic
  const sandbox = await Sandbox.create({
    network: {
      denyOut: [ALL_TRAFFIC]
    }
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox, ALL_TRAFFIC

  # Deny all outbound traffic
  sandbox = Sandbox.create(
      network={
          "deny_out": [ALL_TRAFFIC]
      }
  )
  ```
</CodeGroup>

## Sandbox public URL

Every sandbox has a public URL that can be used to access running services inside the sandbox.

<CodeGroup>
  ```js JavaScript & TypeScript highlight={6} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  // You need to always pass a port number to get the host
  const host = sandbox.getHost(3000)
  console.log(`https://${host}`)
  ```

  ```python Python highlight={6} theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()

  # You need to always pass a port number to get the host
  host = sandbox.get_host(3000)
  print(f'https://{host}')
  ```
</CodeGroup>

The code above will print something like this:

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  https://3000-i62mff4ahtrdfdkyn2esc.e2b.app
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  https://3000-i62mff4ahtrdfdkyn2esc.e2b.app
  ```
</CodeGroup>

The first leftmost part of the host is the port number we passed to the method.

## Restricting public access to sandbox URLs

By default, sandbox URLs are publicly accessible. You can restrict access to require authentication using the `allowPublicTraffic` option:

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  // Create sandbox with restricted public access
  const sandbox = await Sandbox.create({
    network: {
      allowPublicTraffic: false
    }
  })

  // The sandbox has a traffic access token
  console.log(sandbox.trafficAccessToken)

  // Start a server inside the sandbox
  await sandbox.commands.run('python -m http.server 8080', { background: true })

  const host = sandbox.getHost(8080)
  const url = `https://${host}`

  // Request without token will fail with 403
  const response1 = await fetch(url)
  console.log(response1.status) // 403

  // Request with token will succeed
  const response2 = await fetch(url, {
    headers: {
      'e2b-traffic-access-token': sandbox.trafficAccessToken
    }
  })
  console.log(response2.status) // 200
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests
  from e2b_code_interpreter import Sandbox

  # Create sandbox with restricted public access
  sandbox = Sandbox.create(
      network={
          "allow_public_traffic": False
      }
  )

  # The sandbox has a traffic access token
  print(sandbox.traffic_access_token)

  # Start a server inside the sandbox
  sandbox.commands.run("python -m http.server 8080", background=True)

  host = sandbox.get_host(8080)
  url = f"https://{host}"

  # Request without token will fail with 403
  response1 = requests.get(url)
  print(response1.status_code)  # 403

  # Request with token will succeed
  response2 = requests.get(url, headers={
      'e2b-traffic-access-token': sandbox.traffic_access_token
  })
  print(response2.status_code)  # 200
  ```
</CodeGroup>

When `allowPublicTraffic` is set to `false`, all requests to the sandbox's public URLs must include the `e2b-traffic-access-token` header with the value from `sandbox.trafficAccessToken`.

## Connecting to a server running inside the sandbox

You can start a server inside the sandbox and connect to it using the approach above.

In this example we will start a simple HTTP server that listens on port 3000 and responds with the content of the directory where the server is started.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  const sandbox = await Sandbox.create()

  // Start a simple HTTP server inside the sandbox.
  const process = await sandbox.commands.run('python -m http.server 3000', { background: true })
  const host = sandbox.getHost(3000)
  const url = `https://${host}`
  console.log('Server started at:', url)

  // Fetch data from the server inside the sandbox.
  const response = await fetch(url);
  const data = await response.text();
  console.log('Response from server inside sandbox:', data);

  // Kill the server process inside the sandbox.
  await process.kill()
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import requests
  from e2b_code_interpreter import Sandbox

  sandbox = Sandbox.create()

  # Start a simple HTTP server inside the sandbox.
  process = sandbox.commands.run("python -m http.server 3000", background=True)
  host = sandbox.get_host(3000)
  url = f"https://{host}"
  print('Server started at:', url)

  # Fetch data from the server inside the sandbox.
  response = requests.get(url)
  data = response.text
  print('Response from server inside sandbox:', data)

  # Kill the server process inside the sandbox.
  process.kill()
  ```
</CodeGroup>

This output will look like this:

<CodeGroup>
  ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Server started at: https://3000-ip3nfrvajtqu5ktoxugc7.e2b.app
  Response from server inside sandbox: <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
  <html>
  <head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>Directory listing for /</title>
  </head>
  <body>
  <h1>Directory listing for /</h1>
  <hr>
  <ul>
  <li><a href=".bash_logout">.bash_logout</a></li>
  <li><a href=".bashrc">.bashrc</a></li>
  <li><a href=".profile">.profile</a></li>
  </ul>
  <hr>
  </body>
  </html>
  ```

  ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  Server started at: https://3000-ip3nfrvajtqu5ktoxugc7.e2b.app
  Response from server inside sandbox: <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
  <html>
  <head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <title>Directory listing for /</title>
  </head>
  <body>
  <h1>Directory listing for /</h1>
  <hr>
  <ul>
  <li><a href=".bash_logout">.bash_logout</a></li>
  <li><a href=".bashrc">.bashrc</a></li>
  <li><a href=".profile">.profile</a></li>
  </ul>
  <hr>
  </body>
  </html>
  ```
</CodeGroup>

## Masking request host headers

You can customize the `Host` header that gets sent to services running inside the sandbox using the `maskRequestHost` option. This is useful when your application expects a specific host format.

<CodeGroup>
  ```js JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  import { Sandbox } from '@e2b/code-interpreter'

  // Create sandbox with custom host masking
  const sandbox = await Sandbox.create({
    network: {
      maskRequestHost: 'localhost:${PORT}'
    }
  })

  // The ${PORT} variable will be replaced with the actual port number
  // Requests to the sandbox will have Host header set to for example: localhost:8080
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  from e2b_code_interpreter import Sandbox

  # Create sandbox with custom host masking
  sandbox = Sandbox.create(
      network={
          "mask_request_host": "localhost:${PORT}"
      }
  )

  # The ${PORT} variable will be replaced with the actual port number
  # Requests to the sandbox will have Host header set to for example: localhost:8080
  ```
</CodeGroup>

The `${PORT}` variable in the mask will be automatically replaced with the actual port number of the requested service.
