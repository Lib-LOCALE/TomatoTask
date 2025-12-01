// Store pour gérer les notifications toast
type ToastType = 'success' | 'error' | 'info' | 'celebration';

interface Toast {
	id: number;
	message: string;
	type: ToastType;
	duration?: number;
}

class ToastStore {
	toasts = $state<Toast[]>([]);
	private nextId = 0;

	show(message: string, type: ToastType = 'info', duration = 3000) {
		const id = this.nextId++;
		const toast: Toast = { id, message, type, duration };
		this.toasts.push(toast);

		// Auto-remove after duration
		setTimeout(() => {
			this.remove(id);
		}, duration + 300); // Add 300ms for exit animation
	}

	success(message: string, duration = 3000) {
		this.show(message, 'success', duration);
	}

	error(message: string, duration = 4000) {
		this.show(message, 'error', duration);
	}

	info(message: string, duration = 3000) {
		this.show(message, 'info', duration);
	}

	celebration(message: string, duration = 4000) {
		this.show(message, 'celebration', duration);
	}

	remove(id: number) {
		this.toasts = this.toasts.filter((t) => t.id !== id);
	}

	clear() {
		this.toasts = [];
	}
}

export const toastStore = new ToastStore();
