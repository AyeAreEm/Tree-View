<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideCreateEnt } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let createEntDialog;
    let createLocation;
    export let directory;

    const createEnt = () => {
        invoke("create_location", {directory, filename: createLocation})
            .then(([path, success]) => {
                if (success == false) {
                    alert("error: can't create item");
                    return;
                }

                emit("refresh-add", {
                    added: path 
                });

                emit("show-popup");

                createLocation = "";
                createEntDialog.close();
            });
    }

    onMount(() => {
        createEntDialog.addEventListener('close', () => {
            hideCreateEnt.set(true);
        });
    })

    $: {
        if (!$hideCreateEnt) {
            createEntDialog.showModal();
        }
    }
</script>

<dialog bind:this={createEntDialog} class="unselectable" unselectable="on">
    <p style="color: white;">create file or folder (end with / for folder)</p>
    <form on:submit|preventDefault={createEnt}>
        <input bind:value={createLocation} spellcheck="false" type="text" placeholder="entity name" required/><br><br>
        <button type="button" on:click={_ => createEntDialog.close()}>cancel</button>
        <input type="submit" value="create" />
    </form>
</dialog>