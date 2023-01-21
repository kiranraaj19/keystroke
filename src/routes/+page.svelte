<script>
  import { invoke } from "@tauri-apps/api";
  import { listen } from "@tauri-apps/api";
  let key = "";

  await listen("key_event", (/** @type {{ payload: any; }} */ event) => {
    let key = event.payload;
  });
</script>

<p>{key}</p>
