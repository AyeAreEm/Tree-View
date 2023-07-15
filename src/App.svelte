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

    let recWidth = 40;
    let recHeight = 60;

    (async () => {
        received =  await invoke("load_directory", {directory: "D:/brain/Comp Sci"});
        paths.pop();

        
        for (let i = 0; i < received.length - 1; i++) {
            let newString = received[i].replaceAll("\\", "/");
            paths.push(newString)
            paths = paths;
        }
     })()

     const shortenPath = (path) => {
         let index = path.lastIndexOf('/');
         return path.substring(index + 1);
     }

    $: root = d3.stratify().path((d) => d)(paths);
    $: treeLayout = d3.tree().size([width, height - 40])(root);
    // try to figure out how to do it with nodeSize instead, that way we have a constant or more amount of space between each node.

</script>

<main style="position: relative;">
    <ul>
        <li>
            <select bind:value={homeDirectory} id="homeDirectory" title="choose directory" on:change={() => console.log(homeDirectory)}>
                <option value="./notes">Notes</option>
                <option value="./projects" >Projects</option>
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
                <!-- {console.log(node)} -->
                <g>
                    <title>{shortenPath(node.id)}</title>
                    <rect class="node" x={node.x - (recWidth / 2)} y={node.y} width={recWidth} height={recHeight} rx="5" on:dblclick={() => console.log('clicked;')}/>
                    <!-- <text class="label" x={node.x} y={node.y + 9} font-size="10px" fill="white">{node.id}</text> -->
                </g>
            {/each}
            {#each root.links() as link}
                <line x1={link.source.x} y1={link.source.y + recHeight} x2={link.target.x} y2={link.target.y} stroke="#adadad"></line>
            {/each}
        </svg>
</main>

<style>

    .label {
        text-anchor: middle;
        pointer-events: none;
    }

    .node {
        cursor: pointer;
    }
</style>
