<script lang="ts">
    import type { HTMLButtonAttributes } from 'svelte/elements';
    type Props = HTMLButtonAttributes & {
        variant: 'primary' | 'destructive' | 'neutral';
    };
    const { variant, children, ...rest }: Props = $props();
</script>

<button class={`text-btn ${variant}`} {...rest}>{@render children!()}</button>

<style>
    .text-btn {
        padding: 0;
        border: unset;
        background-color: unset;
    }
    .text-btn:hover {
        cursor: pointer;
        text-decoration: underline;
    }
    .text-btn:disabled {
        pointer-events: none;
        cursor: not-allowed;
        opacity: 0.6;
    }
    .neutral {
        color: var(--col-neutral);
    }
    .neutral:hover {
        color: color-mix(in srgb, var(--col-neutral) 70%, white);
    }
    .primary {
        color: var(--col-link);
    }
    .primary:hover {
        color: color-mix(in srgb, var(--col-link) 70%, white);
    }
    .destructive {
        color: var(--col-destructive);
    }
    .destructive:hover {
        color: color-mix(in srgb, var(--col-destructive) 70%, white);
    }
</style>
