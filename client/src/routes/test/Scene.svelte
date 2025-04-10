<script>
    import {T} from '@threlte/core'
    import {interactivity} from '@threlte/extras'
    import Tile from "../../components/3D/Tile.svelte";

    interactivity()

    const spacing = 1.5;

    const tiles = [
        {"color": "green"},
        {"color": "blue"},
        {"color": "red"},
        {"color": "hotpink"},
    ]
</script>

<T.PerspectiveCamera
        makeDefault
        position={[10, 10, 10]}
        oncreate={(ref) => {
    ref.lookAt(0, 2, 0)
  }}
/>

<T.DirectionalLight
        position={[0, 10, 10]}
        castShadow
/>

{#each tiles as tile, index}
    <Tile
            color={tile.color}
            position={[
            spacing * (index % 2),
            1,
            spacing * Math.floor(index / 2)
        ]}
    />
{/each}

<T.Mesh
        rotation.x={-Math.PI / 2}
        receiveShadow
>
    <T.CircleGeometry args={[4, 40]}/>
    <T.MeshStandardMaterial color="white"/>
</T.Mesh>