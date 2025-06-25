import type { RequestEvent } from "@sveltejs/kit";
import { env } from "$env/dynamic/private";
import { dev } from "$app/environment";

export async function GET({
  params,
  request,
  getClientAddress,
}: RequestEvent): Promise<Response> {
  const backendUrl = env.CODERUNNER_BACKEND_URL;
  if (!backendUrl) {
    throw new Error("No backend URL provided");
  }

  const key = params.key;
  if (!key) {
    throw new Error("No key provided");
  }

  const url = `${backendUrl}/api/v1/link/get/${key}`;

  // To make rate limiting work properly, we must request X-Forwarded-For here unconditionally.
  const xforwardedfor = dev
    ? getClientAddress()
    : request.headers.get("x-forwarded-for");
  if (!xforwardedfor) {
    throw new Error("No x-forwarded-for header provided");
  }

  return await fetch(url, {
    method: "GET",
    headers: {
      "X-Forwarded-For": xforwardedfor,
    },
  });
}
