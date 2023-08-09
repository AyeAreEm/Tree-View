<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideRename } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let renameDialog;
    let renameLocation;
    export let directory;
    export let filename

    let hR;
    hideRename.subscribe(value => {
        hR = value;
    });

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

        let renamedLocation = "";
        if (navigator.platform != "Win32") renamedLocation = `${directory.replace(`/${filename}`, '')}/${renameLocation}`;
        else {
            renamedLocation = `${directory.replace(`\\${filename}`, '')}\\${renameLocation}`;
        }

        invoke('rename_location', {location: directory, newLocation: renamedLocation})
            .then((isDir, success) => {
                if (success == 1) {
                    alert("error occured when renaming this item.");
                }

                emit('refresh-rename', {
                  before: directory,
                  after: renamedLocation,
                  isDir
                });

                renameLocation = "";
                renameDialog.close();
            });
    }

    $: {
        if (!hR) {
            renameDialog.showModal();
        }
    }
</script>

<dialog bind:this={renameDialog}>
    <p style="color: white;">rename {filename}</p>
    <form on:submit|preventDefault={renameEnt}>
        <input bind:value={renameLocation} spellcheck="false" type="text" placeholder="filename" required/><br><br>
        <button type="button" on:click={_ => renameDialog.close()}>cancel</button>
        <input type="submit" value="confirm" />
    </form>
</dialog>