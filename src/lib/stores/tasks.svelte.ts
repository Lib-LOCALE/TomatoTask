// Store réactif pour la gestion des tâches (Svelte 5 Runes)
import { invoke } from '@tauri-apps/api/core';
import type { Task, CreateTaskInput, UpdateTaskInput } from '$lib/types';

/**
 * Store des tâches de l'application
 *
 * Gère l'état des tâches et les opérations CRUD via Tauri
 */
class TaskStore {
	// État mutable
	tasks = $state<Task[]>([]);
	selectedTask = $state<Task | null>(null);
	filterCompleted = $state<'all' | 'active' | 'completed'>('all');
	filterProjectId = $state<number | null>(null);
	isLoading = $state(true);
	error = $state<string | null>(null);

	// Dérivé: tâches filtrées
	filteredTasks = $derived(() => {
		let filtered = this.tasks;

		// Filtre par statut de complétion
		if (this.filterCompleted === 'active') {
			filtered = filtered.filter((t) => !t.isCompleted);
		} else if (this.filterCompleted === 'completed') {
			filtered = filtered.filter((t) => t.isCompleted);
		}

		// Filtre par projet
		if (this.filterProjectId !== null) {
			filtered = filtered.filter((t) => t.projectId === this.filterProjectId);
		}

		return filtered;
	});

	// Dérivé: nombre de tâches actives
	activeCount = $derived(() => {
		return this.tasks.filter((t) => !t.isCompleted).length;
	});

	// Dérivé: nombre de tâches complétées
	completedCount = $derived(() => {
		return this.tasks.filter((t) => t.isCompleted).length;
	});

	/**
	 * Charge toutes les tâches depuis Tauri
	 */
	async load(): Promise<void> {
		this.isLoading = true;
		this.error = null;

		try {
			const tasks = await invoke<Task[]>('get_tasks');
			this.tasks = tasks;
		} catch (err) {
			console.error('Failed to load tasks:', err);
			this.error = 'Failed to load tasks';
		} finally {
			this.isLoading = false;
		}
	}

	/**
	 * Crée une nouvelle tâche
	 *
	 * @param input - Données de la tâche
	 * @returns Tâche créée
	 */
	async create(input: CreateTaskInput): Promise<Task> {
		try {
			const task = await invoke<Task>('create_task', { input });
			this.tasks = [task, ...this.tasks]; // Ajoute en début de liste
			return task;
		} catch (err) {
			console.error('Failed to create task:', err);
			this.error = 'Failed to create task';
			throw err;
		}
	}

	/**
	 * Met à jour une tâche existante
	 *
	 * @param id - ID de la tâche
	 * @param input - Nouvelles données
	 * @returns Tâche mise à jour
	 */
	async update(id: number, input: UpdateTaskInput): Promise<Task> {
		try {
			const updated = await invoke<Task>('update_task', { id, input });

			// Met à jour dans la liste
			this.tasks = this.tasks.map((t) => (t.id === id ? updated : t));

			// Met à jour la sélection si nécessaire
			if (this.selectedTask?.id === id) {
				this.selectedTask = updated;
			}

			return updated;
		} catch (err) {
			console.error('Failed to update task:', err);
			this.error = 'Failed to update task';
			throw err;
		}
	}

	/**
	 * Supprime une tâche
	 *
	 * @param id - ID de la tâche à supprimer
	 */
	async delete(id: number): Promise<void> {
		try {
			await invoke('delete_task', { id });

			// Retire de la liste
			this.tasks = this.tasks.filter((t) => t.id !== id);

			// Désélectionne si nécessaire
			if (this.selectedTask?.id === id) {
				this.selectedTask = null;
			}
		} catch (err) {
			console.error('Failed to delete task:', err);
			this.error = 'Failed to delete task';
			throw err;
		}
	}

	/**
	 * Bascule le statut de complétion d'une tâche
	 *
	 * @param id - ID de la tâche
	 * @returns Tâche mise à jour
	 */
	async toggleCompletion(id: number): Promise<Task> {
		try {
			const updated = await invoke<Task>('toggle_task_completion', { id });

			// Met à jour dans la liste
			this.tasks = this.tasks.map((t) => (t.id === id ? updated : t));

			return updated;
		} catch (err) {
			console.error('Failed to toggle task completion:', err);
			this.error = 'Failed to toggle task completion';
			throw err;
		}
	}

	/**
	 * Sélectionne une tâche
	 *
	 * @param task - Tâche à sélectionner (ou null pour désélectionner)
	 */
	select(task: Task | null): void {
		this.selectedTask = task;
	}

	/**
	 * Change le filtre de complétion
	 *
	 * @param filter - Nouveau filtre
	 */
	setCompletionFilter(filter: 'all' | 'active' | 'completed'): void {
		this.filterCompleted = filter;
	}

	/**
	 * Change le filtre de projet
	 *
	 * @param projectId - ID du projet (null pour tous)
	 */
	setProjectFilter(projectId: number | null): void {
		this.filterProjectId = projectId;
	}

	/**
	 * Réinitialise tous les filtres
	 */
	clearFilters(): void {
		this.filterCompleted = 'all';
		this.filterProjectId = null;
	}
}

// Instance singleton exportée
export const taskStore = new TaskStore();
