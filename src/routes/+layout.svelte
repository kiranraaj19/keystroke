<script>
  import "../app.css";
  import TitleBar from "$lib/TitleBar.svelte";
</script>

<main class="container">
  <TitleBar />
  <slot />
</main>
