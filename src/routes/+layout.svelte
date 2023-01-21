<script>
  import "../app.css";
  import TitleBar from "$lib/TitleBar.svelte";
</script>

<main class="container">
  <div class="titlebar"><TitleBar /></div>
  <slot />
</main>

<style>
  .container {
    height: 100%;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .titlebar {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translate(-50%, 0);
    width: max-content;
  }
</style>
