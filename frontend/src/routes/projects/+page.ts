import type { PageLoad } from "./$types";
import { marked } from "marked";
import { PUBLIC_API_URL, PUBLIC_BUCKET_URL } from "$env/static/public";
import { type paths } from "$lib/client";
import createClient from "openapi-fetch";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ fetch }) => {
  const about = await fetch(PUBLIC_BUCKET_URL + "/system/projects.md")
    .then((response) => response.text())
    .then((md) => marked.parse(md));

  const client = createClient<paths>({ baseUrl: PUBLIC_API_URL, fetch: fetch });

  const { data: projects, response } = await client.GET("/projects");

  if (!projects) {
    error(response.status, await response.text());
  }

  return { about, projects };
};
