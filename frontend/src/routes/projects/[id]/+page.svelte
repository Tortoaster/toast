<script lang="ts">
    import {PUBLIC_API_URL} from '$env/static/public';
    import type {PageData} from './$types';
    import Overlay from "$lib/Overlay.svelte";
    import {format} from "date-fns";

    interface Props {
        data: PageData;
    }

    const {data}: Props = $props();

    let [day, date, year] = format(data.project.createdAt, 'PPPP').split(', ');
</script>

<svelte:head>
    <title>{data.project.name} - Tortoaster</title>
</svelte:head>

<Overlay>
    <div class="flex justify-between items-baseline">
        <h1 class="text-4xl text-white-bright font-bold">{data.project.name}</h1>
        <div class="flex">
            <span class="text-white italic hidden lg:block">{day},&nbsp;</span>
            <span class="text-white italic">{date}</span>
            <span class:hidden={year !== `${new Date().getFullYear()}`} class="text-white italic">,&nbsp;{year}</span>
        </div>
    </div>
</Overlay>

<main class="flex flex-col gap-single p-single">
    <div class="card mx-auto">
        <img alt="{data.project.thumbnail.alt}" src="{ PUBLIC_API_URL }{ data.project.thumbnail.url }">
    </div>

    {#if data.project.project_url}
        <a class="btn text-lg mx-auto" href={data.project.project_url}>Visit project page &#x2197;</a>
    {/if}

    <div class="md lg:max-w-gratio lg:mx-auto">
        {@html data.content}
    </div>
</main>
