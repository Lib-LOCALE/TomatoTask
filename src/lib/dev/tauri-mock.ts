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
			position: i,
			plannedFor: null,
			planPosition: null
		}));
		sessions = Array.from({ length: 100 }, (_, i) => {
			const date = new Date();
			const offset = Math.max(0, i - 3);
			date.setDate(date.getDate() - offset - Math.floor(offset / 6));
			date.setHours(8 + (i % 9), (i % 2) * 30, 0, 0);
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
						position: tasks.length,
						plannedFor: null,
						planPosition: null
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
				case 'plan_task': {
					const t = find(tasks, a.id);
					t.plannedFor = a.date;
					t.planPosition =
						Math.max(
							-1,
							...tasks
								.filter((x) => x.id !== t.id && x.plannedFor === a.date)
								.map((x) => x.planPosition ?? -1)
						) + 1;
					return { ...t };
				}
				case 'unplan_task': {
					const t = find(tasks, a.id);
					t.plannedFor = null;
					t.planPosition = null;
					return { ...t };
				}
				case 'get_day_plan': {
					const today = new Date().toLocaleDateString('en-CA');
					const planned = tasks
						.filter(
							(t) =>
								t.plannedFor === a.date ||
								(a.date === today && t.plannedFor && t.plannedFor < a.date && !t.isCompleted)
						)
						.sort(
							(x, y) =>
								(x.planPosition ?? Infinity) - (y.planPosition ?? Infinity) ||
								x.plannedFor.localeCompare(y.plannedFor) ||
								x.id - y.id
						);
					return {
						date: a.date,
						tasks: structuredClone(
							planned.map((t) => ({ ...t, carriedOver: t.plannedFor !== a.date }))
						),
						planned_pomodoros: planned.reduce((sum, t) => sum + t.estimatedPomodoros, 0),
						completed_pomodoros: sessions.filter(
							(s) =>
								s.completedAt &&
								new Date(s.completedAt).toLocaleDateString('en-CA') === a.date &&
								!s.interrupted &&
								s.sessionType === 'work'
						).length
					};
				}
				case 'reorder_plan': {
					const today = new Date().toLocaleDateString('en-CA');
					const expected = tasks
						.filter(
							(t) =>
								t.plannedFor === a.date ||
								(a.date === today && t.plannedFor && t.plannedFor < a.date && !t.isCompleted)
						)
						.map((t) => t.id)
						.sort((x, y) => x - y);
					if (
						JSON.stringify([...a.taskIds].sort((x: number, y: number) => x - y)) !==
						JSON.stringify(expected)
					)
						throw new Error('Ordre invalide');
					a.taskIds.forEach((id: number, i: number) => {
						find(tasks, id).planPosition = i;
					});
					return;
				}
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
				case 'get_focus_streak': {
					const work = sessions.filter(
						(s) => s.completedAt && !s.interrupted && s.sessionType === 'work'
					);
					const localDay = (value: Date) =>
						`${value.getFullYear()}-${String(value.getMonth() + 1).padStart(2, '0')}-${String(value.getDate()).padStart(2, '0')}`;
					const days = [...new Set(work.map((s) => localDay(new Date(s.completedAt))))].sort();
					let best = 0,
						run = 0,
						previous = '';
					for (const day of days) {
						const date = new Date(`${day}T12:00:00`);
						date.setDate(date.getDate() - 1);
						run = previous === localDay(date) ? run + 1 : 1;
						best = Math.max(best, run);
						previous = day;
					}
					const today = new Date();
					const todayKey = localDay(today);
					today.setDate(today.getDate() - 1);
					return {
						current_days: previous === todayKey || previous === localDay(today) ? run : 0,
						best_days: best,
						total_pomodoros: work.length,
						today_pomodoros: work.filter((s) => localDay(new Date(s.completedAt)) === todayKey)
							.length
					};
				}
				case 'get_insights': {
					const days = Number(a.days);
					const localDay = (date: Date) =>
						`${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;
					const today = new Date();
					today.setHours(0, 0, 0, 0);
					const start = new Date(today);
					start.setDate(start.getDate() - days + 1);
					const previous = new Date(start);
					previous.setDate(previous.getDate() - days);
					const work = sessions.filter(
						(s) => s.sessionType === 'work' && new Date(s.startedAt) >= start
					);
					const completed = work.filter((s) => s.completedAt && !s.interrupted);
					const daily = Array.from({ length: days }, (_, index) => {
						const date = new Date(start);
						date.setDate(date.getDate() + index);
						const key = localDay(date);
						const rows = completed.filter((s) => localDay(new Date(s.startedAt)) === key);
						return {
							date: key,
							pomodoros: rows.length,
							focus_minutes: rows.reduce((n, s) => n + s.durationMinutes, 0)
						};
					});
					const by_hour = Array.from(
						{ length: 24 },
						(_, hour) => completed.filter((s) => new Date(s.startedAt).getHours() === hour).length
					);
					const by_weekday = Array.from(
						{ length: 7 },
						(_, day) =>
							completed.filter((s) => (new Date(s.startedAt).getDay() + 6) % 7 === day).length
					);
					return {
						total_focus_minutes: daily.reduce((n, d) => n + d.focus_minutes, 0),
						total_pomodoros: completed.length,
						daily,
						by_hour,
						best_hour: completed.length ? by_hour.indexOf(Math.max(...by_hour)) : null,
						by_weekday,
						completion_rate: work.length ? completed.length / work.length : 0,
						previous_period_pomodoros: sessions.filter(
							(s) =>
								s.sessionType === 'work' &&
								s.completedAt &&
								!s.interrupted &&
								new Date(s.startedAt) >= previous &&
								new Date(s.startedAt) < start
						).length,
						top_tasks: tasks
							.map((t) => ({
								task_id: t.id,
								title: t.title,
								pomodoros: completed.filter((s) => s.taskId === t.id).length
							}))
							.filter((t) => t.pomodoros)
							.sort((a, b) => b.pomodoros - a.pomodoros)
							.slice(0, 5),
						top_projects: projects
							.map((p) => ({
								project_id: p.id,
								name: p.name,
								color: p.color,
								pomodoros: completed.filter((s) =>
									tasks.some((t) => t.id === s.taskId && t.projectId === p.id)
								).length
							}))
							.filter((p) => p.pomodoros)
							.sort((a, b) => b.pomodoros - a.pomodoros)
					};
				}
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
