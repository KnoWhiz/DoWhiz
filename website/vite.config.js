import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'
import fs from 'fs'

// Middleware to serve static HTML pages from public folder
function servePublicHtml() {
  return {
    name: 'serve-public-html',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const url = req.url?.split('?')[0] || ''

        // Check if this is a path that should serve a static HTML file
        const staticPaths = [
          '/agents/', '/blog/', '/privacy/', '/terms/', '/user-guide/', '/auth/'
        ]

        const isStaticPath = staticPaths.some(p => url.startsWith(p))

        if (isStaticPath) {
          // Try to serve index.html from the public folder
          let filePath = url.endsWith('/') ? url + 'index.html' : url
          if (!filePath.endsWith('.html') && !filePath.includes('.')) {
            filePath = filePath + '/index.html'
          }

          const fullPath = resolve(__dirname, 'public' + filePath)

          if (fs.existsSync(fullPath)) {
            res.setHeader('Content-Type', 'text/html')
            fs.createReadStream(fullPath).pipe(res)
            return
          }
        }

        next()
      })
    }
  }
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), servePublicHtml()],
})
