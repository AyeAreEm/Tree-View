<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideDeleteEnt } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let deleteEntDialog;
    export let directory;

    const deleteEnt = () => {
        invoke("remove_location", {location: directory})
            .then(([isDir, success]) => {
                if (success == false) {
                    alert("error: can't delete. may be open elsewhere.");
                    return;
                }

                emit("refresh-remove", {
                    removed: directory,
                    isDir
                });

                emit("show-popup");
            });

        deleteEntDialog.close();
    }

    onMount(() => {
        deleteEntDialog.addEventListener('close', () => {
            hideDeleteEnt.set(true);
        });
    })

    $: {
        if (!$hideDeleteEnt) {
            deleteEntDialog.showModal();
        }
    }
</script>

<dialog bind:this={deleteEntDialog} class="unselectable" unselectable="on">
    <p style="color: white;">delete this?</p>
    <button on:click={_ => deleteEntDialog.close()}>no</button>
    <button class="caution" on:click={deleteEnt}>yes</button>
</dialog>