const { app, BrowserWindow } = require('electron');
const path = require('path');
const http = require('http');
const fs = require('fs');

const WEB_BUILD_DIR = path.join(__dirname, '..', 'dist');

const MIME_TYPES = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.svg': 'image/svg+xml',
  '.ico': 'image/x-icon',
  '.woff': 'font/woff',
  '.woff2': 'font/woff2',
  '.ttf': 'font/ttf',
};

function startStaticServer() {
  return new Promise((resolve, reject) => {
    const server = http.createServer((request, response) => {
      const requestedPath = decodeURIComponent(request.url.split('?')[0]);
      let filePath = path.join(WEB_BUILD_DIR, requestedPath === '/' ? 'index.html' : requestedPath);

      if (!filePath.startsWith(WEB_BUILD_DIR)) {
        response.writeHead(403);
        response.end('Forbidden');
        return;
      }

      fs.readFile(filePath, (error, data) => {
        if (error) {
          fs.readFile(path.join(WEB_BUILD_DIR, 'index.html'), (fallbackError, fallbackData) => {
            if (fallbackError) {
              response.writeHead(404);
              response.end('Not found');
              return;
            }
            response.writeHead(200, { 'Content-Type': 'text/html' });
            response.end(fallbackData);
          });
          return;
        }

        const contentType = MIME_TYPES[path.extname(filePath)] || 'application/octet-stream';
        response.writeHead(200, { 'Content-Type': contentType });
        response.end(data);
      });
    });

    server.listen(0, '127.0.0.1', () => resolve(server));
    server.on('error', reject);
  });
}

async function createWindow() {
  if (!fs.existsSync(WEB_BUILD_DIR)) {
    throw new Error(
      `No web build found at ${WEB_BUILD_DIR}. Run "npm run build:web" before "npm run electron".`
    );
  }

  const server = await startStaticServer();
  const { port } = server.address();

  const window = new BrowserWindow({
    width: 1200,
    height: 800,
    minWidth: 720,
    minHeight: 480,
    title: 'Notes App',
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  window.setMenuBarVisibility(false);
  await window.loadURL(`http://127.0.0.1:${port}/`);

  window.on('closed', () => server.close());
}

app.whenReady().then(() => {
  createWindow().catch((error) => {
    console.error(error);
    app.quit();
  });

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow().catch((error) => console.error(error));
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
