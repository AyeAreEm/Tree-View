<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideCreateEnt } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let createEntDialog;
    let createLocation;
    export let directory;
    
    let cE;
    hideCreateEnt.subscribe(value => {
        cE = value;
    })

    const createEnt = () => {
        let location = "";

        // navigator.userAgent.search("Mac") !== -1
        if (navigator.platform != "Win32") location = `${directory}/${createLocation}`;
        else location = `${directory}\\${createLocation}`;

        if (!location.endsWith("/") && !location.includes(".")) location = `${location}.txt`;

        invoke("create_location", {location})
            .then(res => {
                if (res == "1") {
                    alert("problem with creating entity");
                    return;
                }

                createLocation = "";
                createEntDialog.close();
                if (location.endsWith("/")) location = location.slice(0, -1);
                emit("refresh-add", {
                    added: location
                });
            })
    }

    onMount(() => {
        createEntDialog.addEventListener('close', () => {
            hideCreateEnt.set(true);
        });
    })

    $: {
        if (!cE) {
            createEntDialog.showModal();
        }
    }
</script>

<dialog bind:this={createEntDialog}>
    <p style="color: white;">create file or folder (end with / for folder)</p>
    <form on:submit|preventDefault={createEnt}>
        <input bind:value={createLocation} spellcheck="false" type="text" placeholder="entity name" required/><br><br>
        <button type="button" on:click={_ => createEntDialog.close()}>cancel</button>
        <input type="submit" value="create" />
    </form>
</dialog>