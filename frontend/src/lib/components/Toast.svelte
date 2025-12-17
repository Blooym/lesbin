<script lang="ts">
    import { toastManager } from '$lib/state/toasts.svelte';
    import { fly } from 'svelte/transition';
</script>

<div class="toast-container">
    {#each toastManager.toasts as toast (toast.id)}
        <div class="toast {toast.variant}" transition:fly={{ x: 10, duration: 300 }}>
            {toast.message}
        </div>
    {/each}
</div>

<style>
    .toast-container {
        position: fixed;
        bottom: 1rem;
        right: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        z-index: 1000;
    }
    .toast {
        padding: 1rem;
        border-radius: var(--rounding-normal);
        color: var(--text-colour);
        background-color: var(--surface-colour);
        box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
        min-width: 200px;
    }
    .info {
        background-color: var(--informational-colour);
        color: var(--crust-colour);
    }
    .success {
        background-color: var(--positive-colour);
        color: var(--crust-colour);
    }
    .error {
        background-color: var(--negative-colour);
        color: var(--crust-colour);
    }
</style>
