<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";

  let url = "";
  let db = "";
  let path = null;
  let progressing = false;
  let unListen = null;

  async function setupStatusEvent() {
    unListen = await listen("status", (e) => {
      alert(e.payload);
      progressing = false;
    });
  }

  setupStatusEvent();

  onDestroy(unListen);

  async function chooseDir() {
    const val = await open({ directory: true });
    console.log({ val });
    path = val;
  }

  $: if (db.length) {
    const urlObj = new URL(url);
    urlObj.pathname = db;
    url = urlObj.toString();
  }

  async function generateBackup() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    if (!url || !path) return alert("messing");
    progressing = true;
    await invoke("restore_db", { url, path, db });
  }
</script>

<div>
  <h2>Restore</h2>
  <form class="card" on:submit|preventDefault={generateBackup}>
    <input
      id="greet-input"
      placeholder="Enter DB URL..."
      bind:value={url}
      on:blur={(e) => {
        db = new URL(e.currentTarget.value).pathname.slice(1);
      }}
    />
    <input id="db-name" placeholder="Enter DB Name..." bind:value={db} />
    <div>
      <input
        id="choosen-folder"
        type="button"
        value="Choose Folder"
        on:click={chooseDir}
      />
      <input
        id="db-name"
        placeholder="Enter Folder Path..."
        bind:value={path}
      />
    </div>
    <br />
    <button type="submit" disabled={progressing}>Restore</button>
  </form>
</div>

<style>
  .card {
    display: flex;
    flex-direction: column;
    justify-content: start;
  }
</style>
