<script lang="ts">
    import type { HTMLButtonAttributes } from 'svelte/elements';
    type Props = HTMLButtonAttributes & {
        variant: 'primary' | 'destructive' | 'neutral';
    };
    const { variant, children, ...rest }: Props = $props();
</script>

<button class={`btn ${variant}`} {...rest}> {@render children!()} </button>

<style>
    .btn {
        padding: 6px;
        border: none;
        border-radius: var(--rounding-normal);
    }
    .btn:hover {
        cursor: pointer;
    }
    .btn:disabled {
        pointer-events: none;
        cursor: not-allowed;
        opacity: 0.6;
    }
    .neutral {
        background-color: var(--col-neutral);
        color: var(--col-on-neutral);
    }
    .neutral:hover {
        background-color: color-mix(in srgb, var(--col-neutral) 70%, white);
    }
    .primary {
        background-color: var(--col-primary);
        color: var(--col-on-primary);
    }
    .primary:hover {
        background-color: color-mix(in srgb, var(--col-primary) 70%, white);
    }
    .destructive {
        background-color: var(--col-destructive);
        color: var(--col-on-destructive);
    }
    .destructive:hover {
        background-color: color-mix(in srgb, var(--col-destructive) 70%, white);
    }
</style>
