<script lang="ts">
    const { children, data } = $props();
</script>

<svelte:head>
    <title>Admin | Lesbin</title>
    <meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if data.authenticationToken}
    <small>Signed in as an administrator - <a href="/admin/signout">sign out</a>.</small>
{/if}

{@render children()}

<style>
    small {
        text-align: end;
    }
</style>
