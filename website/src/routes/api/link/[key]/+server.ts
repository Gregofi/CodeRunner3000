import type { RequestEvent } from "@sveltejs/kit";

const backendUrl = import.meta.env.VITE_CODERUNNER_BACKEND_URL as string;

export async function GET({
  params,
  request,
  getClientAddress,
}: RequestEvent): Promise<Response> {
  if (!backendUrl) {
    throw new Error("No backend URL provided");
  }

  const key = params.key;
  if (!key) {
    throw new Error("No key provided");
  }

  const url = `${backendUrl}/api/v1/link/get/${key}`;
  // This will most likely be behind a proxy. Still, if forwarded is not set
  // we have to send some ip address.
  const xforwardedfor =
    request.headers.get("x-forwarded-for") || getClientAddress();

  return await fetch(url, {
    method: "GET",
    headers: {
      "X-Forwarded-For": xforwardedfor,
    },
  });
}
