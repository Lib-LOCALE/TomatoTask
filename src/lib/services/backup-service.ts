import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';

/**
 * Exporte toutes les données de l'application vers un fichier JSON
 */
export async function exportBackup(): Promise<void> {
    try {
        // 1. Récupère les données depuis le backend
        const data = await invoke('export_data');
        const json = JSON.stringify(data, null, 2);

        // 2. Ouvre la boîte de dialogue de sauvegarde
        const filePath = await save({
            filters: [{
                name: 'JSON Backup',
                extensions: ['json']
            }],
            defaultPath: `tomatotask-backup-${new Date().toISOString().split('T')[0]}.json`
        });

        if (filePath) {
            // 3. Écrit le fichier
            await writeTextFile(filePath, json);
            return Promise.resolve();
        }
    } catch (error) {
        console.error('Failed to export backup:', error);
        throw error;
    }
}

/**
 * Importe un fichier de sauvegarde JSON
 * ATTENTION: Écrase les données existantes
 */
export async function importBackup(): Promise<void> {
    try {
        // 1. Ouvre la boîte de dialogue d'ouverture
        const filePath = await open({
            filters: [{
                name: 'JSON Backup',
                extensions: ['json']
            }],
            multiple: false
        });

        if (filePath) {
            // 2. Lit le fichier
            const json = await readTextFile(filePath as string);
            const data = JSON.parse(json);

            // 3. Envoie au backend pour import
            await invoke('import_data', { data });

            // 4. Recharge l'application pour rafraîchir les stores
            window.location.reload();
        }
    } catch (error) {
        console.error('Failed to import backup:', error);
        throw error;
    }
}
