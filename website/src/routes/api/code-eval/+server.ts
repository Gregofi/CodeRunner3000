import type { RequestEvent } from "@sveltejs/kit";
import { error } from "@sveltejs/kit";

export async function POST({ request }: RequestEvent): Promise<Response> {
  const url = import.meta.env.VITE_CODERUNNER_BACKEND_URL as string;
  const api = import.meta.env.VITE_CODERUNNER_BACKEND_API_PATH as string;
  if (!url || !api) {
    console.log("Coderunner backend API URL is not set");
    error(400, "Internal server error");
  }
  const body = await request.text();
  console.log("Request body", body);
  try {
    const response = await fetch(`${url}${api}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body,
    });
    return response;
  } catch (e) {
    console.log("Failed to compile code");
    console.log(" - Backend URL", url);
    console.log(" - Backend API", api);
    console.log(e);
    error(500, `Internal server error`);
  }
}
