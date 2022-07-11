<script lang="ts">
  import type { CupsInfo } from "bindings/CupsInfo";
  import type { CreateRoom } from "bindings/CreateRoom";

  const getCups = async () => {
    const d = await fetch("/cups").then((response) => response.json());
    return d as CupsInfo;
  };
  let cups = getCups();

  let createErrorMsg: string;
  const createNewRoom = async (ev) => {
    const formData = new FormData(ev.target);
    const data: CreateRoom = { new_room: formData.get("new_room") as string };
    const response = await fetch("/cups/create_room", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (response.ok) {
      // redirect to room page
    } else {
      createErrorMsg = await response.text();
    }
  };
</script>

<div class="container mx-auto mt-4 rounded bg-slate-50 p-4">
  <div class="text-6xl">Interactive Class</div>

  <div class="mt-8">
    {#await cups}
      <p>loading...</p>
    {:then data}
      <p class="text-3xl">{data.rooms.length} Rooms</p>
    {:catch error}
      <p>An error occurred: {error}</p>
    {/await}
  </div>

  <form class="mt-4" on:submit|preventDefault={createNewRoom}>
    <input
      type="text"
      id="new_room"
      name="new_room"
      placeholder="New room name"
      required
    />
    <button class="btn" type="submit">Create room</button>
  </form>
  {#if createErrorMsg}
    <p class="bg-red-200">{createErrorMsg}</p>
  {/if}
</div>
