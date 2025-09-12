<script>
    import { resolve } from '$app/paths';
</script>

<svelte:head>
    <title>Admin Panel | Lesbin</title>
</svelte:head>

<div class="container">
    <h1>Admin Panel</h1>
    <a href={resolve('/admin/reports')}>Paste Reports</a>
</div>

<small
    >Note: the admin panel is a work in progress and will be improved over time - feature requests
    can be made <a target="_blank" href="https://codeberg.org/Blooym/lesbin">on GitHub</a>.</small
>

<style>
    .container {
        flex: auto;
    }
</style>
