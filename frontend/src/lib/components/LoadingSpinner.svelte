<span class="loader"></span>

<style>
    .loader {
        width: 20px;
        height: 20px;
        border: 4px solid #fff;
        border-bottom-color: var(--col-primary);
        border-radius: 50%;
        display: inline-block;
        box-sizing: border-box;
        animation: rotation 1s linear infinite;
    }

    @keyframes rotation {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>
