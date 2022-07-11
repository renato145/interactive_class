<script lang="ts">
  import { navigate, Link } from "svelte-navigator";
  import type { CupsInfo } from "bindings/CupsInfo";
  import type { CreateRoom } from "bindings/CreateRoom";

  const getCups = async () => {
    const d = await fetch("/cups").then((response) => response.json());
    return d as CupsInfo;
  };
  let cups = getCups();

  let createErrorMsg: string;
  const createNewRoom = async (ev) => {
    const new_room = new FormData(ev.target).get("new_room") as string;
    const data: CreateRoom = { new_room };
    const response = await fetch("/cups/create_room", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    if (response.ok) {
      navigate(`room/${new_room}`);
    } else {
      createErrorMsg = await response.text();
    }
  };
</script>

<div>
  {#await cups}
    <p>loading...</p>
  {:then data}
    <p class="text-3xl">{data.rooms.length} Rooms</p>
    <ul class="ml-4">
      {#each data.rooms as room}
        <li class="list-disc list-inside">
          <Link to={`room/${room}`}>{room}</Link>
        </li>
      {/each}
    </ul>
  {:catch error}
    <p>An error occurred: {error}</p>
  {/await}

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
