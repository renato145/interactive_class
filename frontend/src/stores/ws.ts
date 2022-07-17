import { writable } from "svelte/store";
import type { WSMessage } from "bindings/WSMessage";
import type { ClientMessage } from "bindings/ClientMessage";
import type { ConnectionType } from "bindings/ConnectionType";

export interface WSData {
  room_name: string;
  status: "disconnected" | "connected" | "working";
  connections: number;
}

/**
 * @param roomname
 * @returns wsMessageStore and sendWSMessage
 */
export const getWSStore = (
  room_name: string,
  connection_type: ConnectionType
) => {
  let ws: WebSocket;

  const sendWSMessage = (msg: WSMessage) => {
    ws.send(JSON.stringify(msg));
  };

  const initWS = () => {
    const ws = new WebSocket("ws://localhost:8000/ws");
    ws.onopen = () => {
      console.log("Starting WebSocket...");
      store.update((d) => ({ ...d, status: "connected" }));
      sendWSMessage({
        task: "RoomConnect",
        payload: {
          room_name,
          connection_type,
        },
      });
    };
    ws.onmessage = (ev) => {
      store.update((d) => ({ ...d, status: "working" }));
      const msg = JSON.parse(ev.data) as ClientMessage;
      console.log("Recieved: ", msg);
      switch (msg.kind) {
        case "RoomInfo":
          store.update((d) => ({
            ...d,
            connections: +msg.payload.connections,
          }));
          break;

        case "Error":
          break;

        default:
          break;
      }
      store.update((d) => ({ ...d, status: "connected" }));
    };
    // ws.onerror
    ws.onclose = () => {
      store.update((d) => ({ ...d, status: "disconnected" }));
    };
    return ws;
  };

  const store = writable<WSData>(
    {
      room_name,
      status: "disconnected",
      connections: 0,
    },
    () => {
      {
        ws = initWS();

        return () => {
          console.log("Closing WebSocket...");
          ws.close();
        };
      }
    }
  );

  return store;
};
