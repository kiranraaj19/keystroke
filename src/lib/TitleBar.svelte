<script>
    //Titlebar events
    import { appWindow } from "@tauri-apps/api/window";
</script>

<svelte:head>
    <script src="https://kit.fontawesome.com/b440af3128.js" crossorigin="anonymous"></script>
</svelte:head>


<div class="buttons">
    <button
        class="button"
        data-tauri-drag-region
        on:click={async () => {
            await appWindow.startDragging();
        }}
    >
    <i data-tauri-drag-region class="fa-solid fa-map-pin"></i>
    </button>
    <button
        class="button"
        on:click={() => {
            appWindow.minimize();
        }}
    >
    <i class="fa-solid fa-compress"></i>
    </button>
    <button
        class="button"
        on:click={() => {
            appWindow.close();
        }}
    >
    <i class="fa-solid fa-xmark"></i>
    </button>
</div>

<style>
    .button {
        background: none;
        color: inherit;
        border: none;
        font-size: 16px;
    }
</style>
