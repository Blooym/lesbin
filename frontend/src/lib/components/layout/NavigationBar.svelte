<script>
    import { resolve } from '$app/paths';
</script>

<nav>
    <h1><a href={resolve('/')}>Lesbin</a></h1>
    <div class="links">
        <a class="link" href={resolve('/')}>New Paste</a>
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

        .links {
            display: flex;
            gap: 1rem;
            .link {
                text-decoration: unset;
            }
            .link:hover {
                text-decoration: underline;
            }
        }
    }
</style>
