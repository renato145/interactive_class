// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { RoomInfo } from "./RoomInfo";

export type ClientMessage = { kind: "RoomInfo", payload: RoomInfo } | { kind: "Error", payload: string };