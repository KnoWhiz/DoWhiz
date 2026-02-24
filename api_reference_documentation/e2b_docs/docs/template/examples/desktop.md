> ## Documentation Index
> Fetch the complete documentation index at: https://e2b.mintlify.app/llms.txt
> Use this file to discover all available pages before exploring further.

# Desktop

> Sandbox with Ubuntu Desktop and VNC access

This template creates a sandbox with a full Ubuntu 22.04 desktop environment, including the XFCE desktop, common applications, and VNC streaming for remote access. It's ideal for building AI agents that need to interact with graphical user interfaces.

The template includes:

* **Ubuntu 22.04** with XFCE desktop environment
* **VNC streaming** via [noVNC](https://novnc.com/) for browser-based access
* **Pre-installed applications**: LibreOffice, text editors, file manager, and common utilities
* **Automation tools**: [xdotool](https://github.com/jordansissel/xdotool) and [scrot](https://github.com/resurrecting-open-source-projects/scrot) for programmatic desktop control

## Template definition

The template installs the desktop environment, sets up VNC streaming via [x11vnc](https://github.com/LibVNC/x11vnc) and noVNC, and configures a startup script.

<CodeGroup>
  ```typescript JavaScript & TypeScript expandable theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // template.ts
  import { Template, waitForPort } from 'e2b'

  export const template = Template()
    .fromUbuntuImage('22.04')
    // Desktop environment and system utilities
    .runCmd([
      'yes | unminimize',
      'apt-get update',
      'apt-get install -y \
        xserver-xorg \
        xorg \
        x11-xserver-utils \
        xvfb \
        x11-utils \
        xauth \
        xfce4 \
        xfce4-goodies \
        util-linux \
        sudo \
        curl \
        git \
        wget \
        python3-pip \
        xdotool \
        scrot \
        ffmpeg \
        x11vnc \
        net-tools \
        netcat \
        x11-apps \
        libreoffice \
        xpdf \
        gedit \
        xpaint \
        tint2 \
        galculator \
        pcmanfm',
      'apt-get clean',
      'rm -rf /var/lib/apt/lists/*',
    ])
    // Streaming server setup
    .runCmd([
      'git clone --branch e2b-desktop https://github.com/e2b-dev/noVNC.git /opt/noVNC',
      'ln -s /opt/noVNC/vnc.html /opt/noVNC/index.html',
      'git clone --branch v0.12.0 https://github.com/novnc/websockify /opt/noVNC/utils/websockify',
    ])
    // Set default terminal
    .runCmd(
      'ln -sf /usr/bin/xfce4-terminal.wrapper /etc/alternatives/x-terminal-emulator'
    )
    .copy('start_command.sh', '/start_command.sh')
    .runCmd('chmod +x /start_command.sh')
    .setStartCmd('/start_command.sh', waitForPort(6080))

  ```

  ```python Python expandable theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # template.py
  from e2b import Template, wait_for_port

  template = (
      Template()
      .from_image("ubuntu:22.04")
      # Initial system setup and packages
      # We are not using .apt_install() here because some packages have interactive prompts (keyboard layout setup, etc.)
      .run_cmd(
          [
              "yes | unminimize",
              "apt-get update",
              "apt-get install -y \
                  xserver-xorg \
                  xorg \
                  x11-xserver-utils \
                  xvfb \
                  x11-utils \
                  xauth \
                  xfce4 \
                  xfce4-goodies \
                  util-linux \
                  sudo \
                  curl \
                  git \
                  wget \
                  python3-pip \
                  xdotool \
                  scrot \
                  ffmpeg \
                  x11vnc \
                  net-tools \
                  netcat \
                  x11-apps \
                  libreoffice \
                  xpdf \
                  gedit \
                  xpaint \
                  tint2 \
                  galculator \
                  pcmanfm",
              "apt-get clean",
              "rm -rf /var/lib/apt/lists/*",
          ]
      )
      # Setup NoVNC and websockify
      .run_cmd(
          [
              "git clone --branch e2b-desktop https://github.com/e2b-dev/noVNC.git /opt/noVNC",
              "ln -s /opt/noVNC/vnc.html /opt/noVNC/index.html",
              "git clone --branch v0.12.0 https://github.com/novnc/websockify /opt/noVNC/utils/websockify",
          ]
      )
      # Set default terminal
      .run_cmd(
          "ln -sf /usr/bin/xfce4-terminal.wrapper /etc/alternatives/x-terminal-emulator"
      )
      # Copy the start command
      .copy("start_command.sh", "/start_command.sh")
      .run_cmd("chmod +x /start_command.sh")
      # Set start command to launch the desktop environment
      .set_start_cmd("/start_command.sh", wait_for_port(6080))
  )
  ```
</CodeGroup>

## Startup script

The startup script initializes the virtual display using [Xvfb](https://www.x.org/releases/X11R7.6/doc/man/man1/Xvfb.1.xhtml) (X Virtual Framebuffer), launches the XFCE desktop session, starts the VNC server, and exposes the desktop via noVNC on port 6080. This script runs automatically when the sandbox starts.

```bash start_command.sh theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
#!/bin/bash

# Set display
export DISPLAY=${DISPLAY:-:0}

# Start Xvfb
Xvfb $DISPLAY -ac -screen 0 1024x768x24 -nolisten tcp &
sleep 2

# Start XFCE session
startxfce4 &
sleep 5

# Start VNC server
x11vnc -bg -display $DISPLAY -forever -wait 50 -shared -rfbport 5900 -nopw \
    -noxdamage -noxfixes -nowf -noscr -ping 1 -repeat -speeds lan &
sleep 2

# Start noVNC server
cd /opt/noVNC/utils && ./novnc_proxy --vnc localhost:5900 --listen 6080 --web /opt/noVNC --heartbeat 30 &
sleep 2
```

## Building the template

Build the template with increased CPU and memory allocation to handle the desktop environment installation. The build process may take several minutes due to the size of the packages being installed.

<CodeGroup>
  ```typescript JavaScript & TypeScript theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  // build.ts
  import { Template, defaultBuildLogger } from 'e2b'
  import { template as desktopTemplate } from './template'

  await Template.build(desktopTemplate, 'desktop', {
    cpuCount: 8,
    memoryMB: 8192,
    onBuildLogs: defaultBuildLogger(),
  })
  ```

  ```python Python theme={"theme":{"light":"github-light","dark":"github-dark-default"}}
  # build.py
  from e2b import Template, default_build_logger
  from .template import template as desktopTemplate

  Template.build(desktopTemplate, 'desktop',
      cpu_count=8,
      memory_mb=8192,
      on_build_logs=default_build_logger(),
  )
  ```
</CodeGroup>
