<script>
    import { resolve } from '$app/paths';
</script>

<nav>
    <h1><a href={resolve('/')}>Lesbin</a></h1>
    <div>
        <a href={resolve('/')}>New Paste</a>
    </div>
</nav>

<style>
    h1 {
        font-size: larger;
        margin: 0;
        a {
            color: inherit;
        }
    }
    nav {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 18px;
        flex-wrap: wrap;
    }
</style>
