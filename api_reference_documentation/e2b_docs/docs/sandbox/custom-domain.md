> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Custom domain

How to set up a custom domain for Sandboxes hosted on E2B.

We will set up a GCP VM running Caddy server with Docker and Cloudflare DNS to proxy the requests to the Sandboxes.

<Tip>
  Example: `8080-sandboxid.mydomain.com` -> `8080-sandboxid.e2b.app`
</Tip>

### Prerequisites

* Domain name registered and configured with Cloudflare DNS.
* Cloudflare API Token that allows you to manage DNS records.

### GCP VM setup

1. Create a VM instance by running the following command:

   <Note>
     Replace `your-project-id` with your actual project ID.
   </Note>

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   gcloud compute instances create e2b-custom-domain-proxy \
       --project=your-project-id \
       --zone=us-west1-a \
       --machine-type=n2-standard-2 \
       --can-ip-forward \
       --tags=http-server,https-server \
       --image-project=debian-cloud \
       --image-family=debian-12 \
       --boot-disk-size=20GB
   ```

2. After the VM is created, you can connect to it using the following command:

   <Note>
     Replace `your-project-id` with your actual project ID.
   </Note>

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   # May take a few seconds until the instance is ready to accept SSH connections
   gcloud compute ssh e2b-custom-domain-proxy \
       --project=your-project-id \
       --zone=us-west1-a
   ```

### Server setup

1. Install the latest stable version of Docker:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   curl -fsSL https://get.docker.com | sudo sh
   ```

2. Create a Dockerfile that will be used to build the Caddy server image with Cloudflare DNS:

   ```Dockerfile Dockerfile theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   FROM caddy:builder AS builder
   RUN xcaddy build \
       --with github.com/caddy-dns/cloudflare

   FROM caddy:latest
   COPY --from=builder /usr/bin/caddy /usr/bin/caddy
   ```

3. Create a Docker Compose file that will be used to start the Caddy server:

   ```yaml docker-compose.yml theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   services:
       caddy:
           build:
               context: .
               dockerfile: Dockerfile
           container_name: caddy-proxy
           restart: unless-stopped
           ports:
               - "80:80"
               - "443:443"
               - "443:443/udp"  # Optional: HTTP/3
           volumes:
               - ./Caddyfile:/etc/caddy/Caddyfile
               - caddy_data:/data
               - caddy_config:/config
           environment:
               - CLOUDFLARE_API_TOKEN=${CLOUDFLARE_API_TOKEN}
           networks:
               - caddy_network

   volumes:
       caddy_data:
       caddy_config:

   networks:
       caddy_network:
           driver: bridge
   ```

4. Create a Caddyfile for proxying the requests to the Sandboxes:

   <Note>
     Replace `*.mydomain.com` with your actual wildcard domain name.
   </Note>

   ```Caddyfile Caddyfile theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   *.mydomain.com {
       # Use Cloudflare DNS for ACME challenge
       tls {
           dns cloudflare {env.CLOUDFLARE_API_TOKEN}
       }

       # Capture sandboxId for reuse
       # {labels.N} splits the host by "." and indexes from right to left:
       # e.g., for "abc123.mydomain.com":
       #   {labels.0} = "com"       (TLD)
       #   {labels.1} = "mydomain"  (domain)
       #   {labels.2} = "abc123"    (subdomain)
       vars sandboxId {labels.2}

       # Reverse proxy to corresponding e2b.app Sandbox
       reverse_proxy {vars.sandboxId}.e2b.app:443 {
           # Set the Host header to the e2b.app domain
           header_up Host {vars.sandboxId}.e2b.app

           # Forward real IP
           header_up X-Real-IP {remote_host}
           header_up X-Forwarded-For {remote_host}
           header_up X-Forwarded-Proto {scheme}
           header_up X-Forwarded-Host {host}

           # Use HTTPS to upstream
           transport http {
               tls
               tls_server_name {vars.sandboxId}.e2b.app
           }
       }

       # Optional: Add logging
       log {
           output file /var/log/caddy/access.log
           format json
       }
   }
   ```

5. Create a .env file that will be used to store the Cloudflare API Token:

   ```env .env theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   CLOUDFLARE_API_TOKEN=your-cloudflare-api-token
   ```

6. Build and start the Caddy server:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   docker compose build
   docker compose up -d
   ```

### Domain setup

Log into the Cloudflare dashboard and create a new A wildcard DNS record pointing to the IP address of the GCP VM.

<Note>
  Replace `GCP_VM_IP` with the IP address of the GCP VM.
</Note>

<Note>
  It may take a few minutes for the DNS record to propagate and for the certificate to be issued.
</Note>

<Note>
  If you have existing AAAA (IPv6) records for this domain name, make sure they are either removed or updated to point to the GCP VM.
</Note>

```txt DNS theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
*.mydomain.com A GCP_VM_IP
```

### Testing the setup

We will create a new Sandbox on E2B and install a simple HTTP server in it.

1. Create a new Sandbox using [E2B CLI](/docs/cli):

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   e2b sandbox create base
   ```

2. Install and run a simple HTTP server in the sandbox:

   ```bash  theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
   sudo apt install nginx
   sudo systemctl start nginx
   ```

3. Visit the sandbox URL in your browser: `https://80-sandboxid.mydomain.com`. You should see the default nginx welcome page.
