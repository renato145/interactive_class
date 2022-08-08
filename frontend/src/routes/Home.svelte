<script lang="ts">
  import { navigate, Link } from "svelte-navigator";
  import { t } from "svelte-i18n";
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
      navigate(`room/${new_room}/teacher`);
    } else {
      createErrorMsg = await response.text();
    }
  };

  const deleteRoom = async (roomName: string) => {
    console.log(`deleting ${roomName}`);
    const response = await fetch("cups/delete_room", {
      method: 'DELETE',
      body: roomName
    });
    console.log(response);
  };
</script>

<div>
  <p class="w-fit rounded-md bg-blue-100 p-3 text-lg shadow">
    {$t("home.description1")}
    <a href="http://cups.fast.ai" target="_black" rel="noopener">cups.fast.ai</a
    >, {$t("home.description2")}
    <a
      href="https://github.com/renato145/interactive_class"
      target="_black"
      rel="noopener">{$t("home.link")}</a
    >.
  </p>

  <div class="mt-8">
    {#await cups}
      <p>{$t("loading_msg")}...</p>
    {:then data}
      <p class="text-3xl">
        {data.rooms.length}
        {data.rooms.length > 1 ? $t("rooms") : $t("room")}
      </p>
      <ul class="mt-2 ml-4">
        {#each data.rooms as room}
          <li class="text-lg list-disc list-inside">
            <span class="font-medium">{room}:</span>
            <Link to={`room/${room}/teacher`}>[{$t("teacher-view")}]</Link>
            - <Link to={`room/${room}`}>[{$t("student-view")}]</Link> -
            <button
              class="text-red-600 hover:text-red-800"
              on:click={() => deleteRoom(room)}
              >[{$t("home.delete-room")}]</button
            >
          </li>
        {/each}
      </ul>
    {:catch error}
      <p>{$t("an-error-occurred")}: {error}</p>
    {/await}
  </div>

  <form class="mt-4" on:submit|preventDefault={createNewRoom}>
    <input
      type="text"
      id="new_room"
      name="new_room"
      placeholder={$t("home.new-room-name")}
      required
    />
    <button class="btn" type="submit">{$t("home.create-room")}</button>
  </form>
  {#if createErrorMsg}
    <p class="bg-red-200">{createErrorMsg}</p>
  {/if}
</div>
