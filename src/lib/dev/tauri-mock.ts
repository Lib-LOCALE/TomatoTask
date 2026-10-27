import { mockIPC, mockWindows } from '@tauri-apps/api/mocks';

type Row = Record<string, any>;

/** Backend éphémère réservé au navigateur et aux tests. */
export function installTauriMock(
	empty = new URLSearchParams(location.search).get('seed') === 'empty'
) {
	const now = () => new Date().toISOString();
	let projects: Row[] = [];
	let tasks: Row[] = [];
	let sessions: Row[] = [];
	let settings: Row = {
		workDuration: 25,
		shortBreakDuration: 5,
		longBreakDuration: 15,
		pomodorosUntilLongBreak: 4,
		language: 'en',
		theme: 'light',
		notificationSound: 'default',
		autoStartBreaks: false,
		autoStartPomodoros: false
	};
	const nextId = (rows: Row[]) => Math.max(0, ...rows.map((row) => row.id)) + 1;
	const find = (rows: Row[], id: number) => {
		const row = rows.find((item) => item.id === id);
		if (!row) throw new Error(`Introuvable : ${id}`);
		return row;
	};
	if (!empty) {
		projects = ['Travail', 'Personnel', 'Apprentissage'].map((name, i) => ({
			id: i + 1,
			name,
			color: ['#ef4444', '#3b82f6', '#a855f7'][i],
			created_at: now(),
			updated_at: now(),
			position: i
		}));
		tasks = [
			'Préparer la réunion',
			'Répondre aux messages',
			'Avancer sur le projet',
			'Planifier la semaine',
			'Lire un chapitre',
			'Faire une promenade',
			'Organiser les notes',
			'Réviser les objectifs'
		].map((title, i) => ({
			id: i + 1,
			title,
			description: null,
			projectId: (i % 3) + 1,
			estimatedPomodoros: 2 + (i % 3),
			completedPomodoros: i % 2,
			isCompleted: i > 5,
			createdAt: now(),
			updatedAt: now(),
			completedAt: i > 5 ? now() : null,
			position: i
		}));
		sessions = Array.from({ length: 30 }, (_, i) => {
			const date = new Date();
			date.setDate(date.getDate() - i);
			return {
				id: i + 1,
				taskId: (i % 8) + 1,
				startedAt: date.toISOString(),
				completedAt: date.toISOString(),
				durationMinutes: 25,
				sessionType: 'work',
				interrupted: false
			};
		});
	}
	const summary = (date: string) => {
		const work = sessions.filter(
			(s) => s.completedAt?.startsWith(date) && !s.interrupted && s.sessionType === 'work'
		);
		return {
			date,
			completedTasksCount: tasks.filter((t) => t.completedAt?.startsWith(date)).length,
			completedPomodorosCount: work.length,
			totalFocusMinutes: work.reduce((n, s) => n + s.durationMinutes, 0)
		};
	};
	const dates = (start: string, end: string) => {
		const result: string[] = [];
		const date = new Date(`${start}T00:00:00Z`);
		while (date.toISOString().slice(0, 10) <= end) {
			result.push(date.toISOString().slice(0, 10));
			date.setUTCDate(date.getUTCDate() + 1);
		}
		return result;
	};
	const history = (days: number) => {
		const end = now().slice(0, 10);
		const start = new Date();
		start.setUTCDate(start.getUTCDate() - days + 1);
		return dates(start.toISOString().slice(0, 10), end).map((date) => ({
			date,
			minutes: summary(date).totalFocusMinutes
		}));
	};
	mockWindows('main');
	mockIPC(
		(command, payload) => {
			const a = (payload ?? {}) as Row;
			switch (command) {
				case 'get_settings':
					return { ...settings };
				case 'update_settings':
					settings = { ...a.settings };
					delete settings.enableProjects;
					return { ...settings };
				case 'get_projects':
					return structuredClone(projects);
				case 'create_project': {
					const p = {
						id: nextId(projects),
						name: a.name,
						color: a.color ?? null,
						created_at: now(),
						updated_at: now(),
						position: projects.length
					};
					projects.push(p);
					return { ...p };
				}
				case 'update_project': {
					const p = find(projects, a.id);
					Object.assign(p, { name: a.name, color: a.color, updated_at: now() });
					return { ...p };
				}
				case 'delete_project':
					projects = projects.filter((p) => p.id !== a.id);
					tasks.forEach((t) => {
						if (t.projectId === a.id) t.projectId = null;
					});
					return;
				case 'reorder_projects':
					a.projectIds.forEach((id: number, i: number) => {
						find(projects, id).position = i;
					});
					return;
				case 'get_tasks':
					return structuredClone(tasks);
				case 'get_task':
					return { ...find(tasks, a.id) };
				case 'get_tasks_by_project':
					return structuredClone(tasks.filter((t) => t.projectId === a.projectId));
				case 'create_task': {
					const t = {
						id: nextId(tasks),
						...a.input,
						description: a.input.description ?? null,
						projectId: a.input.projectId ?? null,
						completedPomodoros: 0,
						isCompleted: false,
						createdAt: now(),
						updatedAt: now(),
						completedAt: null,
						position: tasks.length
					};
					tasks.push(t);
					return { ...t };
				}
				case 'update_task': {
					const t = find(tasks, a.id);
					Object.assign(t, a.input, { updatedAt: now() });
					return { ...t };
				}
				case 'toggle_task_completion': {
					const t = find(tasks, a.id);
					t.isCompleted = !t.isCompleted;
					t.completedAt = t.isCompleted ? now() : null;
					t.updatedAt = now();
					return { ...t };
				}
				case 'delete_task':
					tasks = tasks.filter((t) => t.id !== a.id);
					sessions.forEach((s) => {
						if (s.taskId === a.id) s.taskId = null;
					});
					return;
				case 'reorder_tasks':
					a.taskIds.forEach((id: number, i: number) => {
						find(tasks, id).position = i;
					});
					return;
				case 'create_session': {
					const s = {
						id: nextId(sessions),
						taskId: a.taskId ?? null,
						durationMinutes: a.durationMinutes,
						sessionType: a.sessionType,
						startedAt: now(),
						completedAt: null,
						interrupted: false
					};
					sessions.push(s);
					return { ...s };
				}
				case 'complete_session': {
					const s = find(sessions, a.id);
					s.completedAt = now();
					if (s.sessionType === 'work' && s.taskId) find(tasks, s.taskId).completedPomodoros++;
					return { ...s };
				}
				case 'interrupt_session': {
					const s = find(sessions, a.id);
					s.interrupted = true;
					return { ...s };
				}
				case 'get_sessions_by_date_range':
					return structuredClone(
						sessions.filter(
							(s) =>
								s.startedAt.slice(0, 10) >= a.startDate && s.startedAt.slice(0, 10) <= a.endDate
						)
					);
				case 'get_daily_summary':
					return summary(a.date);
				case 'get_weekly_summary':
					return dates(a.startDate, a.endDate).map(summary);
				case 'get_daily_focus_time':
					return history(7);
				case 'get_focus_history':
					return history(a.days);
				case 'get_project_distribution': {
					const totals = projects.map((p) => ({
						projectName: p.name,
						color: p.color,
						minutes: sessions
							.filter(
								(s) =>
									s.completedAt &&
									!s.interrupted &&
									s.sessionType === 'work' &&
									tasks.some((t) => t.id === s.taskId && t.projectId === p.id)
							)
							.reduce((n, s) => n + s.durationMinutes, 0)
					}));
					const sum = totals.reduce((n, p) => n + p.minutes, 0);
					return totals
						.filter((p) => p.minutes)
						.map((p) => ({ ...p, percentage: sum ? (p.minutes / sum) * 100 : 0 }));
				}
				case 'export_data':
					return structuredClone({
						version: 1,
						timestamp: now(),
						projects,
						tasks,
						sessions,
						settings
					});
				case 'import_data':
					({ projects, tasks, sessions } = structuredClone(a.data));
					if (a.data.settings) settings = structuredClone(a.data.settings);
					return;
				case 'toggle_window':
				case 'show_window':
				case 'hide_window':
				case 'quit_app':
					return;
				case 'plugin:notification|is_permission_granted':
					return false;
				case 'plugin:notification|request_permission':
					return 'denied';
				case 'plugin:dialog|open':
				case 'plugin:dialog|save':
					return null;
				default:
					if (
						command.startsWith('plugin:window|') ||
						command.startsWith('plugin:notification|') ||
						command.startsWith('plugin:fs|')
					)
						return;
					throw new Error(`Commande non simulée : ${command}`);
			}
		},
		{ shouldMockEvents: true }
	);
}
