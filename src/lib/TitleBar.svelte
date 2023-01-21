<script>
    //Titlebar events
    import { appWindow } from "@tauri-apps/api/window";
</script>

<div class="buttons">
    <button
        class="button"
        on:click={async () => {
            await appWindow.startDragging();
        }}
    >
        <ion-icon data-tauri-drag-region name="compass-outline" />
    </button>
    <button
        class="button"
        on:click={() => {
            appWindow.minimize();
        }}
    >
        <ion-icon name="remove-outline" />
    </button>
    <button
        class="button"
        on:click={() => {
            appWindow.close();
        }}
    >
        <ion-icon name="close-outline" />
    </button>

    <script
        type="module"
        src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.esm.js"
    ></script>
    <script
        nomodule
        src="https://unpkg.com/ionicons@5.5.2/dist/ionicons/ionicons.js"
    ></script>
</div>

<style>
    .button {
        background: none;
        color: inherit;
        border: none;
        font-size: 24px;
    }
</style>
