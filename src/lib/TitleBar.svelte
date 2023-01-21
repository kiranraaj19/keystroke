<script>
    //Titlebar events
    import { appWindow } from "@tauri-apps/api/window";
</script>

<div class="buttons">
    <button
        class="button"
        data-tauri-drag-region
        on:click={async () => {
            await appWindow.startDragging();
        }}>/</button
    >
    <button
        class="button"
        on:click={() => {
            appWindow.minimize();
        }}
    >
        -
    </button>
    <button
        class="button"
        on:click={() => {
            appWindow.close();
        }}
    >
        X
    </button>
</div>

<style>
    .buttons {
        position: absolute;
        top: 0;
        right: 0;
    }
</style>
