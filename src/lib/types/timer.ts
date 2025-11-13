// Types TypeScript pour le timer et les sessions Pomodoro

/**
 * Type de session Pomodoro
 */
export type SessionType = 'work' | 'short_break' | 'long_break';

/**
 * Représente une session Pomodoro complétée
 */
export interface PomodoroSession {
	id: number;
	taskId?: number;
	startedAt: string;
	completedAt?: string;
	durationMinutes: number;
	sessionType: SessionType;
	interrupted: boolean;
}

/**
 * Input pour créer une nouvelle session
 */
export interface CreateSessionInput {
	taskId?: number;
	durationMinutes: number;
	sessionType: SessionType;
}

/**
 * État du timer en cours
 */
export interface TimerState {
	// Temps restant en secondes
	remainingSeconds: number;
	// Temps total de la session en secondes
	totalSeconds: number;
	// Le timer est-il en cours d'exécution?
	isRunning: boolean;
	// Type de session actuelle
	sessionType: SessionType;
	// ID de la tâche associée (si applicable)
	taskId?: number;
	// ID de la session en cours dans la DB
	sessionId?: number;
	// Compteur de Pomodoros complétés dans cette série
	pomodoroCount: number;
}
