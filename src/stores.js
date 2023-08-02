import { writable } from "svelte/store";

export const hideSettings = writable(true);
export const pathLimit = writable(JSON.parse(localStorage.getItem("pathLimit")) || 50);