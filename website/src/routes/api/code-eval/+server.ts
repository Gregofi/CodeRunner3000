import type { RequestEvent } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export type Language = "Lua";

export async function POST({ request }: RequestEvent): Promise<Response> {
  const url = import.meta.env.VITE_CODERUNNER_BACKEND_API as string;
  if (!url) {
    throw error(400, "Coderunner backend API URL is not set");
  }
  const body = await request.text();

  try {
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Accept": "application/json",
      },
      body,
    }); 
    return response;
  } catch (e) {
    throw error(500, `Failed to compile code: ${e}`);
  }
}
