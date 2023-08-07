<script>
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    let waiting = false;
    let timeout = null;
    let delay = 300;
    export let titleId = "";

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
    <slot />
</g>
