<script lang="ts">
    import {formatDistanceToNow} from "date-fns";
    import {PUBLIC_API_URL} from '$env/static/public';
    import type {ProjectPreview} from "../routes/projects/+page";

    interface Props {
        project: ProjectPreview;
    }

    let {project}: Props = $props();
</script>

<div class="card bg-black-bright">
    <a href="/projects/{ project.id }" class="group">
        <img class="bg-black object-cover group-hover:object-contain aspect-card w-full"
             alt="{ project.thumbnail.alt }"
             src="{ PUBLIC_API_URL }{ project.thumbnail.url }"/>
        <div class="overflow-hidden p-single aspect-card flex flex-col gap-single">
            <div class="flex justify-between items-center">
                <h3 class="text-xl font-bold">{ project.name }</h3>
                <small class="text-gray-bright text-right">{ formatDistanceToNow(project.createdAt, {addSuffix: true}) }</small>
            </div>
            <div class="description">{ project.preview }</div>
        </div>
        <div class="absolute w-full bottom-0 -translate-y-full h-double bg-linear-to-t from-black-bright from-50%"></div>
    </a>
    <span class="bite"></span>
</div>
