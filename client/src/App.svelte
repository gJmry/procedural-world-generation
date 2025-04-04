<script lang="ts">
    import Background from "./lib/Background.svelte";

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
            console.log("World generated successfully");
            const data = await response.json();
            console.log("Generated world data:", data);
        } else {
            console.error("Error generating world");
        }
    }
</script>

<div class="main">
    <h1>Procedural world generation</h1>
    <form on:submit|preventDefault={callGeneration}>
        <label for="width">Entrer la largeur du monde</label>
        <input id="width" bind:value={width} type="number" required />

        <label for="height">Entrer la hauteur du monde</label>
        <input id="height" bind:value={height} type="number" required />

        <button type="submit">Générer</button>
    </form>
</div>

<Background />
