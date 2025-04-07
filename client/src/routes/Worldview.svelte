<script lang="ts">
    import { world } from "../stores/worldStore";

    $: currentWorld = $world;

    const unsubscribe = world.subscribe((value) => {
        console.log("Le store world a changé :", value);
    });
</script>

<div>
    <h1>Données du Monde</h1>
    {#if $world}
        <pre>{JSON.stringify($world, null, 2)}</pre>
    {:else}
        <p>Le monde n'a pas encore été généré.</p>
    {/if}
</div>