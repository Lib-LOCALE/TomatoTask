// Store réactif pour la gestion du timer Pomodoro (Svelte 5 Runes)
import type { SessionType, TimerState } from '$lib/types';

/**
 * État réactif du timer
 *
 * Utilise les Runes Svelte 5 pour une réactivité optimale
 */
class TimerStore {
	// État mutable avec $state
	remainingSeconds = $state(0);
	totalSeconds = $state(0);
	isRunning = $state(false);
	sessionType = $state<SessionType>('work');
	taskId = $state<number | undefined>(undefined);
	sessionId = $state<number | undefined>(undefined);
	pomodoroCount = $state(0);

	// Dérivé: progression en pourcentage
	progress = $derived(() => {
		if (this.totalSeconds === 0) return 0;
		return Math.round(((this.totalSeconds - this.remainingSeconds) / this.totalSeconds) * 100);
	});

	// Dérivé: temps formaté MM:SS
	displayTime = $derived(() => {
		const mins = Math.floor(this.remainingSeconds / 60);
		const secs = this.remainingSeconds % 60;
		return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	});

	/**
	 * Démarre une nouvelle session
	 *
	 * @param durationMinutes - Durée en minutes
	 * @param type - Type de session
	 * @param linkedTaskId - ID de la tâche associée (optionnel)
	 * @param dbSessionId - ID de la session dans la DB
	 */
	startSession(
		durationMinutes: number,
		type: SessionType,
		linkedTaskId?: number,
		dbSessionId?: number
	): void {
		this.totalSeconds = durationMinutes * 60;
		this.remainingSeconds = this.totalSeconds;
		this.sessionType = type;
		this.taskId = linkedTaskId;
		this.sessionId = dbSessionId;
		this.isRunning = true;
	}

	/**
	 * Met le timer en pause
	 */
	pause(): void {
		this.isRunning = false;
	}

	/**
	 * Reprend le timer
	 */
	resume(): void {
		this.isRunning = true;
	}

	/**
	 * Arrête complètement le timer
	 */
	stop(): void {
		this.isRunning = false;
		this.remainingSeconds = 0;
		this.totalSeconds = 0;
		this.sessionId = undefined;
	}

	/**
	 * Décrémente le timer d'une seconde
	 *
	 * @returns true si le timer a atteint zéro
	 */
	tick(): boolean {
		if (!this.isRunning || this.remainingSeconds <= 0) {
			return false;
		}

		this.remainingSeconds -= 1;

		// Retourne true si terminé
		return this.remainingSeconds === 0;
	}

	/**
	 * Complète la session courante et incrémente le compteur de Pomodoros
	 */
	completeSession(): void {
		if (this.sessionType === 'work') {
			this.pomodoroCount += 1;
		}
		this.stop();
	}

	/**
	 * Réinitialise le compteur de Pomodoros
	 */
	resetPomodoroCount(): void {
		this.pomodoroCount = 0;
	}

	/**
	 * Obtient l'état complet du timer
	 */
	getState(): TimerState {
		return {
			remainingSeconds: this.remainingSeconds,
			totalSeconds: this.totalSeconds,
			isRunning: this.isRunning,
			sessionType: this.sessionType,
			taskId: this.taskId,
			sessionId: this.sessionId,
			pomodoroCount: this.pomodoroCount
		};
	}
}

// Instance singleton exportée
export const timerStore = new TimerStore();
