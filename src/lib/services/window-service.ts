import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

export class WindowService {
    private static readonly NORMAL_WIDTH = 1000;
    private static readonly NORMAL_HEIGHT = 800;
    private static readonly MINI_WIDTH = 300;
    private static readonly MINI_HEIGHT = 150;

    static async setMiniMode(enable: boolean): Promise<void> {
        const appWindow = getCurrentWindow();

        if (enable) {
            await appWindow.setSize(new LogicalSize(this.MINI_WIDTH, this.MINI_HEIGHT));
            await appWindow.setAlwaysOnTop(true);
        } else {
            await appWindow.setSize(new LogicalSize(this.NORMAL_WIDTH, this.NORMAL_HEIGHT));
            await appWindow.setAlwaysOnTop(false);
        }
    }

    static async toggleMiniMode(isMini: boolean): Promise<boolean> {
        const newState = !isMini;
        await this.setMiniMode(newState);
        return newState;
    }
}
