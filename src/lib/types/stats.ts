export interface DailyFocusTime {
	date: string;
	minutes: number;
}

export interface ProjectDistribution {
	name: string;
	color: string;
	percentage: number;
	minutes: number;
}

export interface Insights {
	total_focus_minutes: number;
	total_pomodoros: number;
	daily: { date: string; pomodoros: number; focus_minutes: number }[];
	by_hour: number[];
	best_hour: number | null;
	by_weekday: number[];
	completion_rate: number;
	previous_period_pomodoros: number;
	top_tasks: { task_id: number; title: string; pomodoros: number }[];
	top_projects: { project_id: number; name: string; color: string | null; pomodoros: number }[];
}

export interface FocusStreak {
	current_days: number;
	best_days: number;
	total_pomodoros: number;
	today_pomodoros: number;
}
