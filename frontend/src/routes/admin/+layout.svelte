<script lang="ts">
    import { resolve } from '$app/paths';
    const { children, data } = $props();
</script>

<svelte:head>
    <meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if data.authenticationToken}
    <small>Signed in as an administrator - <a href={resolve('/admin/signout')}>sign out</a>.</small>
{/if}

{@render children()}

<style>
    small {
        text-align: end;
    }
</style>
