<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { hideCreateEnt, createORDelDir } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let createEntDialog;
    let createLocation;
    
    let cE;
    hideCreateEnt.subscribe(value => {
        cE = value;
    })

    let cDD;
    createORDelDir.subscribe(value => {
        cDD = value;
    })

    const createEnt = () => {
        let location = "";

        // navigator.userAgent.search("Mac") !== -1
        if (navigator.platform != "Win32") location = `${cDD}/${createLocation}`;
        else location = `${cDD}\\${createLocation}`;

        if (!location.endsWith("/") && !location.includes(".")) location = `${location}.txt`;

        invoke("create_location", {location})
            .then(res => {
                if (res == "1") {
                    alert("problem with creating entity");
                    return;
                }

                createLocation = "";
                createEntDialog.close();
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
    <form on:submit|preventDefault={createEnt}>
        <p style="color: white;">create file or folder (end with / for folder)</p>
        <input bind:value={createLocation} spellcheck="false" type="text" placeholder="entity name" required/><br><br>
        <button type="button" on:click={_ => createEntDialog.close()}>cancel</button>
        <input type="submit" value="create" />
    </form>
</dialog>