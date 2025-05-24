type Toast = {
    id: number;
    message: string;
    variant?: 'success' | 'error' | 'info';
    duration?: number;
};

class ToastManager {
    private state = $state({
        toasts: [] as Toast[],
        nextId: 1
    });

    get toasts() {
        return this.state.toasts;
    }

    createToast(message: string, options?: Omit<Toast, 'id' | 'message'>) {
        const toast: Toast = {
            id: this.state.nextId++,
            message,
            ...options
        };
        this.state.toasts.push(toast);
        setTimeout(() => {
            this.removeToast(toast.id);
        }, toast.duration ?? 3000);
    }

    removeToast(id: number) {
        this.state.toasts = this.state.toasts.filter((t) => t.id !== id);
    }
}

export const toastManager = new ToastManager();
