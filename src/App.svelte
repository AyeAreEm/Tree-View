<script>
    import { onMount } from "svelte";
    import { invoke } from '@tauri-apps/api/tauri'
    import * as d3 from "d3";

    let homeDirectory;
    let directoryDialog;
    let received;

    let paths = ["yes.js"];

    let width = 1000;
	let height = 550;

    let recWidth = 60;
    let recHeight = 40;

    async function handleLoadDirectory(homeDirectory) {
        received =  await invoke("load_directory", {directory: homeDirectory});
        paths = [];
        
        for (let i = 0; i < received.length; i++) {
            let newString = received[i].replaceAll("\\", "/");
            received[i] = newString;
        }

        paths = received;
    }

    (async () => {
        await handleLoadDirectory("C:/Users/agaye/Desktop/testing");
     })()

     const shortenPath = (path) => {
         let index = path.lastIndexOf('/');
         return path.substring(index + 1);
     }

    $: root = d3.stratify().path((d) => d)(paths);
    $: treeLayout = d3.tree().size([width, height - 40])(root);

</script>

<main style="position: relative;">
    <ul>
        <li>
            <select bind:value={homeDirectory} id="homeDirectory" title="choose directory" on:change={async () => await handleLoadDirectory(homeDirectory)}>
                <option value="D:/brain/Comp Sci">Comp Sci</option>
                <option value="D:/brain/Cloud Vault">Cloud Vault</option>
            </select>
        </li>
        <li>
            <button on:click={() => directoryDialog.showModal()} title="add parent directory">+</button>
        </li>
        <li style="float: right;">
            <input type="text" id="search" spellcheck="false" placeholder="search"/>
        </li>
    </ul>

    <dialog bind:this={directoryDialog}>
        <input type="text" placeholder="directory nickname" /><br><br>
        <input type="text" placeholder="path to directory" /><br><br>
        <button on:click={() => directoryDialog.close()}>close</button>
    </dialog>
    <svg width={width} height={height}  viewBox="0, 0, 1000, 600" xmlns="http://www.w3.org/2000/svg">
        {#each root.descendants() as node}
            {console.log(node)}
            {#if node.id.lastIndexOf('.') == -1}
                <g>
                    <title>{shortenPath(node.id)}</title>
                    <!-- <rect class="node" x={node.x - (recWidth / 2)} y={node.y} width={recWidth} height={recHeight} rx="5" on:dblclick={() => console.log('clicked;')}/> -->
                    <svg class="node" x={node.x - (recWidth / 2)} y={node.y} version="1.1" id={shortenPath(node.id)} xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" xml:space="preserve" width={recWidth} height={recHeight} fill="#000000">
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
                    <!-- <text class="label" x={node.x} y={node.y + 9} font-size="10px" fill="white">{node.id}</text> -->
                </g>
            {:else}
                <g>
                    <title>{shortenPath(node.id)}</title>
                    <svg class="node" x={node.x - (recHeight / 2)} y={node.y - 10} version="1.0" id={shortenPath(node.id)} xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width={recHeight} height={recWidth} viewBox="0 0 64 64" enable-background="new 0 0 64 64" xml:space="preserve" fill="#000000">
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
                </g>
            {/if}
        {/each}
        {#each root.links() as link}
            <line x1={link.source.x} y1={link.source.y + recHeight} x2={link.target.x} y2={link.target.y} stroke="#adadad"></line>
        {/each}
    </svg>
</main>

<style>
    /* .label {
        text-anchor: middle;
        pointer-events: none;
    } */

    .node {
        cursor: pointer;
    }
</style>
