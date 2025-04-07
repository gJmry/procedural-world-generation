<script>
    import { onMount } from 'svelte';
    import HomePage from './routes/Home.svelte';
    import WorldPage from './routes/Worldview.svelte';
    import NotFoundPage from './routes/NotFoundPage.svelte';

    let currentRoute;
    let component;

    const routes = {
        '/': HomePage,
        '/world': WorldPage,
    };

    function updateRoute() {
        currentRoute = window.location.pathname;
        component = routes[currentRoute] || NotFoundPage;
    }

    function navigate(path) {
        window.history.pushState(null, null, path);
        updateRoute();
    }

    onMount(() => {
        updateRoute();
        window.addEventListener('popstate', updateRoute);
    });

    function handleClick(event) {
        const target = event.target;
        if (target.nodeName === 'A' && target.hasAttribute('href')) {
            event.preventDefault();
            navigate(target.getAttribute('href'));
        }
    }
</script>

<svelte:window on:click={handleClick} />

<svelte:component this={component} />