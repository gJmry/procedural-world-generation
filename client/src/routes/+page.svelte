<script lang="ts">
    import Background from "../components/Background.svelte";
    import {setWorld, world, type WorldData} from "$lib/stores/worldStore";
    import { goto } from "\$app/navigation";

    let width: number = 0;
    let height: number = 0;

    async function callGeneration() {
        const data = {
            width,
            height
        };

        const response = await fetch('http://localhost:9090/world', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });

        if (response.ok) {
            const worldData: WorldData = await response.json();
            setWorld(worldData);
            await goto('/world');
        } else {
            console.error("Error generating world");
        }
    }
</script>

<div class="main">
    <h1>Procedural worldStore generation</h1>
    <form on:submit|preventDefault={callGeneration}>
        <label for="width">Entrer la largeur du monde</label>
        <input id="width" bind:value={width} type="number" required />

        <label for="height">Entrer la hauteur du monde</label>
        <input id="height" bind:value={height} type="number" required />

        <button type="submit">Générer</button>
    </form>
</div>

<Background />
