<script>
    import { hideSettings } from "../stores";
    import { onMount } from "svelte";

    let settingsDialog;
    let bgColor = "#252525";
    let bgUrl;

    let hS;
    hideSettings.subscribe(value => {
        hS = value;
    });

    onMount(() => {
        settingsDialog.addEventListener('close', () => {
            hideSettings.set(true);
        });

        bgUrl.addEventListener('keydown', (e) => {
            if (e.key === "Enter") {
                document.body.style.backgroundImage = `url('${bgUrl.value}')`
            }
        });
    });

    $: {
        if (!hS) {
            settingsDialog.showModal();
        }
    }
</script>

<dialog bind:this={settingsDialog} style="text-align: left;">
    <label style="color: white;" for="bg-color">Background Colour: </label>
    <input on:change={_ => document.body.style.backgroundColor = bgColor} type="color" name="bg-color" id="bg-color" bind:value={bgColor}/><br><br>
    <label style="color: white;" for="bg-url">Background Image: </label>
    <input spellcheck="false" type="text" name="bg-url" id="bg-url" bind:this={bgUrl}/><br><br>

    <div style="text-align: center;">
        <button on:click={_ => settingsDialog.close()}>close</button>
    </div>
</dialog>