<script lang="ts">
  import { Router, Route, Link } from "svelte-navigator";
  import Home from "./routes/Home.svelte";
  import Room from "./routes/Room.svelte";
  import TeacherView from "./routes/TeacherView.svelte";
</script>

<Router primary={false}>
  <div class="container mx-auto my-4 rounded bg-slate-50 p-4">
    <nav>
      <Link to="/">Home</Link>
    </nav>
    <div class="mt-2 text-6xl">Interactive Class</div>

    <div class="mt-6">
      <Route path="/">
        <Home />
      </Route>
      <Route path="/room/:roomName" component={Room} />
      <Route path="/room/:roomName/teacher" component={TeacherView} />
    </div>
  </div>
</Router>
