<script>
    import { emit } from "@tauri-apps/api/event";
    import { hideSettings } from "../stores";
    import { pathLimit } from "../stores";
    import { onMount } from "svelte";

    let settingsDialog;
    let selectedPin;
    let selectedLimit;

    export let bgColor;
    export let bgUrl;
    export let storedDirectories;
    export let pinned;

    let pL;
    pathLimit.subscribe(value => {
        pL = value;
    });

    let hS;
    hideSettings.subscribe(value => {
        hS = value;
    });

    onMount(() => {
        settingsDialog.addEventListener('close', () => {
            hideSettings.set(true);
        });
    });

    const displayBgUrl = (e) => {
        if (e.key === "Enter") {
            document.body.style.backgroundImage = `url('${bgUrl}')`
        }
    }
    
    const saveSettings = () => {
        localStorage.setItem("bgColor", JSON.stringify(bgColor));
        localStorage.setItem("bgUrl", JSON.stringify(bgUrl));
        localStorage.setItem("pinned", JSON.stringify(selectedPin));

        localStorage.setItem("pathLimit", JSON.stringify(selectedLimit));
        pathLimit.set(selectedLimit);
        emit("refresh");

        settingsDialog.close();
    }

    $: {
        if (!hS) {
            settingsDialog.showModal();
        }
    }
</script>

<dialog bind:this={settingsDialog} style="text-align: left;">
    <div style="color: white;">
        <p style="text-align: center;">reminder: save your changes</p>
        <hr><br>

        <label for="bg-color">background colour: </label>
        <input on:change={_ => document.body.style.backgroundColor = bgColor} type="color" name="bg-color" id="bg-color" bind:value={bgColor}/><br><br>

        <label for="bg-url">background image: </label>
        <input spellcheck="false" type="text" name="bg-url" id="bg-url" bind:value={bgUrl} on:keydown={e => displayBgUrl(e)}/><br><br>

        <label for="pin">pin on launch: </label>
        <select name="pin" bind:value={selectedPin}>
            {#each storedDirectories as storedDirectory}
                {#if storedDirectory.nickname == pinned}
                    <option selected value={storedDirectory.nickname}>{storedDirectory.nickname}</option>
                {:else}
                    <option value={storedDirectory.nickname}>{storedDirectory.nickname}</option>
                {/if}
            {/each}
        </select><br><br>

        <hr>

        <p>changes number of folders / files that are allowed to be rendered. <i style="color: #E0115F;">not recommended</i></p>
        <details>
            <summary><i>note</i></summary>
            <p>the rest of the items will render, just not the ones over the limit</p>
        </details>
        <label for="limit">entity limit: </label>
        <select name="limit" bind:value={selectedLimit}>
            {#each {length: 100} as _, index}
                {#if index+1 == pL}
                    <option selected value={index + 1}>{index + 1}</option>
                {:else}
                    <option value={index + 1}>{index + 1}</option>
                {/if}
            {/each}
        </select><br><br>

        <div style="text-align: center;">
            <button class="caution" title="close without saving" on:click={_ => settingsDialog.close()}>close</button>
            <button on:click={saveSettings}>save</button>
        </div>
    </div>
</dialog>