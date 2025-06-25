import type { RequestEvent } from "@sveltejs/kit";

const backendUrl = import.meta.env.VITE_CODERUNNER_BACKEND_URL as string;

export async function GET({
  params,
  request,
}: RequestEvent): Promise<Response> {
  if (!backendUrl) {
    throw new Error("No backend URL provided");
  }

  const key = params.key;
  if (!key) {
    throw new Error("No key provided");
  }

  const url = `${backendUrl}/api/v1/link/get/${key}`;

  // To make rate limiting work properly, we must request X-Forwarded-For here unconditionally.
  const xforwardedfor = request.headers.get("x-forwarded-for");
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
