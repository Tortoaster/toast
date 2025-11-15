import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { PUBLIC_API_URL } from "$env/static/public";
import { marked } from "marked";

export type Project = {
  createdAt: string;
  id: string;
  name: string;
  thumbnail: { url: string; alt: string };
  project_url: string | undefined;
  preview: string;
  content: string;
};

export const load: PageLoad = async ({ params, fetch }) => {
  const project = await fetch(
    PUBLIC_API_URL +
      `/api/projects/${params.id}?depth=1&draft=false&locale=undefined&trash=false`,
  )
    .then((response) => response.json().then((json) => ({ json, response })))
    .then(({ json, response }) =>
      response.ok
        ? json as Project
        : error(response.status, json.errors[0].message)
    );

  const content = marked.parse(project.content);

  return { project, content };
};
