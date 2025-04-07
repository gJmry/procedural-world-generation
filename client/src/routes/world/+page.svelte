<script lang="ts">
    import {world, type WorldData} from "$lib/stores/worldStore";
    import {onMount} from "svelte";

    let localWorld: WorldData | undefined;

    onMount(() => {
        console.log("WorldView monté, valeur initiale:", $world);
        return world.subscribe(value => {
            console.log("Store mis à jour dans WorldView:", value);
            localWorld = value;
        });
    });
</script>

<div>
    {#if $world}
        <div class="flex flex-col gap-1">
            {#each $world as row}
                <div class="flex flex-row gap-1">
                    {#each row as tile}
                        {#if tile.biome === "Ocean"}
                            <div class="w-8 h-8 bg-blue-500"></div>
                        {:else if tile.biome === "Beach"}
                            <div class="w-8 h-8 bg-yellow-300"></div>
                        {:else if tile.biome === "Desert"}
                            <div class="w-8 h-8 bg-yellow-500"></div>
                        {:else if tile.biome === "Plains"}
                            <div class="w-8 h-8 bg-green-500"></div>
                        {:else if tile.biome === "Forest"}
                            <div class="w-8 h-8 bg-green-700"></div>
                        {:else if tile.biome === "Mountain"}
                            <div class="w-8 h-8 bg-gray-700"></div>
                        {/if}
                    {/each}
                </div>
            {/each}
        </div>
    {:else}
        <p>Le monde n'a pas encore été généré.</p>
    {/if}
</div>