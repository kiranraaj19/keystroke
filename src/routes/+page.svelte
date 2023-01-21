<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  let key = "";

  onMount(async () => {
    const unlisten = await listen(
      "key_event",
      (/** @type {{ payload: any; }} */ event) => {
        let key = event.payload;
      }
    );
  });
</script>

<p>{key}</p>
