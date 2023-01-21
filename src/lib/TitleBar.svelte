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
        }}
    >
        
    </button>
    <button
        class="button"
        on:click={() => {
            appWindow.minimize();
        }}
    >
        
    </button>
    <button
        class="button"
        on:click={() => {
            appWindow.close();
        }}
    >
        
    </button>
</div>

<style>
    .button {
        background: none;
        color: inherit;
        border: none;
        font-size: 24px;
    }
</style>
