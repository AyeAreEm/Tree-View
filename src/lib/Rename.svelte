<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideRename } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let renameDialog;
    let renameLocation;
    export let directory;
    export let filename

    onMount(() => {
        renameDialog.addEventListener('close', () => {
            hideRename.set(true);
        })
    })

    const renameEnt = () => {
        if (renameLocation.includes("/") || renameLocation.includes("\\")) {
            alert("/ and \\ aren't allowed");
            return;
        }

        invoke('rename_location', {location: directory, newLocation: renameLocation, filename})
            .then(([isDir, renamedLocation, success]) => {
                if (success == false) {
                    alert("error: can't find path or currently being used");
                    return;
                }

                emit('refresh-rename', {
                  before: directory,
                  after: renamedLocation,
                  isDir
                });

                emit('show-popup');

                renameLocation = "";
                renameDialog.close();
            });
    }

    $: {
        if (!$hideRename) {
            renameDialog.showModal();
        }
    }
</script>

<dialog bind:this={renameDialog} class="unselectable" unselectable="on">
    <p style="color: white;">rename {filename}</p>
    <form on:submit|preventDefault={renameEnt}>
        <input bind:value={renameLocation} spellcheck="false" type="text" placeholder="filename" required/><br><br>
        <button type="button" on:click={_ => renameDialog.close()}>cancel</button>
        <input type="submit" value="confirm" />
    </form>
</dialog>