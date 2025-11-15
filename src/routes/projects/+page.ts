import type { PageLoad } from "./$types";
import { marked } from "marked";
import { PUBLIC_API_URL } from "$env/static/public";
import { error } from "@sveltejs/kit";
import type { Project } from "./[id]/+page";

export type ProjectPreview = Project;

type Projects = {
  hasNextPage: boolean;
  hasPrevPage: boolean;
  limit: number;
  nextPage: number | null;
  pagingCounter: number;
  prevPage: number | null;
  totalDocs: number;
  totalPages: number;
  [key: number]: ProjectPreview;
};

export const load: PageLoad = async ({ fetch }) => {
  const about = await fetch(
    PUBLIC_API_URL +
      "/api/globals/project-header?draft=false&locale=undefined&trash=false",
  )
    .then((response) => response.json().then((json) => ({ json, response })))
    .then(({ json, response }) =>
      response.ok ? json : error(response.status, json.errors[0].message)
    )
    .then((json) => marked.parse(json.content));

  const projects = await fetch(
    PUBLIC_API_URL +
      "/api/projects?depth=1&draft=false&locale=undefined&trash=false",
  )
    .then((response) => response.json().then((json) => ({ json, response })))
    .then(({ json, response }) =>
      response.ok ? json : error(response.status, json.errors[0].message)
    )
    .then((json: { docs: Projects }) =>
      Object.entries(json.docs).filter(([k]) => !isNaN(Number(k))).map((
        [, v],
      ) => v as ProjectPreview)
    );

  return { about, projects };
};
