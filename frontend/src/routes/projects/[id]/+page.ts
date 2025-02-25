import type { PageLoad } from "./$types";
import { marked } from "marked";
import { PUBLIC_API_URL, PUBLIC_BUCKET_URL } from "$env/static/public";
import { Configuration, DefaultApi } from "$lib/client";

export const load: PageLoad = async ({ params, fetch }) => {
  const api = new DefaultApi(new Configuration({ basePath: PUBLIC_API_URL }));
  const project = await api.getProject({ id: params.id });

  const content = await fetch(`${PUBLIC_BUCKET_URL}/content/${project.id}.md`)
    .then((response) => response.text())
    .then((md) => marked.parse(md));

  const comments = api.listComments({ projectId: params.id });

  return { project, content, comments };
};
