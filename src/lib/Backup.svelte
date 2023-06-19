<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";

  let url = "";
  let db = "";
  let outputMsg = "";
  let folder = null;
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

  $: if (db.length) {
    const urlObj = new URL(url);
    urlObj.pathname = db;
    url = urlObj.toString();
  }

  async function chooseDir() {
    const val = await open({
      directory: true,
    });
    console.log({ val });
    folder = val;
  }

  async function generateBackup() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    if (!url || !folder) return alert("messing");
    progressing = true;
    outputMsg = await invoke("generate_backup", { url, folder, db });
    setTimeout(() => (outputMsg = ""), 5000);
  }
</script>

<div style="margin-bottom: 3rem;">
  <h2>Backup</h2>
  <form class="card" on:submit|preventDefault={generateBackup}>
    <input
      id="url-input"
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
        id="folder"
        placeholder="Enter Folder Path..."
        bind:value={folder}
      />
    </div>
    <button type="submit" disabled={progressing}>Backup</button>
  </form>
</div>

<style>
  .card {
    display: flex;
    flex-direction: column;
    justify-content: start;
  }
</style>
