# DoWhiz Website

Product website for DoWhiz, built with Vite + React.

## Prerequisites
- Node.js 18+ (20+ recommended).
- npm (ships with Node).

## Quick start (from repo root)
```
cd website
npm install
npm run dev
```

Open the local URL shown in the terminal (defaults to http://localhost:5173).

## Common commands (from website/)
- Dev server: `npm run dev`
- Lint: `npm run lint`
- Production build: `npm run build`
- Preview production build: `npm run preview`

Build output goes to `website/dist/`.

## VM Deployment Workflow

If you host the website on a VM (instead of Vercel), build the static assets and serve `website/dist` from Nginx.

```bash
cd website
npm install
npm run build
```

Nginx example:
```nginx
server {
    listen 80;
    server_name dowhiz.com www.dowhiz.com;

    return 301 https://dowhiz.com$request_uri;
}

server {
    listen 443 ssl;
    server_name dowhiz.com www.dowhiz.com;

    # SSL config omitted; use certbot or your provider's recommendations.
    root /home/azureuser/DoWhiz/website/dist;
    try_files $uri /index.html;
}
```

If you are using a dedicated API subdomain (example: `api.dowhiz.com`) for the Rust service, you can keep the website hosted on Vercel and only run the API on the VM.

## Project structure
- `website/src/`: React components and app code.
- `website/public/`: Static assets copied as-is.
- `website/index.html`: App entry HTML.
- `website/vite.config.js`: Vite configuration.
- `website/eslint.config.js`: Lint rules (source of truth).

## Environment variables
No `.env` file is required for local development at the moment.

## Troubleshooting
- Port 5173 in use: run `npm run dev -- --port 5174` or stop the other process.
- Clean install: remove `website/node_modules/` and run `npm install` again.
