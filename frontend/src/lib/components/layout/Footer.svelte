<script lang="ts">
    import LesbianHeart from '$lib/assets/icons/lesbian_heart.png';
</script>

<footer>
    <p>
        <a href="https://github.com/Blooym/lesbin">Lesbin</a> is made with
        <img src={LesbianHeart} alt="a love heart emoji filled by a lesbian flag" />
        by <a href="https://blooym.dev">Blooym</a>
    </p>
    <small
        >Lesbin is in active development - pastes may be removed and functionality may change until
        stablised</small
    >
</footer>

<style>
    p {
        margin: 0;
    }
    small {
        color: orange;
    }
    img {
        display: inline;
        vertical-align: middle;
        width: 18px;
        height: 18px;
    }
    footer {
        margin: 1rem;
        text-align: center;
    }
</style>
