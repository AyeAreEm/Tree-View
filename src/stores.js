import { writable } from "svelte/store";

export const hideSettings = writable(true);

export const hideCreateEnt = writable(true);
export const hideDeleteEnt = writable(true);

export const hideRename = writable(true);

export const storedDirectories = writable(JSON.parse(localStorage.getItem("storedDirectories")) || []);
export const ignores = writable(JSON.parse(localStorage.getItem("ignores")) || []);
export const lineColor = writable(JSON.parse(localStorage.getItem("lineColor")) || "#adadad");