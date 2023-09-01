<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideDeleteEnt } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let deleteEntDialog;
    export let directory;
    
    let cE;
    hideDeleteEnt.subscribe(value => {
        cE = value;
    })

    const deleteEnt = () => {
        invoke("remove_location", {location: directory})
            .then(([isDir, success]) => {
                if (success == 1) {
                    alert("error occured when deleting. ensure no program is currently using it.");
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
        if (!cE) {
            deleteEntDialog.showModal();
        }
    }
</script>

<dialog bind:this={deleteEntDialog} class="unselectable" unselectable="on">
    <p style="color: white;">delete this?</p>
    <button on:click={_ => deleteEntDialog.close()}>no</button>
    <button class="caution" on:click={deleteEnt}>yes</button>
</dialog>