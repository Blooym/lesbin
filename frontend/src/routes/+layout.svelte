<script lang="ts">
    import favicon from '$lib/assets/icons/lesbian_heart.png';
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
    <meta
        name="description"
        content="A privacy-first end-to-end encrypted paste service for sharing your dreams, fanfiction, logs and code."
    />
    <meta property="og:site_name" content="Lesbin" />
    <link rel="icon" href={favicon} />
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
        background-color: var(--base-colour);
        color: var(--text-colour);
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
