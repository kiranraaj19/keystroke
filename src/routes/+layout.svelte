<script>
  import { TauriEvent } from "@tauri-apps/api/event";
  import "../app.css";

  //Titlebar events
  import { appWindow } from "@tauri-apps/api/window";
</script>

<main class="container">
  <div class="buttons">
    <button
      class="drag_button"
      data-tauri-drag-region
      on:click={async () => {
        await appWindow.startDragging();
      }}>/</button
    >
    <button
      class="minimize_button"
      on:click={() => {
        appWindow.minimize();
      }}
    >
      -
    </button>
    <button
      class="close_button"
      on:click={() => {
        appWindow.close();
      }}
    >
      X
    </button>
  </div>
  <slot />
</main>
