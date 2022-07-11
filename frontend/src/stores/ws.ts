import { readable } from "svelte/store";
import type { WSMessage } from "bindings/WSMessage";
import type { ClientMessage } from "bindings/ClientMessage";

let ws: WebSocket;

const initWS = () => {
  const ws = new WebSocket("ws://localhost:8000/ws");
  ws.onopen = () => {
    console.log("Starting WebSocket...");
  };
  ws.onmessage = (ev) => {
    const msg = ev.data as ClientMessage;
    console.log("Recieved: ", JSON.stringify(msg));
  };
  // ws.onerror
  // ws.onclose
  return ws;
};

export const wsMessageStore = readable("", () => {
  ws = initWS();

  return () => {
    console.log("Closing WebSocket...");
    ws.close();
  };
});

export const sendWSMessage = (msg: WSMessage) => {
  ws.send(JSON.stringify(msg));
};
