<script lang="ts">
    import Footer from '$lib/components/layout/Footer.svelte';
    import NavigationBar from '$lib/components/layout/NavigationBar.svelte';
    import Toast from '$lib/components/Toast.svelte';
    import '$lib/styles/reset.css';
    import '$lib/styles/theme.css';
    import '@fontsource/inter';

    const { children } = $props();
</script>

<svelte:head>
    <title>Lesbin</title>
    <meta name="title" content="Lesbin" />
    <meta name="description" content="A privacy respecting and end-to-end encrypted pastebin." />
</svelte:head>

<NavigationBar />
<main>
    <Toast />
    {@render children()}
</main>
<Footer />

<style>
    /* Design */
    :global(html),
    :global(body) {
        height: 100%;
    }
    :global(body) {
        display: flex;
        flex-direction: column;
        background-color: var(--col-background);
        color: var(--col-on-background);
        margin: auto;
        font-family: 'inter', sans-serif;
        max-width: 1200px;
    }

    /* Scoped */
    main {
        display: flex;
        flex-direction: column;
        flex: auto;
        margin: 8px 15px;
    }
</style>
