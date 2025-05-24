<script lang="ts">
    import Button from '$lib/components/button/Button.svelte';
    import type { PageProps } from './$types';
    let { data, form }: PageProps = $props();
</script>

<svelte:head>
    <title>Admin Sign-in | Lesbin</title>
    <meta name="title" content="Admin Sign-in" />
    <meta name="description" content="Please sign in to continue." />
</svelte:head>

<div class="container">
    <h1>Admin Sign-in</h1>
    <form method="POST" class="signin-form">
        <input
            name="token"
            type="password"
            autocomplete="current-password"
            placeholder="Authentication Token"
            required
        />
        <Button variant="primary" type="submit">Continue</Button>
    </form>
    {#if form?.error}
        <p class="form-error">{form.error.message}</p>
    {/if}

    {#if data.returnUrl}
        <p>You will be redirected to {data.returnUrl} after signing in</p>
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        flex: 1;
        justify-content: center;
        align-items: center;

        .signin-form {
            display: flex;
            flex-direction: column;
            width: 100%;
            max-width: 350px;
            gap: 0.75rem;
        }

        .form-error {
            color: var(--col-error);
            margin-top: 0.5rem;
        }
    }
</style>
