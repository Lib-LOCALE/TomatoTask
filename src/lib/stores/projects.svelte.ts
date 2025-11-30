// Store réactif pour les projets (Svelte 5 Runes)
import { invoke } from '@tauri-apps/api/core';
import type { Project } from '$lib/types';

/**
 * Store des projets de l'application
 *
 * Gère la liste des projets et les opérations CRUD
 */
class ProjectStore {
	// État mutable
	projects = $state<Project[]>([]);
	selectedProjectId = $state<number | null>(null);
	isLoading = $state(true);

	/**
	 * Charge tous les projets depuis Tauri
	 */
	async load(): Promise<void> {
		this.isLoading = true;
		try {
			const projects = await invoke<Project[]>('get_projects');
			this.projects = projects;
		} catch (error) {
			console.error('Failed to load projects:', error);
			this.projects = [];
		} finally {
			this.isLoading = false;
		}
	}

	/**
	 * Crée un nouveau projet
	 */
	async create(name: string, color: string): Promise<Project> {
		const project = await invoke<Project>('create_project', { name, color });
		this.projects = [...this.projects, project];
		return project;
	}

	/**
	 * Met à jour un projet
	 */
	async update(id: number, name: string, color: string): Promise<Project> {
		const project = await invoke<Project>('update_project', { id, name, color });
		this.projects = this.projects.map((p) => (p.id === id ? project : p));
		return project;
	}

	/**
	 * Supprime un projet
	 */
	async delete(id: number): Promise<void> {
		await invoke('delete_project', { id });
		this.projects = this.projects.filter((p) => p.id !== id);

		// Désélectionne si c'était le projet sélectionné
		if (this.selectedProjectId === id) {
			this.selectedProjectId = null;
		}
	}

	/**
	 * Sélectionne un projet pour filtrer les tâches
	 */
	selectProject(id: number | null): void {
		this.selectedProjectId = id;
	}

	/**
	 * Obtient un projet par ID
	 */
	getById(id: number): Project | undefined {
		return this.projects.find((p) => p.id === id);
	}

	/**
	 * Réordonne les projets
	 *
	 * @param projectIds - Liste des IDs de projets dans le nouvel ordre
	 */
	async reorder(projectIds: number[]): Promise<void> {
		try {
			// Optimistic update
			const idToProject = new Map(this.projects.map((p) => [p.id, p]));
			const newProjects: Project[] = [];

			// Add reordered projects
			projectIds.forEach((id, index) => {
				const project = idToProject.get(id);
				if (project) {
					project.position = index;
					newProjects.push(project);
					idToProject.delete(id);
				}
			});

			// Add remaining projects
			idToProject.forEach((project) => {
				newProjects.push(project);
			});

			this.projects = newProjects;

			await invoke('reorder_projects', { projectIds });
		} catch (err) {
			console.error('Failed to reorder projects:', err);
			// Revert on error
			await this.load();
		}
	}
}

// Instance singleton exportée
export const projectStore = new ProjectStore();
