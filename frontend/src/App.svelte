<script lang="ts">
  import type { CupsInfo } from "bindings/CupsInfo";
  import type { CreateRoom } from "bindings/CreateRoom";

  const getCups = async () => {
    const d = await fetch("/cups").then((response) => response.json());
    return d as CupsInfo;
  };
  let cups = getCups();

  let res;
  const createNewRoom = async (e) => {
    const formData = new FormData(e.target);
    const data: CreateRoom = { new_room: formData.get("new_room") as string };
    const d = await fetch("/cups/create_room", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    }).then((response) => response.json());

    console.log(d);
    res = JSON.stringify(d);
  };
</script>

<div class="container mx-auto mt-4 rounded bg-slate-50 p-4">
  <div class="text-6xl">Interactive Class</div>

  <div class="mt-8">
    {#await cups}
      <p>loading...</p>
    {:then data}
      <p class="text-3xl">{data.rooms} Rooms</p>
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
  {res}
</div>
