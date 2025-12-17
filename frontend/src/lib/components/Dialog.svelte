<script lang="ts">
    import Button from './button/Button.svelte';

    let { showModal = $bindable(), title, children, actions, closeText = 'Close' } = $props();

    let dialog: HTMLDialogElement | undefined = $state();

    $effect(() => {
        if (showModal) dialog?.showModal();
        if (!showModal) dialog?.close();
    });
</script>

<!-- 
    TODO: Replace Javascript for open/close handling with closedby, command & commandfor 
    once baseline support is achieved.
-->
<dialog
    bind:this={dialog}
    closedby="any"
    onclose={() => (showModal = false)}
    onclick={(e) => {
        if (e.target === dialog) dialog.close();
    }}
>
    <div>
        <h2>{title}</h2>
        {@render children?.()}

        <div class="actions">
            {@render actions?.()}
            <Button variant="neutral" onclick={() => dialog?.close()}>{closeText}</Button>
        </div>
    </div>
</dialog>

<style>
    dialog {
        width: 90%;
        max-width: 35em;
        border-radius: var(--rounding-normal);
        background-color: var(--surface-colour);
        color: var(--text-colour);
        border: 1px solid black;
        padding: 0;

        h2 {
            margin: 0;
        }

        .actions {
            margin-top: 4px;
            :global(button) {
                min-width: 6em;
            }
        }
    }
    dialog::backdrop {
        background: rgba(0, 0, 0, 0.7);
    }

    dialog > div {
        padding: 1.5em;
        display: flex;
        flex-direction: column;
        gap: 8px;
    }
    dialog[open] {
        animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    }
    @keyframes zoom {
        from {
            transform: scale(0.95);
        }
        to {
            transform: scale(1);
        }
    }
    dialog[open]::backdrop {
        animation: fade 0.2s ease-out;
    }
    @keyframes fade {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }
</style>
