<script lang="ts">
  import { Router, Route, Link } from "svelte-navigator";
  import Home from "./routes/Home.svelte";
  import Room from "./routes/Room.svelte";
</script>

<Router primary={false}>
  <div class="container mx-auto mt-4 rounded bg-slate-50 p-4">
    <nav>
      <Link to="/">Home</Link>
    </nav>
    <div class="text-6xl">Interactive Class</div>

    <div class="mt-8">
      <Route path="/">
        <Home />
      </Route>
      <Route path="/room/:roomName" component={Room} />
    </div>
  </div>
</Router>
