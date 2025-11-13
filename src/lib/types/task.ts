// Types TypeScript pour les tâches et projets

/**
 * Représente une tâche à accomplir
 */
export interface Task {
	id: number;
	title: string;
	description?: string;
	projectId?: number;
	estimatedPomodoros: number;
	completedPomodoros: number;
	isCompleted: boolean;
	createdAt: string;
	updatedAt: string;
	completedAt?: string;
}

/**
 * Input pour créer une nouvelle tâche
 */
export interface CreateTaskInput {
	title: string;
	description?: string;
	projectId?: number;
	estimatedPomodoros: number;
}

/**
 * Input pour mettre à jour une tâche existante
 */
export interface UpdateTaskInput {
	title: string;
	description?: string;
	projectId?: number;
	estimatedPomodoros: number;
}

/**
 * Représente un projet pour organiser les tâches
 */
export interface Project {
	id: number;
	name: string;
	color?: string;
	createdAt: string;
	updatedAt: string;
}

/**
 * Input pour créer un nouveau projet
 */
export interface CreateProjectInput {
	name: string;
	color?: string;
}
