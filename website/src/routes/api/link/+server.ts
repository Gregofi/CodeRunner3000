import type { RequestEvent } from "@sveltejs/kit";

const TOO_MANY_REQUESTS = 429;

const backendUrl = import.meta.env.VITE_CODERUNNER_BACKEND_URL as string;

export async function POST({
  request,
  getClientAddress,
}: RequestEvent): Promise<Response> {
  if (backendUrl === undefined) {
    throw new Error("Backend URL is not set");
  }
  const url = `${backendUrl}/api/v1/link/new`;

  const body = await request.text();

  // This will most likely be behind a proxy. Still, if forwarded is not set
  // we have to send some ip address.
  const xforwardedfor =
    request.headers.get("x-forwarded-for") || getClientAddress();

  const response = await fetch(url, {
    method: "POST",
    body,
    headers: {
      "content-type": request.headers.get("content-type") ?? "text/plain",
      accept: "application/json",
      "X-Forwarded-For": xforwardedfor,
    },
  });

  if (!response.ok) {
    if (response.status === TOO_MANY_REQUESTS) {
      throw new Error("Too many requests to link generator, try again later");
    }

    throw new Error("Failed to fetch link data");
  }

  return response;
}
