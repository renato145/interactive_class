import { writable } from "svelte/store";
import type { WSMessage } from "bindings/WSMessage";
import type { ClientMessage } from "bindings/ClientMessage";

let ws: WebSocket;

export const wsStatusStore = writable<"disconnected" | "connected" | "working">(
  "disconnected"
);

const initWS = () => {
  const ws = new WebSocket("ws://localhost:8000/ws");
  ws.onopen = () => {
    console.log("Starting WebSocket...");
    wsStatusStore.set("connected");
  };
  ws.onmessage = (ev) => {
    wsStatusStore.set("working");
    const msg = JSON.parse(ev.data);
    console.log("Recieved: ", msg);
    wsMessageStore.set(msg);
    wsStatusStore.set("connected");
  };
  // ws.onerror
  ws.onclose = (ev) => {
    wsStatusStore.set("disconnected");
  };
  return ws;
};

export const wsMessageStore = writable<ClientMessage>(null, () => {
  ws = initWS();

  return () => {
    console.log("Closing WebSocket...");
    ws.close();
  };
});

export const sendWSMessage = (msg: WSMessage) => {
  ws.send(JSON.stringify(msg));
};
