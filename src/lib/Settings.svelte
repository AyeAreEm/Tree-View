<script>
    import { hideSettings, pathLimit, ignores, hides, lineColor } from "../stores";
    import { onMount } from "svelte";
    import { emit } from "@tauri-apps/api/event";

    let settingsDialog;
    let selectedPin;
    let selectedLimit;
    let setIgnores = "";
    let selectedIgnore = "";
    let setHides = "";
    let selectedHide = "";

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

    let ig;
    ignores.subscribe(value => {
        ig = value;
    });

    let hI;
    hides.subscribe(value => {
        hI = value;
    });

    let lC;
    lineColor.subscribe(value => {
        lC = value;
    });
    let setLineColor = lC;

    onMount(() => {
        settingsDialog.addEventListener('close', () => {
            hideSettings.set(true);
        });

        selectedIgnore = ig[0];
    });

    const displayBgUrl = (e) => {
        if (e.key === "Enter") {
            document.body.style.backgroundImage = `url('${bgUrl}')`
        }
    }
    
    const addingINH = (e, setter, setTo, isIgnores) => {
        if (e.key === "Enter" && setter != "") {
            for (let i = 0; i < setTo.length; i++) {
                if (setTo[i].trim() === setter.trim()) {
                    alert("already ignoring.");
                    return;
                }
            }

            if (isIgnores) {
                ignores.set([...ig, setIgnores]);
                setIgnores = "";
            } else {
                hides.set([...hI, setHides]);
                setHides = "";
            }
        }
    }

    const removeIgnore = () => {
        ignores.update(value => {
            return value.filter(obj => obj !== selectedIgnore);
        })
    }

    const removeHide = () => {
        hides.update(value => {
            return value.filter(obj => obj !== selectedHide);
        })
    }
    
    const saveSettings = () => {
        localStorage.setItem("bgColor", JSON.stringify(bgColor));
        localStorage.setItem("bgUrl", JSON.stringify(bgUrl));
        localStorage.setItem("lineColor", JSON.stringify(setLineColor));
        localStorage.setItem("pinned", JSON.stringify(selectedPin));

        localStorage.setItem("pathLimit", JSON.stringify(selectedLimit));
        pathLimit.set(selectedLimit);

        localStorage.setItem("ignores", JSON.stringify(ig));

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

        <label for="line-color">line colour: </label>
        <input on:change={_ => document.querySelectorAll('line').forEach(elem => elem.style.stroke = setLineColor)} type="color" name="line-color" id="line-color" bind:value={setLineColor}/><br><br>

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

        <p>hide any items that has a given name. still searchable.</p>
        <label for="hides">hide folders/ files:</label>
        <input type="text" name="hides" spellcheck="false" bind:value={setHides} on:keydown={e => addingINH(e, setHides, hI, false)}/><br>
        <select bind:value={selectedHide}>
            {#if hI.length !== 0}
                {#each hI as hide}
                    <option value={hide}>{hide}</option>
                {/each}
            {/if}
        </select>
        <button class="caution" on:click={removeHide}>
            <svg width="15px" height="15px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                <g id="SVGRepo_iconCarrier">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M9.54405 4C8.25225 4 7.03997 4.62386 6.28912 5.67505L3.01681 10.2563C2.27174 11.2994 2.27174 12.7006 3.01681 13.7437L6.28912 18.325C7.03997 19.3761 8.25225 20 9.54405 20H18.0002C20.2093 20 22.0002 18.2091 22.0002 16V8C22.0002 5.79086 20.2093 4 18.0002 4H9.54405ZM10.2929 8.29289C10.6834 7.90237 11.3166 7.90237 11.7071 8.29289L14 10.5858L16.2929 8.29289C16.6834 7.90237 17.3166 7.90237 17.7071 8.29289C18.0976 8.68342 18.0976 9.31658 17.7071 9.70711L15.4142 12L17.7071 14.2929C18.0976 14.6834 18.0976 15.3166 17.7071 15.7071C17.3166 16.0976 16.6834 16.0976 16.2929 15.7071L14 13.4142L11.7071 15.7071C11.3166 16.0976 10.6834 16.0976 10.2929 15.7071C9.90237 15.3166 9.90237 14.6834 10.2929 14.2929L12.5858 12L10.2929 9.70711C9.90237 9.31658 9.90237 8.68342 10.2929 8.29289Z" fill="#cfcfcf"></path>
                </g>
            </svg>
        </button><br><br>

        <p>completely ignores any items that has a given name.</p>
        <label for="ignores">ignore folders/ files:</label>
        <input type="text" name="ignores" spellcheck="false" bind:value={setIgnores} on:keydown={e => addingINH(e, setIgnores, ig, true)}/><br>
        <select bind:value={selectedIgnore}>
            {#if ig.length !== 0}
                {#each ig as ignore}
                    <option value={ignore}>{ignore}</option>
                {/each}
            {/if}
        </select>
        <button class="caution" on:click={removeIgnore}>
            <svg width="15px" height="15px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                <g id="SVGRepo_iconCarrier">
                    <path fill-rule="evenodd" clip-rule="evenodd" d="M9.54405 4C8.25225 4 7.03997 4.62386 6.28912 5.67505L3.01681 10.2563C2.27174 11.2994 2.27174 12.7006 3.01681 13.7437L6.28912 18.325C7.03997 19.3761 8.25225 20 9.54405 20H18.0002C20.2093 20 22.0002 18.2091 22.0002 16V8C22.0002 5.79086 20.2093 4 18.0002 4H9.54405ZM10.2929 8.29289C10.6834 7.90237 11.3166 7.90237 11.7071 8.29289L14 10.5858L16.2929 8.29289C16.6834 7.90237 17.3166 7.90237 17.7071 8.29289C18.0976 8.68342 18.0976 9.31658 17.7071 9.70711L15.4142 12L17.7071 14.2929C18.0976 14.6834 18.0976 15.3166 17.7071 15.7071C17.3166 16.0976 16.6834 16.0976 16.2929 15.7071L14 13.4142L11.7071 15.7071C11.3166 16.0976 10.6834 16.0976 10.2929 15.7071C9.90237 15.3166 9.90237 14.6834 10.2929 14.2929L12.5858 12L10.2929 9.70711C9.90237 9.31658 9.90237 8.68342 10.2929 8.29289Z" fill="#cfcfcf"></path>
                </g>
            </svg>
        </button>

        <br><br>
        <div style="text-align: center;">
            <button class="caution" title="close without saving" on:click={_ => settingsDialog.close()}>close</button>
            <button on:click={saveSettings}>save</button>
        </div>
    </div>
</dialog>