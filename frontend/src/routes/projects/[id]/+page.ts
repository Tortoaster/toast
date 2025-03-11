import type { PageLoad } from "./$types";
import { marked } from "marked";
import { PUBLIC_API_URL, PUBLIC_BUCKET_URL } from "$env/static/public";
import type { paths } from "$lib/client";
import createClient from "openapi-fetch";

export const load: PageLoad = async ({ params, fetch }) => {
  const client = createClient<paths>({ baseUrl: PUBLIC_API_URL, fetch: fetch });

  const project = await client.GET("/projects/{id}", {
    params: { path: { id: params.id } },
  }).then(({ data }) => data!);
  const comments = client.GET("/projects/{project_id}/comments", {
    params: { path: { project_id: params.id } },
  }).then(({ data }) => data!);

  const content = await fetch(`${PUBLIC_BUCKET_URL}/content/${project.id}.md`)
    .then((response) => response.text())
    .then((md) => marked.parse(md));

  return { project, content, comments };
};
