import { writable } from "svelte/store";
import type { WSMessage } from "bindings/WSMessage";
import type { ClientMessage } from "bindings/ClientMessage";

let ws: WebSocket;

export const wsStatusStore = writable<
  "disconnected" | "started" | "connected2room" | "working"
>("disconnected");

const initWS = () => {
  const ws = new WebSocket("ws://localhost:8000/ws");
  ws.onopen = () => {
    console.log("Starting WebSocket...");
    wsStatusStore.set("started");
  };
  ws.onmessage = (ev) => {
    wsStatusStore.set("working");
    const msg = JSON.parse(ev.data);
    console.log("Recieved: ", msg);
    wsMessageStore.set(msg);
    wsStatusStore.set("connected2room");
  };
  // ws.onerror
  ws.onclose = () => {
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
