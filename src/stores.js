import { writable } from "svelte/store";

export const hideSettings = writable(true);

export const hideCreateEnt = writable(true);
export const hideDeleteEnt = writable(true);

export const hideRename = writable(true);

export const pathLimit = writable(JSON.parse(localStorage.getItem("pathLimit")) || 50);
export const ignores = writable(JSON.parse(localStorage.getItem("ignores")) || []);
export const hides = writable(JSON.parse(localStorage.getItem("hides")) || []);
export const lineColor = writable(JSON.parse(localStorage.getItem("lineColor")) || "#adadad");