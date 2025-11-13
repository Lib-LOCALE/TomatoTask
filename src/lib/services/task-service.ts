// Service de gestion des tâches
import { taskStore } from '$lib/stores/tasks.svelte';
import type { CreateTaskInput, UpdateTaskInput, Task } from '$lib/types';

/**
 * Initialise le store des tâches
 *
 * Charge toutes les tâches depuis la base de données
 */
export async function initializeTasks(): Promise<void> {
	await taskStore.load();
}

/**
 * Crée une nouvelle tâche
 *
 * @param title - Titre de la tâche
 * @param description - Description (optionnel)
 * @param projectId - ID du projet (optionnel)
 * @param estimatedPomodoros - Nombre de Pomodoros estimés (défaut: 1)
 * @returns Tâche créée
 */
export async function createTask(
	title: string,
	description?: string,
	projectId?: number,
	estimatedPomodoros: number = 1
): Promise<Task> {
	const input: CreateTaskInput = {
		title: title.trim(),
		description: description?.trim(),
		projectId,
		estimatedPomodoros
	};

	return await taskStore.create(input);
}

/**
 * Met à jour une tâche existante
 *
 * @param id - ID de la tâche
 * @param title - Nouveau titre
 * @param description - Nouvelle description
 * @param projectId - Nouveau projet
 * @param estimatedPomodoros - Nouveaux Pomodoros estimés
 * @returns Tâche mise à jour
 */
export async function updateTask(
	id: number,
	title: string,
	description?: string,
	projectId?: number,
	estimatedPomodoros: number = 1
): Promise<Task> {
	const input: UpdateTaskInput = {
		title: title.trim(),
		description: description?.trim(),
		projectId,
		estimatedPomodoros
	};

	return await taskStore.update(id, input);
}

/**
 * Supprime une tâche
 *
 * @param id - ID de la tâche à supprimer
 */
export async function deleteTask(id: number): Promise<void> {
	await taskStore.delete(id);
}

/**
 * Marque une tâche comme complétée ou non complétée
 *
 * @param id - ID de la tâche
 * @returns Tâche mise à jour
 */
export async function toggleTaskCompletion(id: number): Promise<Task> {
	return await taskStore.toggleCompletion(id);
}

/**
 * Sélectionne une tâche (pour l'associer au timer par exemple)
 *
 * @param task - Tâche à sélectionner
 */
export function selectTask(task: Task | null): void {
	taskStore.select(task);
}

/**
 * Obtient la tâche actuellement sélectionnée
 *
 * @returns Tâche sélectionnée ou null
 */
export function getSelectedTask(): Task | null {
	return taskStore.selectedTask;
}

/**
 * Filtre les tâches par statut de complétion
 *
 * @param filter - Filtre à appliquer ('all', 'active', 'completed')
 */
export function filterTasksByCompletion(filter: 'all' | 'active' | 'completed'): void {
	taskStore.setCompletionFilter(filter);
}

/**
 * Filtre les tâches par projet
 *
 * @param projectId - ID du projet (null pour tous)
 */
export function filterTasksByProject(projectId: number | null): void {
	taskStore.setProjectFilter(projectId);
}

/**
 * Réinitialise tous les filtres
 */
export function clearTaskFilters(): void {
	taskStore.clearFilters();
}
