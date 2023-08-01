<script>
    import { hideSettings } from "./stores";
    import { onMount } from "svelte";
    import { invoke } from '@tauri-apps/api/tauri';
    import { listen } from '@tauri-apps/api/event'
    import { open } from '@tauri-apps/api/dialog';
    import * as d3 from "d3";
    import G from "./lib/G.svelte";
    import Settings from "./lib/Settings.svelte";

    let homeDirectory; // selected storedDirectory
    let searchValue; // search bar
    
    // dialogs
    let addDirectoryDialog;
    let removeDirectoryDialog;

    // maybe have "artifical links" stored in localstorage either in its own or maybe better to use storedDirectories but change it to an object
    let storedDirectories = localStorage.getItem("storedDirectories") ? JSON.parse(localStorage.getItem("storedDirectories")) : [];

    let paths = ["welcome, create a directory^"]; // main path array, everything tracks back to this
    let pathTmp; // to be able to go back to the original paths when done with searching

    let width = 1400;
    let height = 775;
    let recWidth = 60;
    let recHeight = 40;

    async function handleLoadDirectory(homeDirectory) {
        let received =  await invoke("load_directory", {directory: homeDirectory});
        paths = [];
        pathTmp = [];
        
        for (let i = 0; i < received.length; i++) {
            let newString = received[i].replaceAll("\\", "/");

            received[i] = newString;
        }

        paths = received;
    }

    onMount(async () => {
        if (!storedDirectories.length) {
            return;
        }
        
        await handleLoadDirectory(storedDirectories[0].directoryPath);
    })

    const shortenPath = (path) => {
        let index = path.lastIndexOf('/');

        return path.substring(index + 1);
    }

    const handleSearchBar = (e) => {
        if (e.key === "Enter" && searchValue != "" && searchValue != "") {
            paths = pathTmp.length !== 0 ? pathTmp : paths;

            let fullName = paths.filter(obj => {
                return obj.toLowerCase().includes(searchValue.toLowerCase());
            })

            pathTmp = paths; // this the temperary value to the old paths
            paths = fullName; // this updates d3 with the found searched terms
        } else if (e.key === "Enter" && searchValue == "") {
            paths = pathTmp.length !== 0 ? pathTmp : paths;
        }
    }

    const handleAddDirectory = async () => {
        let nickname = document.forms["add-directory"]["nickname"];
        
        for (let i = 0; i < storedDirectories.length; i++) {
            if (storedDirectories[i].nickname.trim() === nickname.value.trim()) {
                alert("nickname already exists");
                return;
            }
        }

        const selectedPath = await open({
            multiple: false,
            directory: true,
            title: "choose a directory", 
        });

        if (!selectedPath) {
            return;
        }
        
        storedDirectories.push({"nickname": nickname.value, "directoryPath": selectedPath});
        storedDirectories = storedDirectories;
        localStorage.setItem("storedDirectories", JSON.stringify(storedDirectories));

        homeDirectory = storedDirectories[storedDirectories.length - 1].directoryPath;
        await handleLoadDirectory(homeDirectory);

        nickname.value = "";
        addDirectoryDialog.close();
    }

    const handleRemoveDirectory = () => {
        if (storedDirectories.length === 0) {
            return;
        }

        let nickname = document.forms["remove-directory"]["nickname"];

        storedDirectories = storedDirectories.filter(obj => {
            return obj.nickname !== nickname.value;
        });

        localStorage.setItem("storedDirectories", JSON.stringify(storedDirectories));

        if (storedDirectories.length !== 0) {
            homeDirectory = storedDirectories[storedDirectories.length - 1].directoryPath
            handleLoadDirectory(homeDirectory);
        } else {
            homeDirectory = "";
            paths = ["welcome, create a directory^"];
        }

        nickname.value = "";
        removeDirectoryDialog.close();
    }
    
    const handleContext = async (directory, filename) => {
        // rmb to actually make a context menu, and instead have this code below be one of the onclick functions

        await invoke("make_properties_window", {filename});

        setTimeout( async () => {
            await invoke("get_properties_command", {directory, filename});
        }, 300);
    }

    listen('refresh-remove', async (event) => {
        if (event.payload.isDir) {
            await handleLoadDirectory(homeDirectory);
            return;
        }

        paths = paths.filter(obj => {
            return obj !== event.payload.removed;
        });
    });

    listen('refresh-add', (event) => {
        paths = paths.concat([event.payload.added]);
    });

    $: root = d3.stratify().path((d) => d)(paths);
   // @ts-ignore
    $: treeLayout = d3.tree().size([width, height - 40])(root);
