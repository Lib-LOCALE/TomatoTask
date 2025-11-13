// Gestionnaire de raccourcis clavier

/**
 * Type pour les handlers de raccourcis clavier
 */
type KeyboardShortcutHandler = () => void;

/**
 * Map des raccourcis clavier enregistrés
 */
const shortcuts = new Map<string, KeyboardShortcutHandler>();

/**
 * Génère une clé unique pour un raccourci clavier
 *
 * @param key - Touche principale
 * @param ctrl - Ctrl pressé?
 * @param shift - Shift pressé?
 * @param alt - Alt pressé?
 * @returns Clé unique pour ce raccourci
 */
function getShortcutKey(
	key: string,
	ctrl: boolean = false,
	shift: boolean = false,
	alt: boolean = false
): string {
	const parts: string[] = [];
	if (ctrl) parts.push('ctrl');
	if (shift) parts.push('shift');
	if (alt) parts.push('alt');
	parts.push(key.toLowerCase());
	return parts.join('+');
}

/**
 * Enregistre un raccourci clavier global
 *
 * @param key - Touche principale (ex: 'n', 's', 'l')
 * @param handler - Fonction à appeler
 * @param ctrl - Nécessite Ctrl? (défaut: true)
 * @param shift - Nécessite Shift? (défaut: false)
 * @param alt - Nécessite Alt? (défaut: false)
 */
export function registerShortcut(
	key: string,
	handler: KeyboardShortcutHandler,
	ctrl: boolean = true,
	shift: boolean = false,
	alt: boolean = false
): void {
	const shortcutKey = getShortcutKey(key, ctrl, shift, alt);
	shortcuts.set(shortcutKey, handler);
}

/**
 * Désenregistre un raccourci clavier
 *
 * @param key - Touche principale
 * @param ctrl - Ctrl était requis?
 * @param shift - Shift était requis?
 * @param alt - Alt était requis?
 */
export function unregisterShortcut(
	key: string,
	ctrl: boolean = true,
	shift: boolean = false,
	alt: boolean = false
): void {
	const shortcutKey = getShortcutKey(key, ctrl, shift, alt);
	shortcuts.delete(shortcutKey);
}

/**
 * Handler d'événement clavier qui gère tous les raccourcis enregistrés
 *
 * @param event - Événement clavier
 */
export function handleKeyboardEvent(event: KeyboardEvent): void {
	// Ignore les événements dans les inputs/textarea sauf Escape
	const target = event.target as HTMLElement;
	const isInputField =
		target.tagName === 'INPUT' ||
		target.tagName === 'TEXTAREA' ||
		target.isContentEditable;

	if (isInputField && event.key !== 'Escape') {
		return;
	}

	const shortcutKey = getShortcutKey(event.key, event.ctrlKey, event.shiftKey, event.altKey);
	const handler = shortcuts.get(shortcutKey);

	if (handler) {
		event.preventDefault();
		event.stopPropagation();
		handler();
	}
}

/**
 * Initialise le gestionnaire de raccourcis clavier global
 *
 * Ajoute un listener sur document qui intercepte tous les événements clavier
 */
export function initKeyboardShortcuts(): void {
	document.addEventListener('keydown', handleKeyboardEvent);
}

/**
 * Nettoie le gestionnaire de raccourcis clavier
 *
 * Retire le listener et vide tous les raccourcis enregistrés
 */
export function cleanupKeyboardShortcuts(): void {
	document.removeEventListener('keydown', handleKeyboardEvent);
	shortcuts.clear();
}

/**
 * Formate un raccourci clavier pour l'affichage
 *
 * @param key - Touche principale
 * @param ctrl - Ctrl requis?
 * @param shift - Shift requis?
 * @param alt - Alt requis?
 * @returns Chaîne formatée (ex: "Ctrl+N", "Ctrl+Shift+S")
 */
export function formatShortcut(
	key: string,
	ctrl: boolean = false,
	shift: boolean = false,
	alt: boolean = false
): string {
	const parts: string[] = [];
	if (ctrl) parts.push('Ctrl');
	if (shift) parts.push('Shift');
	if (alt) parts.push('Alt');
	parts.push(key.toUpperCase());
	return parts.join('+');
}

/**
 * Raccourcis clavier prédéfinis pour TomatoTask
 */
export const SHORTCUTS = {
	NEW_TASK: { key: 'n', ctrl: true, display: 'Ctrl+N' },
	START_STOP_TIMER: { key: 's', ctrl: true, display: 'Ctrl+S' },
	CHANGE_LANGUAGE: { key: 'l', ctrl: true, display: 'Ctrl+L' },
	CLOSE_MODAL: { key: 'Escape', ctrl: false, display: 'Esc' },
	SUBMIT_FORM: { key: 'Enter', ctrl: false, display: 'Enter' }
} as const;
