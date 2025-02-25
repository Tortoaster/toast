import type { PageLoad } from "./$types";
import { marked } from "marked";
import { PUBLIC_API_URL, PUBLIC_BUCKET_URL } from "$env/static/public";
import { Configuration, DefaultApi } from "$lib/client";

export const load: PageLoad = async ({ fetch }) => {
  const about = await fetch(PUBLIC_BUCKET_URL + "/system/projects.md")
    .then((response) => response.text())
    .then((md) => marked.parse(md));

  const api = new DefaultApi(new Configuration({ basePath: PUBLIC_API_URL }));
  const projects = await api.listProjects();

  return { about, projects };
};