</script>

<main>
    <ul class="unselectable" unselectable="on">
        <li>
            <!-- refactor this to drag and drop the options to pin them -->
            <select bind:value={homeDirectory} id="homeDirectory" title="choose directory" on:change={async () => await handleLoadDirectory(homeDirectory)}>
                {#each storedDirectories as storedDirectory}
                    <option value={storedDirectory.directoryPath}>{storedDirectory.nickname}</option>
                {/each}
            </select>
        </li>
        <li>
            <button on:click={addDirectoryDialog.showModal()} title="add parent directory">+</button>
        </li>
        <li>
            <button class="caution" on:click={removeDirectoryDialog.showModal()} title="remove parent directory">-</button>
        </li>
        <li style="float: right;">
            <input type="text" id="search" spellcheck="false" placeholder="search" bind:value={searchValue} on:keydown={handleSearchBar}/>
        </li>
        <li style="float: right;">
            <button on:click={_ => handleLoadDirectory(homeDirectory)} title="refresh tree">
                <svg width="15px" height="15px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                    <g id="SVGRepo_iconCarrier">
                        <path d="M21 3V8M21 8H16M21 8L18 5.29168C16.4077 3.86656 14.3051 3 12 3C7.02944 3 3 7.02944 3 12C3 16.9706 7.02944 21 12 21C16.2832 21 19.8675 18.008 20.777 14" stroke="#cfcfcf" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path>
                    </g>
                </svg>
            </button>
        </li>
    </ul>

    <button title="settings" style="position: absolute; bottom: 0; left: 0; margin: 0.5em;" on:click={_ => hideSettings.set(false)}>
        <svg width="18px" height="18px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
            <g id="SVGRepo_iconCarrier">
                <circle cx="12" cy="12" r="3" stroke="#cfcfcf" stroke-width="1.5"></circle>
                <path d="M3.66122 10.6392C4.13377 10.9361 4.43782 11.4419 4.43782 11.9999C4.43781 12.558 4.13376 13.0638 3.66122 13.3607C3.33966 13.5627 3.13248 13.7242 2.98508 13.9163C2.66217 14.3372 2.51966 14.869 2.5889 15.3949C2.64082 15.7893 2.87379 16.1928 3.33973 16.9999C3.80568 17.8069 4.03865 18.2104 4.35426 18.4526C4.77508 18.7755 5.30694 18.918 5.83284 18.8488C6.07287 18.8172 6.31628 18.7185 6.65196 18.5411C7.14544 18.2803 7.73558 18.2699 8.21895 18.549C8.70227 18.8281 8.98827 19.3443 9.00912 19.902C9.02332 20.2815 9.05958 20.5417 9.15224 20.7654C9.35523 21.2554 9.74458 21.6448 10.2346 21.8478C10.6022 22 11.0681 22 12 22C12.9319 22 13.3978 22 13.7654 21.8478C14.2554 21.6448 14.6448 21.2554 14.8478 20.7654C14.9404 20.5417 14.9767 20.2815 14.9909 19.9021C15.0117 19.3443 15.2977 18.8281 15.7811 18.549C16.2644 18.27 16.8545 18.2804 17.3479 18.5412C17.6837 18.7186 17.9271 18.8173 18.1671 18.8489C18.693 18.9182 19.2249 18.7756 19.6457 18.4527C19.9613 18.2106 20.1943 17.807 20.6603 17C20.8677 16.6407 21.029 16.3614 21.1486 16.1272M20.3387 13.3608C19.8662 13.0639 19.5622 12.5581 19.5621 12.0001C19.5621 11.442 19.8662 10.9361 20.3387 10.6392C20.6603 10.4372 20.8674 10.2757 21.0148 10.0836C21.3377 9.66278 21.4802 9.13092 21.411 8.60502C21.3591 8.2106 21.1261 7.80708 20.6601 7.00005C20.1942 6.19301 19.9612 5.7895 19.6456 5.54732C19.2248 5.22441 18.6929 5.0819 18.167 5.15113C17.927 5.18274 17.6836 5.2814 17.3479 5.45883C16.8544 5.71964 16.2643 5.73004 15.781 5.45096C15.2977 5.1719 15.0117 4.6557 14.9909 4.09803C14.9767 3.71852 14.9404 3.45835 14.8478 3.23463C14.6448 2.74458 14.2554 2.35523 13.7654 2.15224C13.3978 2 12.9319 2 12 2C11.0681 2 10.6022 2 10.2346 2.15224C9.74458 2.35523 9.35523 2.74458 9.15224 3.23463C9.05958 3.45833 9.02332 3.71848 9.00912 4.09794C8.98826 4.65566 8.70225 5.17191 8.21891 5.45096C7.73557 5.73002 7.14548 5.71959 6.65205 5.4588C6.31633 5.28136 6.0729 5.18269 5.83285 5.15108C5.30695 5.08185 4.77509 5.22436 4.35427 5.54727C4.03866 5.78945 3.80569 6.19297 3.33974 7C3.13231 7.35929 2.97105 7.63859 2.85138 7.87273" stroke="#cfcfcf" stroke-width="1.5" stroke-linecap="round"></path>
            </g>
        </svg>
    </button>

    <dialog bind:this={addDirectoryDialog}>
        <p style="color: white;">provide an existing directory</p>
        <form on:submit|preventDefault={handleAddDirectory} name="add-directory">
            <input spellcheck="false" type="text" name="nickname" placeholder="directory nickname" required/><br><br>
            <button type="button" on:click={_ => addDirectoryDialog.close()}>cancel</button>
            <input type="submit" value="add" />
        </form>
    </dialog>
    <dialog bind:this={removeDirectoryDialog}>
        <p style="color: white;">removing won't delete from the device</p>
        <form on:submit|preventDefault={handleRemoveDirectory} name="remove-directory">
            <input spellcheck="false" type="text" name="nickname" placeholder="directory nickname" required/><br><br>
            <button type="button" on:click={_ => removeDirectoryDialog.close()}>cancel</button>
            <input class="caution" type="submit" value="remove"/>
        </form>
    </dialog>
    <Settings />

    <!-- refactor this, remove repeating code, maybe put some of it inside the G.svelte file -->
    <svg width={width} height={height}  viewBox="0, 0, 1400, 825" xmlns="http://www.w3.org/2000/svg">
        {#each root.descendants() as node}
            {@const short = shortenPath(node.id)}
            {#if node.id.lastIndexOf('.') == -1 || short.lastIndexOf('.') == 0}
                <G id={short} on:sglclick={async () => await invoke("open_location", {location: node.data, application: ""})} on:dblclick={async () => await handleContext(node.data, short)}>
                    <svg id={node.id} class="node" x={node.x - (recWidth / 2)} y={node.y} xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" xml:space="preserve" width={recWidth} height={recHeight} fill="#000000">
                        <g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                        <g id="SVGRepo_iconCarrier">
                            <path id="SVGCleanerId_0" style="fill:#FFC36E;" d="M183.295,123.586H55.05c-6.687,0-12.801-3.778-15.791-9.76l-12.776-25.55 l12.776-25.55c2.99-5.982,9.103-9.76,15.791-9.76h128.246c6.687,0,12.801,3.778,15.791,9.76l12.775,25.55l-12.776,25.55 C196.096,119.808,189.983,123.586,183.295,123.586z"></path>
                            <g>
                                <path id="SVGCleanerId_0_1_" style="fill:#FFC36E;" d="M183.295,123.586H55.05c-6.687,0-12.801-3.778-15.791-9.76l-12.776-25.55 l12.776-25.55c2.99-5.982,9.103-9.76,15.791-9.76h128.246c6.687,0,12.801,3.778,15.791,9.76l12.775,25.55l-12.776,25.55 C196.096,119.808,189.983,123.586,183.295,123.586z"></path>
                            </g>
                            <path style="fill:#EFF2FA;" d="M485.517,70.621H26.483c-4.875,0-8.828,3.953-8.828,8.828v44.138h476.69V79.448 C494.345,74.573,490.392,70.621,485.517,70.621z"></path>
                            <rect x="17.655" y="105.931" style="fill:#E1E6F2;" width="476.69" height="17.655"></rect>
                            <path style="fill:#FFD782;" d="M494.345,88.276H217.318c-3.343,0-6.4,1.889-7.895,4.879l-10.336,20.671 c-2.99,5.982-9.105,9.76-15.791,9.76H55.05c-6.687,0-12.801-3.778-15.791-9.76L28.922,93.155c-1.495-2.99-4.552-4.879-7.895-4.879 h-3.372C7.904,88.276,0,96.18,0,105.931v335.448c0,9.751,7.904,17.655,17.655,17.655h476.69c9.751,0,17.655-7.904,17.655-17.655 V105.931C512,96.18,504.096,88.276,494.345,88.276z"></path>
                            <path style="fill:#FFC36E;" d="M485.517,441.379H26.483c-4.875,0-8.828-3.953-8.828-8.828l0,0c0-4.875,3.953-8.828,8.828-8.828 h459.034c4.875,0,8.828,3.953,8.828,8.828l0,0C494.345,437.427,490.392,441.379,485.517,441.379z"></path>
                            <path style="fill:#EFF2FA;" d="M326.621,220.69h132.414c4.875,0,8.828-3.953,8.828-8.828v-70.621c0-4.875-3.953-8.828-8.828-8.828 H326.621c-4.875,0-8.828,3.953-8.828,8.828v70.621C317.793,216.737,321.746,220.69,326.621,220.69z"></path>
                            <path style="fill:#C7CFE2;" d="M441.379,167.724h-97.103c-4.875,0-8.828-3.953-8.828-8.828l0,0c0-4.875,3.953-8.828,8.828-8.828 h97.103c4.875,0,8.828,3.953,8.828,8.828l0,0C450.207,163.772,446.254,167.724,441.379,167.724z"></path>
                            <path style="fill:#D7DEED;" d="M441.379,203.034h-97.103c-4.875,0-8.828-3.953-8.828-8.828l0,0c0-4.875,3.953-8.828,8.828-8.828 h97.103c4.875,0,8.828,3.953,8.828,8.828l0,0C450.207,199.082,446.254,203.034,441.379,203.034z"></path>
                        </g>
                    </svg>
                </G>
            {:else}
                <G id={short} on:sglclick={async () => await invoke("open_location", {location: node.data, application: ""})} on:dblclick={async () => await handleContext(node.data, short)}>
                    <svg id={node.id} class="node" x={node.x - (recHeight / 2)} y={node.y - 10} xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width={recHeight} height={recWidth} viewBox="0 0 64 64" xml:space="preserve" fill="#000000">
                        <g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                        <g id="SVGRepo_iconCarrier">
                            <g>
                                <g>
                                    <polygon fill="#cfcfcf" points="46,3.414 46,14 56.586,14 "></polygon>
                                    <path fill="#cfcfcf" d="M45,16c-0.553,0-1-0.447-1-1V2H8C6.896,2,6,2.896,6,4v56c0,1.104,0.896,2,2,2h48c1.104,0,2-0.896,2-2V16 H45z"></path>
                                </g>
                                <path fill="#394240" d="M14,26c0,0.553,0.447,1,1,1h34c0.553,0,1-0.447,1-1s-0.447-1-1-1H15C14.447,25,14,25.447,14,26z"></path>
                                <path fill="#394240" d="M49,37H15c-0.553,0-1,0.447-1,1s0.447,1,1,1h34c0.553,0,1-0.447,1-1S49.553,37,49,37z"></path>
                                <path fill="#394240" d="M49,43H15c-0.553,0-1,0.447-1,1s0.447,1,1,1h34c0.553,0,1-0.447,1-1S49.553,43,49,43z"></path>
                                <path fill="#394240" d="M49,49H15c-0.553,0-1,0.447-1,1s0.447,1,1,1h34c0.553,0,1-0.447,1-1S49.553,49,49,49z"></path>
                                <path fill="#394240" d="M49,31H15c-0.553,0-1,0.447-1,1s0.447,1,1,1h34c0.553,0,1-0.447,1-1S49.553,31,49,31z"></path>
                                <path fill="#394240" d="M15,20h16c0.553,0,1-0.447,1-1s-0.447-1-1-1H15c-0.553,0-1,0.447-1,1S14.447,20,15,20z"></path>
                                <path fill="#394240" d="M59.706,14.292L45.708,0.294C45.527,0.112,45.277,0,45,0H8C5.789,0,4,1.789,4,4v56c0,2.211,1.789,4,4,4h48 c2.211,0,4-1.789,4-4V15C60,14.723,59.888,14.473,59.706,14.292z M46,3.414L56.586,14H46V3.414z M58,60c0,1.104-0.896,2-2,2H8 c-1.104,0-2-0.896-2-2V4c0-1.104,0.896-2,2-2h36v13c0,0.553,0.447,1,1,1h13V60z"></path>
                                <polygon opacity="0.15" fill="#231F20" points="46,3.414 56.586,14 46,14 "></polygon>
                            </g>
                        </g>
                    </svg>
                </G>
            {/if}
        {/each}
        {#each root.links() as link}
            <line x1={link.source.x} y1={link.source.y + recHeight} x2={link.target.x} y2={link.target.y} stroke="#adadad"></line>
        {/each}
    </svg>
</main>

<style>
    .node {
        cursor: pointer;
    }
</style>
