import { readable } from "svelte/store";

const initWS = () => {
  const ws = new WebSocket("ws://localhost:8000/ws");
  ws.onopen = (ev) => {
    console.log(ev);
  };
  return ws;
};

export const wsMessageStore = readable("", () => {
  const ws = initWS();

  return () => {
    console.log("Closing WebSocket...");
    ws.close();
  };
});
