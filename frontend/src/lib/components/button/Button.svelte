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
        background-color: var(--neutral-colour);
        color: var(--crust-colour);
    }
    .neutral:hover {
        background-color: color-mix(in srgb, var(--neutral-colour) 70%, white);
    }
    .primary {
        background-color: var(--accent-colour);
        color: var(--crust-colour);
    }
    .primary:hover {
        background-color: color-mix(in srgb, var(--accent-colour) 70%, white);
    }
    .destructive {
        background-color: var(--negative-colour);
        color: var(--crust-colour);
    }
    .destructive:hover {
        background-color: color-mix(in srgb, var(--negative-colour) 70%, white);
    }
</style>
