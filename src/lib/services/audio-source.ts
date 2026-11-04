// Résolution des fichiers audio embarqués en URL lisibles par le webview
//
// Sous Linux, WebKitGTK ne sait pas lire un <audio> servi par le protocole
// personnalisé de Tauri (tauri://localhost) : la lecture échoue avec
// MEDIA_ERR_SRC_NOT_SUPPORTED. On charge donc le fichier avec fetch() et on le
// sert au lecteur sous forme d'URL blob:, que tous les webviews savent lire.

const cache = new Map<string, Promise<string>>();

/**
 * Retourne une URL blob: pour un fichier audio du dossier public
 *
 * Le résultat est mis en cache : chaque fichier n'est chargé qu'une fois.
 *
 * @param path - Chemin du fichier (ex. '/sounds/birds.ogg')
 */
export function resolveAudioUrl(path: string): Promise<string> {
	let url = cache.get(path);
	if (!url) {
		url = fetch(path)
			.then((response) => {
				if (!response.ok) throw new Error(`Failed to load ${path}: HTTP ${response.status}`);
				return response.blob();
			})
			.then((blob) => URL.createObjectURL(blob));
		// En cas d'échec, on permet une nouvelle tentative au prochain appel
		url.catch(() => cache.delete(path));
		cache.set(path, url);
	}
	return url;
}

/**
 * Crée un élément audio prêt à jouer pour un fichier du dossier public
 */
export async function createAudio(path: string): Promise<HTMLAudioElement> {
	return new Audio(await resolveAudioUrl(path));
}
