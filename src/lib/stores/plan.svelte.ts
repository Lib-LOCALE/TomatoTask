import { invoke } from '@tauri-apps/api/core';
import { taskStore } from './tasks.svelte';
import type { Task } from '$lib/types';

export type PlannedTask = Task & { carriedOver: boolean };
export interface DayPlan {
	date: string;
	tasks: PlannedTask[];
	planned_pomodoros: number;
	completed_pomodoros: number;
}
export function localDate(date = new Date()): string {
	return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;
}
class PlanStore {
	plan = $state<DayPlan | null>(null);
	loading = $state(false);
	error = $state(false);
	async load() {
		this.loading = true;
		try {
			this.plan = await invoke<DayPlan>('get_day_plan', { date: localDate() });
			this.error = false;
		} catch (error) {
			console.error(error);
			this.error = true;
		} finally {
			this.loading = false;
		}
	}
	async change(command: 'plan_task' | 'unplan_task', id: number) {
		try {
			await invoke(command, command === 'plan_task' ? { id, date: localDate() } : { id });
			await taskStore.load();
			await this.load();
		} catch (error) {
			this.error = true;
			throw error;
		}
	}
	async reorder(ids: number[]) {
		try {
			await invoke('reorder_plan', { date: localDate(), taskIds: ids });
			await this.load();
		} catch (error) {
			this.error = true;
			throw error;
		}
	}
}
export const planStore = new PlanStore();
