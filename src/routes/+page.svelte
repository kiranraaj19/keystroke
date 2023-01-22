<script>
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";

  /**
   * @type {any[]}
   */
  let prev_keys = [];
  let curr_key = "";
  onMount(async () => {
    const unlisten = await listen(
      "key_event",
      (/** @type {{ payload: any; }} */ event) => {
        if (prev_keys.length > 1) {
          prev_keys.shift();
        }
        prev_keys = [...prev_keys, curr_key];
        curr_key = event.payload;
        // console.log(event);
      }
    );
  });
</script>

<p>
  {#each prev_keys as key}
    <span in:fly={{ x: 5 }} out:fade>{key}</span>
  {/each}
  {#key curr_key}
    <span in:fly={{ x: 5 }}>{curr_key}</span>
  {/key}
</p>

<style>
  p {
    display: flex;
    align-items: center;
    gap: 2px;
  }
</style>
