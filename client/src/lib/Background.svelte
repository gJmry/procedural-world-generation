<script>
    import {onMount} from "svelte";
    import Granim from "granim";

    onMount(() => {
        new Granim({
            element: '#granim-canvas',
            name: 'granim',
            opacity: [1, 1],
            states: {
                "default-state": {
                    gradients: [
                        ['#834D9B', '#D04ED6'],
                        ['#1CD8D2', '#93EDC7']
                    ]
                }
            }
        });
    });
</script>

<canvas id="granim-canvas"></canvas>