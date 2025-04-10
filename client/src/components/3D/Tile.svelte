<script>
    import { T } from '@threlte/core'

    let {color = "white", position = [1, 1, 1]} = $props();
</script>

<T.Mesh
        position={position}
castShadow
>
<T.BoxGeometry args={[1, 1, 1]} />
<T.MeshStandardMaterial color={color} />
</T.Mesh>