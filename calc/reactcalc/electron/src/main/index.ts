import { app, BrowserWindow } from 'electron';
import path from 'node:path';
import { serveStaticDir } from './staticServer';

// Set only for the live-reload dev workflow (Metro dev server). Unset means
// load the static export instead, served locally so its absolute asset
// paths (e.g. /_expo/static/...) resolve correctly.
const devServerUrl = process.env.EXPO_WEB_URL;
const staticDir = app.isPackaged
  ? path.join(process.resourcesPath, 'web-build')
  : path.join(__dirname, '../../../web-build');

async function createWindow(): Promise<void> {
  const win = new BrowserWindow({
    width: 380,
    height: 620,
    minWidth: 320,
    minHeight: 480,
    title: 'React Native Calculator',
    autoHideMenuBar: true,
  });

  if (devServerUrl) {
    const loadDevServer = () => win.loadURL(devServerUrl).catch(() => {});
    win.webContents.on('did-fail-load', () => setTimeout(loadDevServer, 1000));
    loadDevServer();
  } else {
    const { url } = await serveStaticDir(staticDir);
    win.loadURL(url);
  }
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});
