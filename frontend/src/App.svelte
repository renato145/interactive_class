<script lang="ts">
  import { Router, Route, Link } from "svelte-navigator";
  import { t, isLoading, locale, locales } from "svelte-i18n";
  import Home from "./routes/Home.svelte";
  import Room from "./routes/Room.svelte";
  import TeacherView from "./routes/TeacherView.svelte";
  import "./i18n";
</script>

<Router primary={false}>
  <div class="container mx-auto my-4 rounded bg-slate-50 p-4">
    {#if $isLoading}
      <p>loading...</p>
    {:else}
      <nav class="flex items-center justify-between">
        <Link to="/">Home</Link>
        <select bind:value={$locale} class="rounded border-gray-700 py-1 pl-2">
          {#each $locales as locale}
            <option value={locale}>{locale}</option>
          {/each}
        </select>
      </nav>
      <div class="mt-2 text-6xl">{$t("title")}</div>

      <div class="mt-6">
        <Route path="/">
          <Home />
        </Route>
        <Route path="/room/:roomName" component={Room} />
        <Route path="/room/:roomName/teacher" component={TeacherView} />
      </div>
    {/if}
  </div>
</Router>
