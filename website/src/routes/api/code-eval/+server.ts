import { env } from "$env/dynamic/private";
import { dev } from "$app/environment";
import type { RequestEvent } from "@sveltejs/kit";
import { error } from "@sveltejs/kit";

export async function POST({
  request,
  getClientAddress,
}: RequestEvent): Promise<Response> {
  const url = env.CODERUNNER_BACKEND_URL;
  const api = env.CODERUNNER_BACKEND_API_PATH;
  if (!url || !api) {
    console.log("Coderunner backend API URL is not set");
    error(400, "Internal server error");
  }
  const body = await request.text();
  console.log("Request body", body);

  // To make rate limiting work properly, we must request X-Forwarded-For here unconditionally.
  const xforwardedfor = dev
    ? getClientAddress()
    : request.headers.get("x-forwarded-for");
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
