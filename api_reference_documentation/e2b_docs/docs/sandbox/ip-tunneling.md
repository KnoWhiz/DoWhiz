> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Proxy tunneling

> How to tunnel Sandbox network traffic through a proxy server

## Setting up proxy tunneling

We will set up a proxy server on a GCP VM instance running [Shadowsocks](https://shadowsocks.org) and use it to tunnel the Sandbox network traffic.
This will allow you to use a dedicated IP address for outgoing requests.

### GCP VM setup

1. Create a firewall rule to allow all tcp/udp traffic to port 8388.

   <Note>
     Replace `your-project-id` with your actual project ID.
   </Note>

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   gcloud compute firewall-rules create shadowsocks \
       --project=your-project-id \
       --direction=INGRESS \
       --priority=1000 \
       --network=default \
       --action=ALLOW \
       --rules=tcp:8388,udp:8388 \
       --source-ranges=0.0.0.0/0 \
       --target-tags=allow-shadowsocks
   ```

2. Create a VM instance with the following tags: `allow-shadowsocks`.

   <Note>
     Replace `your-project-id` with your actual project ID.
   </Note>

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   gcloud compute instances create shadowsocks-vm \
       --project=your-project-id \
       --zone=us-west1-a \
       --machine-type=n2-standard-2 \
       --can-ip-forward \
       --tags=allow-shadowsocks \
       --image-project=debian-cloud \
       --image-family=debian-12 \
       --boot-disk-size=20GB
   ```

3. After the VM is created, you can connect to it using the following command:

   <Note>
     Replace `your-project-id` with your actual project ID.
   </Note>

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   # May take a few seconds until the instance is ready to accept SSH connections
   gcloud compute ssh shadowsocks-vm \
       --project=your-project-id \
       --zone=us-west1-a
   ```

### Shadowsocks server setup (VM)

SSH into the VM and follow the instructions below to install and configure Shadowsocks.

1. Install the necessary packages, if missing:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   sudo apt update
   sudo apt install -y wget tar
   ```

2. Download and install Shadowsocks (v1.24.0):

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   wget https://github.com/shadowsocks/shadowsocks-rust/releases/latest/download/shadowsocks-v1.24.0.x86_64-unknown-linux-gnu.tar.xz
   tar -xf shadowsocks-*.tar.xz
   sudo mv ssserver ssmanager ssurl /usr/local/bin/
   ```

3. Create a shadowsocks configuration file:

   <Note>
     Replace `STRONG_PASSWORD_HERE` with your own password.
   </Note>

   ```json /etc/shadowsocks/server.json theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   {
       "server": "0.0.0.0",
       "server_port": 8388,
       "password": "STRONG_PASSWORD_HERE",
       "method": "aes-256-gcm",
       "mode": "tcp_and_udp",
       "timeout": 300,
       "fast_open": true
   }
   ```

4. Enable IP forwarding:

   ```txt /etc/sysctl.d/99-shadowsocks.conf theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   net.core.default_qdisc=fq
   net.ipv4.tcp_congestion_control=bbr
   net.ipv4.ip_forward=1
   ```

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   sudo sysctl -p /etc/sysctl.d/99-shadowsocks.conf
   ```

   Optional: Update the Ubuntu Firewall rules to allow traffic to port 8388:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   sudo ufw allow 8388/tcp
   sudo ufw allow 8388/udp
   ```

5. Start the Shadowsocks server:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   sudo ssserver -c /etc/shadowsocks/server.json -v
   ```

   You should see the following in the console output:

   ```
   shadowsocks tcp server listening on 0.0.0.0:8388
   shadowsocks udp server listening on 0.0.0.0:8388
   ```

6. Optional: Create a systemd service to start the Shadowsocks server on boot:

   ```ini /etc/systemd/system/ssserver.service theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   [Unit]
   Description=Shadowsocks Rust Server
   After=network.target

   [Service]
   ExecStart=/usr/local/bin/ssserver -c /etc/shadowsocks/server.json
   Restart=always
   LimitNOFILE=51200
   User=root

   [Install]
   WantedBy=multi-user.target
   ```

   Reload the systemd daemon and start the service:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   sudo systemctl daemon-reload
   sudo systemctl enable --now ssserver
   ```

   You can check the status of the service with the following command:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   sudo systemctl status ssserver
   ```

### Shadowsocks client setup (sandbox)

Create a custom Sandbox template that uses the shadowsocks client to tunnel TCP traffic through the proxy server we set up above.

<Tabs>
  <Tab title="Local proxy">
    Route only designated traffic through the proxy.

    1. Create a configuration file for the shadowsocks client:

       <Note>
         Replace `SERVER_IP` with the IP address of the proxy server and `STRONG_PASSWORD_HERE` with your own password.
       </Note>

       ```json config.json theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
       {
           "server": "SERVER_IP",
           "server_port": 8388,
           "password": "STRONG_PASSWORD_HERE",
           "method": "aes-256-gcm",
           "local_address": "0.0.0.0",
           "local_port": 1080,
           "mode": "tcp"
       }
       ```

    2. Create a template file (`template.ts` / `template.py`):

       <CodeGroup>
         ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         import { Template, waitForPort } from "e2b";

         const shadowSocksVersion = "1.24.0"
         const downloadUrl = `https://github.com/shadowsocks/shadowsocks-rust/releases/latest/download/shadowsocks-v${shadowSocksVersion}.x86_64-unknown-linux-gnu.tar.xz`

         export const template = Template()
             .fromBaseImage()
             .runCmd([
                 `wget ${downloadUrl}`,
                 "tar -xf shadowsocks-*.tar.xz",
                 "sudo mv sslocal /usr/local/bin/"
             ])
             .copy('config.json', 'config.json')
             .setStartCmd(
                 "sudo sslocal -c config.json --daemonize",
                 waitForPort(1080)
             )
         ```

         ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         from e2b import Template, wait_for_port

         shadowsocks_version = "1.24.0"
         download_url = f"https://github.com/shadowsocks/shadowsocks-rust/releases/latest/download/shadowsocks-v{shadowsocks_version}.x86_64-unknown-linux-gnu.tar.xz"

         template = (
             Template()
             .from_base_image()
             .run_cmd([
                 f"wget {download_url}",
                 "tar -xf shadowsocks-*.tar.xz",
                 "sudo mv sslocal /usr/local/bin/"
             ])
             .copy("config.json", "config.json")
             .set_start_cmd(
                 "sudo sslocal -c config.json --daemonize",
                 wait_for_port(1080)
             )
         )
         ```
       </CodeGroup>

    3. Create a build script (`build.ts` / `build.py`):

       <CodeGroup>
         ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         import { Template, defaultBuildLogger } from "e2b";
         import { template as localProxyTemplate } from "./template";

         await Template.build(template, {
             alias: 'shadowsocks-client',
             memoryMB: 1024,
             cpuCount: 1,
             onBuildLogs: defaultBuildLogger()
         })
         ```

         ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         from e2b import Template, default_build_logger
         from .template import template

         Template.build(
             template,
             alias="shadowsocks-client",
             memory_mb=1024,
             cpu_count=1,
             on_build_logs=default_build_logger()
         )
         ```
       </CodeGroup>

    4. Build the template using the build script:

       <CodeGroup>
         ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         npx tsx build.ts
         ```

         ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         python build.py
         ```
       </CodeGroup>
  </Tab>

  <Tab title="Transparent proxy">
    Route all traffic through the proxy.

    1. Create a configuration file for the shadowsocks client:

       <Note>
         Replace `SERVER_IP` with the IP address of the proxy server and `STRONG_PASSWORD_HERE` with your own password.
       </Note>

       ```json config.json theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
       {
           "server": "SERVER_IP",
           "server_port": 8388,
           "password": "STRONG_PASSWORD_HERE",
           "method": "aes-256-gcm",
           "local_address": "0.0.0.0",
           "local_port": 1080,
           "mode": "tcp"
       }
       ```

    2. Create a script for setting up the iptables rules:

       <Note>
         Replace `SERVER_IP` with the IP address of the proxy server.
       </Note>

       ```bash iptables-rules.sh theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
       iptables -t nat -N SHADOWSOCKS
       iptables -t nat -A SHADOWSOCKS -d SERVER_IP -j RETURN
       iptables -t nat -A SHADOWSOCKS -d 0.0.0.0/8 -j RETURN
       iptables -t nat -A SHADOWSOCKS -d 127.0.0.0/8 -j RETURN
       iptables -t nat -A SHADOWSOCKS -d 169.254.0.0/16 -j RETURN
       iptables -t nat -A SHADOWSOCKS -p tcp -j REDIRECT --to-ports 12345
       iptables -t nat -A OUTPUT -p tcp -j SHADOWSOCKS
       ```

    3. Create a template file (`template.ts` / `template.py`):

       <CodeGroup>
         ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         import { Template, waitForPort } from "e2b";

         const shadowsocksVersion = "1.24.0"
         const downloadUrl = `https://github.com/shadowsocks/shadowsocks-rust/releases/latest/download/shadowsocks-v${shadowsocksVersion}.x86_64-unknown-linux-gnu.tar.xz`

         export const template = Template()
             .fromBaseImage()
             .aptInstall("iptables")
             .runCmd([
                 `wget ${downloadUrl}`,
                 "tar -xf shadowsocks-*.tar.xz",
                 "sudo mv sslocal /usr/local/bin/"
             ])
             .copy('config.json', 'config.json')
             .copy('iptables-rules.sh', 'iptables-rules.sh', { mode: 0o755 })
             .setStartCmd(
                 "sudo sslocal -c config.json --protocol redir -b 0.0.0.0:12345 --daemonize && sudo iptables-rules.sh",
                 waitForPort(1080)
             )
         ```

         ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         from e2b import Template, wait_for_port

         shadowsocks_version = "1.24.0"
         download_url = f"https://github.com/shadowsocks/shadowsocks-rust/releases/latest/download/shadowsocks-v{shadowsocks_version}.x86_64-unknown-linux-gnu.tar.xz"

         template = (
             Template()
             .from_base_image()
             .apt_install("iptables")
             .run_cmd([
                 f"wget {download_url}",
                 "tar -xf shadowsocks-*.tar.xz",
                 "sudo mv sslocal /usr/local/bin/"
             ])
             .copy("config.json", "config.json")
             .copy("iptables-rules.sh", "iptables-rules.sh", mode=0o755)
             .set_start_cmd(
                 "sudo sslocal -c config.json --protocol redir -b 0.0.0.0:12345 --daemonize && sudo iptables-rules.sh",
                 wait_for_port(1080)
             )
         )
         ```
       </CodeGroup>

    4. Create a build script (`build.ts` / `build.py`):

       <CodeGroup>
         ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         import { Template, defaultBuildLogger } from "e2b";
         import { template as localProxyTemplate } from "./template";

         await Template.build(template, {
             alias: 'shadowsocks-client',
             memoryMB: 1024,
             cpuCount: 1,
             onBuildLogs: defaultBuildLogger()
         })
         ```

         ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         from e2b import Template, default_build_logger
         from .template import template

         Template.build(
             template,
             alias="shadowsocks-client",
             memory_mb=1024,
             cpu_count=1,
             on_build_logs=default_build_logger()
         )
         ```
       </CodeGroup>

    5. Build the template using the build script:

       <CodeGroup>
         ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         npx tsx build.ts
         ```

         ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
         python build.py
         ```
       </CodeGroup>
  </Tab>
</Tabs>

## Using the proxies

Create a new Sandbox from the built template and run a curl command to verify that the traffic is routed through the proxy:

<Tabs>
  <Tab title="Local proxy">
    Only designated traffic should be routed through the proxy.

    <CodeGroup>
      ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      import { Sandbox } from "e2b";

      // create a new Sandbox from the built template
      const sbx = await Sandbox.create('shadowsocks-client')

      // run a command to curl the IP address of the proxy server
      const curl = await sbx.commands.run('curl --socks5 127.0.0.1:1080 https://ifconfig.me')
      console.log(curl.stdout)

      await sbx.kill()
      ```

      ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      from e2b import Sandbox

      # create a new Sandbox from the built template
      sbx = Sandbox.create('shadowsocks-client')

      # run a command to curl the IP address of the proxy server
      curl = sbx.commands.run('curl --socks5 127.0.0.1:1080 https://ifconfig.me')
      print(curl.stdout)

      sbx.kill()
      ```
    </CodeGroup>

    <CodeGroup>
      ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      npx tsx sandbox.ts
      ```

      ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      python sandbox.py
      ```
    </CodeGroup>

    You should see the IP address of the proxy server.
  </Tab>

  <Tab title="Transparent proxy">
    All traffic should be routed through the proxy.

    <CodeGroup>
      ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      import { Sandbox } from "e2b";

      // create a new Sandbox from the built template
      const sbx = await Sandbox.create('shadowsocks-client')

      // run a command to curl the IP address of the proxy server
      const curl = await sbx.commands.run('curl https://ifconfig.me')
      console.log(curl.stdout)

      await sbx.kill()
      ```

      ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      from e2b import Sandbox

      # create a new Sandbox from the built template
      sbx = Sandbox.create('shadowsocks-client')

      # run a command to curl the IP address of the proxy server
      curl = sbx.commands.run('curl https://ifconfig.me')
      print(curl.stdout)

      sbx.kill()
      ```
    </CodeGroup>

    <CodeGroup>
      ```bash JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      npx tsx sandbox.ts
      ```

      ```bash Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
      python sandbox.py
      ```
    </CodeGroup>

    You should see the IP address of the proxy server.
  </Tab>
</Tabs>
