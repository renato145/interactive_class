import { writable } from "svelte/store";
import type { WSMessage } from "bindings/WSMessage";
import type { ClientMessage } from "bindings/ClientMessage";
import type { ConnectionType } from "bindings/ConnectionType";
import type { CupColor } from "bindings/CupColor";

export interface WSData {
  room_name: string;
  status: "disconnected" | "connected" | "working";
  connections: number;
  cups: {
    green: number;
    yellow: number;
    red: number;
  };
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

  const chooseCup = (cup: CupColor) => {
    sendWSMessage({
      task: "ChooseCup",
      payload: cup,
    });
  };

  const initWS = () => {
    const ws = new WebSocket("ws://localhost:8000/ws");
    ws.onopen = () => {
      console.log("Starting WebSocket...");
      wsStore.update((d) => ({ ...d, status: "connected" }));
      sendWSMessage({
        task: "RoomConnect",
        payload: {
          room_name,
          connection_type,
        },
      });
    };
    ws.onmessage = (ev) => {
      wsStore.update((d) => ({ ...d, status: "working" }));
      const msg = JSON.parse(ev.data) as ClientMessage;
      console.log("Recieved: ", msg);
      switch (msg.kind) {
        case "RoomInfo":
          wsStore.update((d) => ({
            ...d,
            connections: msg.payload.connections,
            cups: {
              green: msg.payload.green,
              yellow: msg.payload.yellow,
              red: msg.payload.red,
            },
          }));
          break;

        case "Error":
          break;

        default:
          break;
      }
      wsStore.update((d) => ({ ...d, status: "connected" }));
    };
    // ws.onerror
    ws.onclose = () => {
      wsStore.update((d) => ({ ...d, status: "disconnected" }));
    };

    return ws;
  };

  const wsStore = writable<WSData>(
    {
      room_name,
      status: "disconnected",
      connections: 0,
      cups: {
        green: 0,
        yellow: 0,
        red: 0,
      },
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

  return { wsStore, chooseCup };
};
