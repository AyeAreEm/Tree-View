<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideDeleteEnt, createORDelDir } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let deleteEntDialog;
    
    let cE;
    hideDeleteEnt.subscribe(value => {
        cE = value;
    })

    let cDD;
    createORDelDir.subscribe(value => {
        cDD = value;
    })

    const deleteEnt = () => {
        invoke("remove_location", {location: cDD})
            .then((isDir, success) => {
                if (success == 1) {
                    alert("error occured when deleting. ensure no program is currently using it.");
                    return;
                }

                emit("refresh-remove", {
                    removed: cDD,
                    isDir
                });
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

<dialog bind:this={deleteEntDialog}>
    <p style="color: white;">delete this?</p>
    <button on:click={_ => deleteEntDialog.close()}>no</button>
    <button class="caution" on:click={deleteEnt}>yes</button>
</dialog>