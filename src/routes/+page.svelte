<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let key = " ";

  onMount(async () => {
    const unlisten = await listen(
      "key_event",
      (/** @type {{ payload: any; }} */ event) => {
        key = event.payload;
        // console.log(event);
      }
    );
  });
</script>

<p>{key}</p>

<style>
  p {
    display: flex;
    align-items: center;
  }
</style>
