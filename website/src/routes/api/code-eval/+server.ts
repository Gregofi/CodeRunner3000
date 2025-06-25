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

  // To make rate limiting work properly, we must request X-Forwarded-For here unconditionally.
  const xforwardedfor = request.headers.get("x-forwarded-for");
  if (!xforwardedfor) {
    console.error("No x-forwarded-for header provided");
    error(500, "No x-forwarded-for header provided");
  }

  const response = await fetch(`${url}${api}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
      "X-Forwarded-For": xforwardedfor,
    },
    body,
  });
  if (response.status === 429) {
    error(429, "Rate limit exceeded");
  }
  return response;
}
