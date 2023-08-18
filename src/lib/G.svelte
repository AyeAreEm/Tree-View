<script>
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    let waiting = false;
    let timeout = null;
    let delay = 300;

    export let titleId = "";
    export let xCord = 0;
    export let yCord = 0;
    export let viewbox = "";
    export let w = 0;
    export let h = 0;

    const handleClickType = () => {
        if (waiting) {
            clearTimeout(timeout);
            dispatch('dblclick');
            waiting = false;
            return;
        }
        waiting = true;
        timeout = setTimeout(() => {
            dispatch('sglclick');
            waiting = false;
        }, delay);
    }
</script>

<g on:click={handleClickType} on:keydown={handleClickType}>
    <title>{titleId}</title>
    <svg id={titleId} x={xCord} y={yCord} viewBox={viewbox} width={w} height={h} class="node" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" fill="#000000">
        <slot />
    </svg>
</g>
